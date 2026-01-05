pub mod renderer;
pub mod empty_state;

pub use renderer::ComponentRenderer;
pub use empty_state::CanvasEmptyState;

use leptos::prelude::*;
use web_sys::DragEvent;

use crate::builder::component_library::create_canvas_component;
use crate::builder::drag_drop::DropZone;
use crate::domain::{CanvasComponent, ComponentId};
use crate::state::{AppState, CanvasState, Snapshot};

/// Main Canvas component for the UI builder
///
/// The Canvas is where users drag and drop components to build their UI.
/// It uses AppState context for all state management, eliminating prop drilling.
#[component]
pub fn Canvas() -> impl IntoView {
    // Get app state from context - no prop drilling!
    let app_state = AppState::expect_context();
    let canvas_state = app_state.canvas;

    // Drag and drop handlers
    let drop_zone_on_drop = move |ev: leptos::ev::DragEvent| {
        handle_drop(ev, canvas_state, None);
    };

    // Clear selection when clicking on empty canvas area
    let on_canvas_click = move |_ev: leptos::ev::MouseEvent| {
        canvas_state.selected.set(None);
    };

    // Optimization: Track render time in an effect to avoid side effects during render
    // Use a Memo to capture the start time when the dependency changes (before render)
    let render_tracker = Memo::new(move |_| {
        canvas_state.components.track(); // Track changes

        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            if let Some(perf) = window.performance() {
                return Some(perf.now());
            }
        }
        None
    });

    Effect::new(move |_| {
        // Dependencies
        let start_time = render_tracker.get();

        #[cfg(target_arch = "wasm32")]
        if let Some(start) = start_time {
             if let Some(window) = web_sys::window() {
                if let Some(perf) = window.performance() {
                    let end = perf.now();
                    let duration = (end - start).max(0.0);
                    app_state.ui.render_time.set(duration);
                }
            }
        }

        app_state.ui.render_count.update(|count| {
            *count = count.saturating_add(1);
        });
    });

    // Memoize the empty check to prevent unnecessary re-evaluations
    let is_empty = Memo::new(move |_| canvas_state.components.with(|c| c.is_empty()));

    view! {
        <DropZone
            zone_name="canvas-root".to_string()
            drag_state=app_state.canvas.drag_state
            on_drop=drop_zone_on_drop
            config=None
        >
            <div
                class="canvas"
                on:click=on_canvas_click
            >
                <div class="canvas-content">
                    {move || {
                        // Force evaluation of render_tracker during render phase
                        let _ = render_tracker.get();

                        if is_empty.get() {
                            view! { <CanvasEmptyState /> }.into_any()
                        } else {
                            view! {
                                <For
                                    each=move || canvas_state.components.get()
                                    key=|comp| *comp.id()
                                    children=move |comp| {
                                        view! {
                                            <ComponentRenderer
                                                component=comp
                                                canvas_state=canvas_state
                                            />
                                        }
                                    }
                                />
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </DropZone>
    }
}

/// Handle drop event on canvas
pub fn handle_drop(ev: DragEvent, canvas_state: CanvasState, parent_id: Option<ComponentId>) {
    ev.prevent_default();
    ev.stop_propagation(); // Stop event bubbling to avoid double drops

    if let Some(data_transfer) = ev.data_transfer()
        && let Ok(component_type) = data_transfer.get_data("component")
    {
        if component_type.is_empty() {
            return;
        }

        // Create snapshot before modification
        let snapshot = Snapshot::new(
            canvas_state.components.get(),
            canvas_state.selected.get(),
            if parent_id.is_some() { "Add Child Component" } else { "Add Component" }.to_string(),
        );
        canvas_state.history.update(|h| h.push(snapshot));

        // Add new component based on type
        let new_component = create_component_from_type(&component_type);

        if let Some(component) = new_component {
            if let Some(parent) = parent_id {
                canvas_state.add_child_component_without_snapshot(&parent, component);
            } else {
                canvas_state.add_component_without_snapshot(component);
            }
        }
    }
}

/// Create component from drag data type string
fn create_component_from_type(component_type: &str) -> Option<CanvasComponent> {
    create_canvas_component(component_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_component_button() {
        let comp = create_component_from_type("Button");
        assert!(comp.is_some());
        if let Some(CanvasComponent::Button(btn)) = comp {
            assert_eq!(btn.label, "Button");
        } else {
            panic!("Expected Button component");
        }
    }

    #[test]
    fn test_create_component_text() {
        let comp = create_component_from_type("Text");
        assert!(comp.is_some());
        if let Some(CanvasComponent::Text(txt)) = comp {
            assert_eq!(txt.content, "Text");
        } else {
            panic!("Expected Text component");
        }
    }

    #[test]
    fn test_create_component_custom() {
        let comp = create_component_from_type("Custom::MyComponent");
        assert!(comp.is_some());
        if let Some(CanvasComponent::Custom(custom)) = comp {
            assert_eq!(custom.name, "MyComponent");
        } else {
            panic!("Expected Custom component");
        }
    }

    #[test]
    fn test_create_component_invalid() {
        let comp = create_component_from_type("InvalidType");
        assert!(comp.is_none());
    }
}
