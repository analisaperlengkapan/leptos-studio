use crate::builder::breadcrumb::BreadcrumbNavigation;
use crate::builder::canvas::renderer::ComponentRenderer;
use crate::builder::component_library::create_canvas_component;
use crate::builder::context_menu::ContextMenu;
use crate::domain::ComponentId;
use crate::state::app_state::AppState;
use leptos::{ev, html, prelude::*};
use wasm_bindgen::JsCast;

pub mod renderer;

pub fn handle_drag_over(ev: ev::DragEvent) {
    ev.prevent_default();
}

pub fn handle_drop(ev: ev::DragEvent, _target_id: Option<ComponentId>, app_state: AppState) {
    ev.prevent_default();
    ev.stop_propagation();

    let drag_ev = ev.unchecked_into::<web_sys::DragEvent>();
    if let Some(dt) = drag_ev.data_transfer()
        && let Ok(component_type_str) = dt.get_data("component")
        // Use factory method from component_library
        && let Some(new_component) = create_canvas_component(&component_type_str)
    {
        if let Some(target) = _target_id {
            app_state.canvas.add_child_component(&target, new_component);
        } else {
            app_state.canvas.add_component(new_component);
        }
    }

    app_state
        .canvas
        .drag_state
        .set(crate::builder::drag_drop::DragState::NotDragging);
}

#[component]
pub fn Canvas() -> impl IntoView {
    let app_state = AppState::expect_context();

    // Track canvas element for dimension measurements
    let canvas_ref = NodeRef::<html::Div>::new();

    // Context Menu State
    let (cm_visible, set_cm_visible) = signal(false);
    let (cm_position, set_cm_position) = signal((0.0, 0.0));
    let (cm_target_id, set_cm_target_id) = signal(Option::<ComponentId>::None);

    // Save Custom Component Logic
    let save_custom_component = move |id: ComponentId| {
        if let Some(comp) = app_state.canvas.get_component(&id) {
            // Convert CanvasComponent to LibraryComponent
            // This is a simplified conversion. Realistically we need a name prompt.
            // For now, we'll use a prompt via window.prompt (not ideal UX but functional for MVP)
            if let Some(window) = web_sys::window() {
                if let Ok(Some(name)) = window.prompt_with_message("Enter name for custom component:") {
                    if !name.is_empty() {
                         let lib_comp = crate::builder::component_library::LibraryComponent {
                            name: name.clone(),
                            kind: comp.component_type().to_string(),
                            category: "Custom".to_string(),
                            description: Some("User saved component".to_string()),
                            template: Some(serde_json::to_string_pretty(&comp).unwrap_or_default()),
                            props_schema: None, // Simplified
                        };

                        app_state.ui.custom_components.update(|c| c.push(lib_comp));
                        app_state.ui.notify(crate::state::app_state::Notification::success(
                            format!("Saved '{}' to custom components", name)
                        ));
                    }
                }
            }
        }
    };

    // Handle background click to deselect
    let on_canvas_click = move |ev: ev::MouseEvent| {
        // Only deselect if clicking the canvas background directly
        let target = event_target::<web_sys::HtmlElement>(&ev);
        if target.id() == "main-canvas" {
            app_state.canvas.selected.set(None);
        }
    };

    // Handle context menu
    let on_context_menu = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        let target = event_target::<web_sys::Element>(&ev);

        // Find closest component ID
        if let Some(closest) = target.closest("[data-component-id]").ok().flatten()
            && let Some(id_str) = closest.get_attribute("data-component-id")
        {
            let components = app_state.canvas.components.get_untracked();
            let found_id = find_component_id_by_string(&components, &id_str);

            if let Some(id) = found_id {
                set_cm_target_id.set(Some(id));
                set_cm_position.set((ev.client_x() as f64, ev.client_y() as f64));
                set_cm_visible.set(true);

                // Also select it
                app_state.canvas.selected.set(Some(id));
                return;
            }
        }

        // If background
        set_cm_visible.set(false);
    };

    // Calculate width based on responsive mode
    let canvas_width = move || {
        use crate::state::app_state::ResponsiveMode;
        match app_state.ui.responsive_mode.get() {
            ResponsiveMode::Desktop => "100%".to_string(),
            ResponsiveMode::Tablet => "768px".to_string(),
            ResponsiveMode::TabletLandscape => "1024px".to_string(),
            ResponsiveMode::Mobile => "375px".to_string(),
            ResponsiveMode::MobileLandscape => "667px".to_string(),
        }
    };

    view! {
        <div
            class="flex-1 bg-gray-100 relative overflow-hidden flex flex-col"
            on:contextmenu=on_context_menu
        >
            <div
                class="flex-1 relative overflow-auto flex flex-col items-center justify-center p-8 canvas-area"
                on:click=on_canvas_click
                on:dragover=handle_drag_over
                on:drop=move |ev| handle_drop(ev, None, app_state)
            >
                <div
                    id="main-canvas"
                    node_ref=canvas_ref
                    class="bg-white shadow-lg min-h-[600px] w-full max-w-[1024px] relative transition-all duration-300"
                    style:width=canvas_width
                >
                    {move || {
                        let components = app_state.canvas.components.get();

                        if components.is_empty() {
                            view! {
                                <div class="canvas-empty-state">
                                    <div class="empty-state-content">
                                        <h3>"Start from scratch"</h3>
                                        <p>"Drag components from the left sidebar or add a container to get started."</p>
                                        <button
                                            class="btn btn-primary mt-4"
                                            on:click=move |_| {
                                                if let Some(comp) = create_canvas_component("Container") {
                                                    app_state.canvas.add_component(comp);
                                                }
                                            }
                                        >
                                            "Add Container"
                                        </button>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <For
                                    each=move || components.clone()
                                    key=|comp| *comp.id()
                                    children=move |comp| {
                                        view! {
                                            <ComponentRenderer
                                                component=comp
                                                canvas_state=app_state.canvas
                                            />
                                        }
                                    }
                                />
                            }.into_any()
                        }
                    }}
                </div>
            </div>

            // Breadcrumbs at the bottom
            <BreadcrumbNavigation />

            // Context Menu
            <ContextMenu
                visible=cm_visible
                position=cm_position
                component_id=cm_target_id
                on_close=Callback::new(move |_| set_cm_visible.set(false))
                on_delete=Callback::new(move |id| {
                    app_state.canvas.remove_component(&id);
                })
                on_duplicate=Callback::new(move |id| {
                    // Use the new duplicate_with_new_id method
                    if let Some(comp) = app_state.canvas.get_component(&id) {
                         let new_comp = comp.duplicate_with_new_id();
                         app_state.canvas.add_component(new_comp);
                    }
                })
                on_select_parent=Callback::new(move |id| {
                     if let Some(parent_id) = find_parent_id(&app_state.canvas.components.get_untracked(), id) {
                         app_state.canvas.selected.set(Some(parent_id));
                     }
                })
                on_save_custom=Callback::new(move |id| {
                     save_custom_component(id);
                })
            />
        </div>
    }
}

