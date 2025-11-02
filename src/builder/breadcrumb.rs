use crate::builder::canvas::{CanvasComponent, SelectedComponent};
use leptos::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
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
            "Custom" => "⚡",
            _ => "📄",
        }
    }

    pub fn display_name(&self) -> String {
        match &self.index {
            Some(idx) => format!("{} #{}", self.component_type, idx + 1),
            None => "Canvas".to_string(),
        }
    }
}

fn component_type(component: &CanvasComponent) -> String {
    match component {
        CanvasComponent::Button { .. } => "Button".to_string(),
        CanvasComponent::Text { .. } => "Text".to_string(),
        CanvasComponent::Input { .. } => "Input".to_string(),
        CanvasComponent::Container { .. } => "Container".to_string(),
        CanvasComponent::Custom { .. } => "Custom".to_string(),
    }
}

fn component_name(component: &CanvasComponent) -> String {
    match component {
        CanvasComponent::Button { label } => label.clone(),
        CanvasComponent::Text { content } => content.clone(),
        CanvasComponent::Input { placeholder } => placeholder.clone(),
        CanvasComponent::Container { .. } => "Container".to_string(),
        CanvasComponent::Custom { name } => name.clone(),
    }
}

#[component]
pub fn BreadcrumbNavigation(
    components: RwSignal<Vec<CanvasComponent>>,
    selected: RwSignal<SelectedComponent>,
) -> impl IntoView {
    let breadcrumbs = create_memo(move |_| {
        let mut items = vec![BreadcrumbItem {
            name: "Canvas".to_string(),
            component_type: "Canvas".to_string(),
            index: None,
        }];

        if let Some(idx) = selected.get().idx {
            if let Some(component) = components.get().get(idx) {
                items.push(BreadcrumbItem {
                    name: component_name(component),
                    component_type: component_type(component),
                    index: Some(idx),
                });
            }
        }

        items
    });

    let navigate_to = move |item: BreadcrumbItem| {
        selected.set(SelectedComponent { idx: item.index });
    };

    view! {
        <nav class="breadcrumb-nav" aria-label="Component navigation">
            <For
                each=move || breadcrumbs.get().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(index, item)| {
                    let is_last = move || index == breadcrumbs.get().len() - 1;
                    let item_clone = item.clone();

                    view! {
                        <>
                            {move || if index > 0 {
                                view! { <span class="breadcrumb-separator">"/"</span> }.into_view()
                            } else {
                                view! { <></> };
                                ().into_view()
                            }}

                            <button
                                class="breadcrumb-item"
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
                                <span>{item.icon()}</span>
                                <span>{item.display_name()}</span>
                                {move || {
                                    let item_name = item.name.clone();
                                    let display_name = item.display_name();
                                    if !item_name.is_empty() && item_name != display_name {
                                        view! {
                                            <span style="
                                                font-size: 11px;
                                                opacity: 0.7;
                                                background: rgba(0,0,0,0.1);
                                                padding: 1px 4px;
                                                border-radius: 2px;
                                                margin-left: 4px;
                                                max-width: 80px;
                                                overflow: hidden;
                                                text-overflow: ellipsis;
                                                white-space: nowrap;
                                            ">{item_name}</span>
                                        }.into_view()
                                    } else {
                                        view! { <></> };
                                        ().into_view()
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
