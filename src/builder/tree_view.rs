use crate::domain::{CanvasComponent, ComponentId};
use crate::state::AppState;
use leptos::prelude::*;
use std::collections::HashMap;

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
                    CanvasComponent::Custom(c) => format!("Custom: {}", c.name),
                };

                let icon = match &comp {
                    CanvasComponent::Button(_) => "🔘",
                    CanvasComponent::Text(_) => "📝",
                    CanvasComponent::Input(_) => "📥",
                    CanvasComponent::Container(_) => "📦",
                    CanvasComponent::Image(_) => "🖼️",
                    CanvasComponent::Card(_) => "🃏",
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
