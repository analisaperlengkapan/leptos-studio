use crate::builder::component_library::create_canvas_component;
use crate::domain::{CanvasComponent, ComponentId};
use crate::state::AppState;
use leptos::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

#[component]
pub fn TreeView() -> impl IntoView {
    let app_state = AppState::expect_context();
    let components = app_state.canvas.components;

    // Create a reactive map for O(1) component lookup
    let component_map = Memo::new(move |_| {
        let mut map = HashMap::new();
        fn traverse(comps: &[CanvasComponent], map: &mut HashMap<ComponentId, CanvasComponent>) {
            for c in comps {
                map.insert(*c.id(), c.clone());
                match c {
                    CanvasComponent::Container(cont) => traverse(&cont.children, map),
                    CanvasComponent::Card(card) => traverse(&card.children, map),
                    _ => {}
                }
            }
        }
        traverse(&components.get(), &mut map);
        map
    });

    view! {
        <div class="tree-view" role="tree">
             {move || {
                if components.get().is_empty() {
                    view! {
                        <div class="tree-view-empty" role="status">
                            "No components on canvas"
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="tree-view-list">
                            <For
                                each=move || components.get()
                                key=|c| *c.id()
                                children=move |component| {
                                    view! {
                                        <TreeNode
                                            id=*component.id()
                                            component_map=component_map
                                            level=0
                                        />
                                    }
                                }
                            />
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn TreeNode(
    id: ComponentId,
    component_map: Memo<HashMap<ComponentId, CanvasComponent>>,
    level: usize,
) -> impl IntoView {
    let app_state = AppState::expect_context();

    // Reactively look up the component. If not found (deleted), return None (view! {} handles it)
    let component = Memo::new(move |_| component_map.with(|m| m.get(&id).cloned()));

    let selected_id = app_state.canvas.selected;
    let is_selected = move || selected_id.get() == Some(id);

    view! {
        {move || {
            if let Some(comp) = component.get() {
                 let label = match &comp {
                    CanvasComponent::Button(c) => format!("Button: {}", c.label),
                    CanvasComponent::Text(c) => format!("Text: {:.20}", c.content),
                    CanvasComponent::Input(c) => format!("Input ({:?})", c.input_type),
                    CanvasComponent::Container(_) => "Container".to_string(),
                    CanvasComponent::Image(_) => "Image".to_string(),
                    CanvasComponent::Card(_) => "Card".to_string(),
                    CanvasComponent::Select(_) => "Select".to_string(),
                    CanvasComponent::Custom(c) => format!("Custom: {}", c.name),
                };

                let icon = match &comp {
                    CanvasComponent::Button(_) => "🔘",
                    CanvasComponent::Text(_) => "📝",
                    CanvasComponent::Input(_) => "📥",
                    CanvasComponent::Container(_) => "📦",
                    CanvasComponent::Image(_) => "🖼️",
                    CanvasComponent::Card(_) => "🃏",
                    CanvasComponent::Select(_) => "🔽",
                    CanvasComponent::Custom(_) => "⚙️",
                };

                let on_click = move |ev: leptos::ev::MouseEvent| {
                    ev.stop_propagation();
                    app_state.canvas.selected.set(Some(id));
                };

                let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
                     if ev.key() == "Enter" || ev.key() == " " {
                        ev.prevent_default();
                        ev.stop_propagation();
                        app_state.canvas.selected.set(Some(id));
                    }
                };

                let on_drag_start = move |ev: leptos::ev::DragEvent| {
                    if let Some(dt) = ev.data_transfer() {
                        let _ = dt.set_data("move-component", &id.to_string());
                        dt.set_effect_allowed("move");
                    }
                    ev.stop_propagation();
                };

                let on_drag_over = move |ev: leptos::ev::DragEvent| {
                    ev.prevent_default(); // Allow drop
                    ev.stop_propagation();
                    if let Some(dt) = ev.data_transfer() {
                        dt.set_drop_effect("move");
                    }
                };

                let on_drop = move |ev: leptos::ev::DragEvent| {
                    // Only stop propagation if we actually handle the drop.
                    // However, we are in a tree node. If we don't handle it, should it bubble?
                    // Bubbling would mean dropping "on" the node might trigger the parent container's drop handler?
                    // In a tree view, usually dropping on a node targets that node (e.g. as parent or sibling).

                    let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
                    let mut handled = false;

                    if let Some(dt) = drag_ev.data_transfer() {
                        // Case 1: Reordering (move-component)
                        if let Ok(dragged_id_str) = dt.get_data("move-component")
                            && !dragged_id_str.is_empty()
                        {
                            let map = component_map.get();
                            let dragged_id = map.keys().find(|k| k.to_string() == dragged_id_str).cloned();

                            if let Some(did) = dragged_id {
                                app_state.canvas.move_component_relative(did, id);
                                handled = true;
                            }
                        }
                        // Case 2: Adding New Component (component)
                        else {
                            #[allow(clippy::collapsible_if)]
                            if let Ok(component_type_str) = dt.get_data("component")
                                && !component_type_str.is_empty()
                            {
                                if let Some(new_component) = create_canvas_component(&component_type_str) {
                                    // Add as child if container/card, otherwise add after (sibling)?
                                    // For consistency with move_component_relative, we might want "insert after" logic
                                    // unless we explicitly drop "into".
                                    // But `add_child_component` tries to add as child.

                                    // Let's try to add as child first (if container)
                                    let added = app_state.canvas.add_child_component(&id, new_component.clone());
                                    if !added {
                                        // Fallback: Add to parent of this node?
                                        // Or for now, just let it fail if not a container.
                                        // Or bubble up?
                                    } else {
                                        handled = true;
                                    }
                                }
                            }
                        }
                    }

                    if handled {
                        ev.prevent_default();
                        ev.stop_propagation();
                    }
                    // If not handled, we don't stop propagation, so it might bubble up to a parent container
                    // or the root canvas drop handler.
                };

                let children = match &comp {
                    CanvasComponent::Container(c) => c.children.clone(),
                    CanvasComponent::Card(c) => c.children.clone(),
                    _ => Vec::new(),
                };

                view! {
                    <div class="tree-node-wrapper" role="presentation">
                        <div
                            class=move || if is_selected() { "tree-node selected" } else { "tree-node" }
                            style=format!("padding-left: {}px", level * 12 + 12)
                            on:click=on_click
                            on:keydown=on_keydown
                            draggable="true"
                            on:dragstart=on_drag_start
                            on:dragover=on_drag_over
                            on:drop=on_drop
                            role="treeitem"
                            aria-selected=move || is_selected().to_string()
                            tabindex="0"
                        >
                            <span class="tree-node-icon" aria-hidden="true">{icon}</span>
                            <span class="tree-node-label">{label}</span>
                        </div>
                        {if !children.is_empty() {
                            Some(view! {
                                <div class="tree-node-children" role="group">
                                    <For
                                        each=move || children.clone()
                                        key=|child| *child.id()
                                        children=move |child| {
                                            view! {
                                                <TreeNode
                                                    id=*child.id()
                                                    component_map=component_map
                                                    level=level + 1
                                                />
                                            }
                                        }
                                    />
                                </div>
                            })
                        } else {
                            None
                        }}
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}
