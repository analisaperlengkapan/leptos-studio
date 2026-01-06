use crate::domain::{CanvasComponent, ComponentId, ComponentType};
use crate::state::AppState;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub id: Option<ComponentId>,
    pub name: String,
    pub component_type: String,
    pub index: Option<usize>,
}

impl BreadcrumbItem {
    pub fn icon(&self) -> &'static str {
        match self.component_type.as_str() {
            "Button" => "🔘",
            "Text" => "📝",
            "Input" => "📝",
            "Container" => "📦",
            "Image" => "🖼️",
            "Card" => "🃏",
            "Select" => "🔽",
            "Custom" => "⚡",
            _ => "📄",
        }
    }

    pub fn display_name(&self) -> String {
        self.component_type.clone()
    }
}

fn component_type_str(component: &CanvasComponent) -> String {
    match component.component_type() {
        ComponentType::Button => "Button".to_string(),
        ComponentType::Text => "Text".to_string(),
        ComponentType::Input => "Input".to_string(),
        ComponentType::Container => "Container".to_string(),
        ComponentType::Image => "Image".to_string(),
        ComponentType::Card => "Card".to_string(),
        ComponentType::Select => "Select".to_string(),
        ComponentType::Custom => "Custom".to_string(),
    }
}

fn component_name(component: &CanvasComponent) -> String {
    match component {
        CanvasComponent::Button(btn) => btn.label.clone(),
        CanvasComponent::Text(txt) => txt.content.clone(),
        CanvasComponent::Input(inp) => {
            if inp.placeholder.is_empty() {
                "Input".to_string()
            } else {
                inp.placeholder.clone()
            }
        }
        CanvasComponent::Container(_) => "Container".to_string(),
        CanvasComponent::Image(img) => img.alt.clone(),
        CanvasComponent::Card(_) => "Card".to_string(),
        CanvasComponent::Select(_) => "Select".to_string(),
        CanvasComponent::Custom(custom) => custom.name.clone(),
    }
}

// Recursive search for path
fn find_path(
    components: &[CanvasComponent],
    target_id: ComponentId,
    current_path: &mut Vec<BreadcrumbItem>,
) -> bool {
    for (i, comp) in components.iter().enumerate() {
        if *comp.id() == target_id {
            current_path.push(BreadcrumbItem {
                id: Some(*comp.id()),
                name: component_name(comp),
                component_type: component_type_str(comp),
                index: Some(i),
            });
            return true;
        }

        // Search children
        match comp {
            CanvasComponent::Container(c) => {
                current_path.push(BreadcrumbItem {
                    id: Some(*comp.id()),
                    name: component_name(comp),
                    component_type: component_type_str(comp),
                    index: Some(i),
                });
                if find_path(&c.children, target_id, current_path) {
                    return true;
                }
                current_path.pop();
            }
            CanvasComponent::Card(c) => {
                current_path.push(BreadcrumbItem {
                    id: Some(*comp.id()),
                    name: component_name(comp),
                    component_type: component_type_str(comp),
                    index: Some(i),
                });
                if find_path(&c.children, target_id, current_path) {
                    return true;
                }
                current_path.pop();
            }
            _ => {}
        }
    }
    false
}

#[component]
pub fn BreadcrumbNavigation() -> impl IntoView {
    let app_state = AppState::expect_context();

    let breadcrumbs = Memo::new(move |_| {
        let mut items = vec![BreadcrumbItem {
            id: None,
            name: "Canvas".to_string(),
            component_type: "Root".to_string(),
            index: None,
        }];

        if let Some(selected_id) = app_state.canvas.selected.get() {
            let components = app_state.canvas.components.get();
            let mut path = Vec::new();
            if find_path(&components, selected_id, &mut path) {
                items.extend(path);
            }
        }

        items
    });

    let navigate_to = move |item: BreadcrumbItem| {
        app_state.canvas.selected.set(item.id);
    };

    view! {
        <nav class="breadcrumb-nav flex items-center gap-1 overflow-x-auto whitespace-nowrap py-1 px-2 border-t border-gray-200 bg-white" aria-label="Component navigation">
            <For
                each=move || breadcrumbs.get().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(index, item)| {
                    let is_last = move || index == breadcrumbs.get().len() - 1;
                    let item_clone = item.clone();

                    view! {
                        <>
                            {move || if index > 0 {
                                view! { <span class="text-gray-400 text-xs px-1">"/"</span> }.into_any()
                            } else {
                                view! { <span class="hidden"></span> }.into_any()
                            }}

                            <button
                                class="breadcrumb-item flex items-center gap-1 px-1.5 py-0.5 rounded hover:bg-gray-100 transition-colors disabled:opacity-100 disabled:font-semibold disabled:text-blue-600"
                                disabled=is_last()
                                on:click={
                                    let item = item_clone.clone();
                                    move |_| {
                                        if !is_last() {
                                            navigate_to(item.clone());
                                        }
                                    }
                                }
                            >
                                <span class="text-xs">{item.icon()}</span>
                                <span class="text-xs">{item.display_name()}</span>
                                {move || {
                                    let item_name = item.name.clone();
                                    let display_name = item.display_name();
                                    if !item_name.is_empty() && item_name != display_name {
                                        view! {
                                            <span class="text-[10px] text-gray-500 bg-gray-50 px-1 rounded ml-1 max-w-[80px] truncate">
                                                {item_name}
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! { <span class="hidden"></span> }.into_any()
                                    }
                                }}
                            </button>
                        </>
                    }
                }
            />
        </nav>
    }
}