// Helper to find ID from string
fn find_component_id_by_string(
    components: &[crate::domain::CanvasComponent],
    id_str: &str,
) -> Option<ComponentId> {
    for comp in components {
        if comp.id().to_string() == id_str {
            return Some(*comp.id());
        }

        match comp {
            crate::domain::CanvasComponent::Container(c) => {
                if let Some(found) = find_component_id_by_string(&c.children, id_str) {
                    return Some(found);
                }
            }
            crate::domain::CanvasComponent::Card(c) => {
                if let Some(found) = find_component_id_by_string(&c.children, id_str) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

// Helper to find parent ID
fn find_parent_id(
    components: &[crate::domain::CanvasComponent],
    target_id: ComponentId,
) -> Option<ComponentId> {
    for comp in components {
        let is_parent = match comp {
            crate::domain::CanvasComponent::Container(c) => {
                c.children.iter().any(|child| *child.id() == target_id)
            }
            crate::domain::CanvasComponent::Card(c) => {
                c.children.iter().any(|child| *child.id() == target_id)
            }
            _ => false,
        };

        if is_parent {
            return Some(*comp.id());
        }

        // Recurse
        match comp {
            crate::domain::CanvasComponent::Container(c) => {
                if let Some(found) = find_parent_id(&c.children, target_id) {
                    return Some(found);
                }
            }
            crate::domain::CanvasComponent::Card(c) => {
                if let Some(found) = find_parent_id(&c.children, target_id) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}
