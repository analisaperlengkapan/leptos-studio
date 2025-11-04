pub mod renderer;

pub use renderer::ComponentRenderer;

use leptos::prelude::*;
use web_sys::DragEvent;

use crate::state::{AppState, CanvasState, Snapshot};
use crate::domain::{
    CanvasComponent, ButtonComponent, TextComponent, InputComponent,
    ContainerComponent, CustomComponent
};

/// Main Canvas component for the UI builder
/// 
/// The Canvas is where users drag and drop components to build their UI.
/// It uses AppState context for all state management, eliminating prop drilling.
#[component]
pub fn Canvas() -> impl IntoView {
    // Get app state from context - no prop drilling!
    let app_state = AppState::use_context();
    let canvas_state = app_state.canvas;
    
    // Drag and drop handlers
    let on_drop = move |ev: leptos::ev::DragEvent| {
        handle_drop(ev.clone(), canvas_state);
    };
    
    let on_drag_over = move |ev: leptos::ev::DragEvent| {
        handle_drag_over(ev.clone());
    };
    
    // Clear selection when clicking on empty canvas area
    let on_canvas_click = move |_ev: leptos::ev::MouseEvent| {
        canvas_state.selected.set(None);
    };
    
    view! {
        <div 
            class="canvas"
            on:drop=on_drop
            on:dragover=on_drag_over
            on:click=on_canvas_click
        >
            <div class="canvas-content">
                {move || {
                    let components = canvas_state.components.get();
                    if components.is_empty() {
                        view! {
                            <div class="canvas-empty-state">
                                <p>"Drag components here to start building"</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <For
                                each=move || canvas_state.components.get()
                                key=|comp| comp.id().clone()
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
    }
}

/// Handle drop event on canvas
fn handle_drop(ev: DragEvent, canvas_state: CanvasState) {
    ev.prevent_default();
    
    if let Some(data_transfer) = ev.data_transfer()
        && let Ok(component_type) = data_transfer.get_data("component") {
            if component_type.is_empty() {
                return;
            }
            
            // Create snapshot before modification
            let snapshot = Snapshot::new(
                canvas_state.components.get(),
                canvas_state.selected.get()
            );
            canvas_state.history.update(|h| h.push(snapshot));
            
            // Add new component based on type
            let new_component = create_component_from_type(&component_type);
            
            if let Some(component) = new_component {
                canvas_state.add_component(component);
            }
        }
}

/// Handle drag over event (required to allow drop)
fn handle_drag_over(ev: DragEvent) {
    ev.prevent_default();
}

/// Create component from drag data type string
fn create_component_from_type(component_type: &str) -> Option<CanvasComponent> {
    match component_type {
        "Button" => {
            let button = ButtonComponent::new("Button".to_string());
            Some(CanvasComponent::Button(button))
        }
        "Text" => {
            let text = TextComponent::new("Text".to_string());
            Some(CanvasComponent::Text(text))
        }
        "Input" => {
            let input = InputComponent::new();
            Some(CanvasComponent::Input(input))
        }
        "Container" => {
            let container = ContainerComponent::new();
            Some(CanvasComponent::Container(container))
        }
        data if data.starts_with("Custom::") => {
            // Custom component format: "Custom::ComponentName"
            let name = data.strip_prefix("Custom::").unwrap_or("Custom");
            let custom = CustomComponent::new(
                name.to_string(),
                "<div>Custom Component</div>".to_string()
            );
            Some(CanvasComponent::Custom(custom))
        }
        _ => None,
    }
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
