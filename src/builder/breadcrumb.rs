use crate::domain::CanvasComponent;
use crate::state::AppState;
use leptos::prelude::*;

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
        CanvasComponent::Button(_) => "Button".to_string(),
        CanvasComponent::Text(_) => "Text".to_string(),
        CanvasComponent::Input(_) => "Input".to_string(),
        CanvasComponent::Container(_) => "Container".to_string(),
        CanvasComponent::Custom(_) => "Custom".to_string(),
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
        CanvasComponent::Custom(custom) => custom.name.clone(),
    }
}

#[component]
pub fn BreadcrumbNavigation() -> impl IntoView {
    // Get app state from context - no prop drilling!
    let app_state = AppState::expect_context();
    let canvas_state = app_state.canvas;

    let breadcrumbs = Memo::new(move |_| {
        let mut items = vec![BreadcrumbItem {
            name: "Canvas".to_string(),
            component_type: "Canvas".to_string(),
            index: None,
        }];

        if let Some(selected_id) = canvas_state.selected.get() {
            let components = canvas_state.components.get();
            if let Some((idx, component)) = components
                .iter()
                .enumerate()
                .find(|(_, c)| c.id() == &selected_id)
            {
                items.push(BreadcrumbItem {
                    name: component_name(component),
                    component_type: component_type(component),
                    index: Some(idx),
                });
            }
        }

        items
    });

    let navigate_to = move |item: BreadcrumbItem| match item.index {
        Some(idx) => {
            let components = canvas_state.components.get();
            if let Some(component) = components.get(idx) {
                canvas_state.selected.set(Some(*component.id()));
            }
        }
        None => {
            canvas_state.selected.set(None);
        }
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
                                view! { <span class="breadcrumb-separator">"/"</span> }.into_any()
                            } else {
                                view! { <span class="breadcrumb-separator" style="display:none;"></span> }.into_any()
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
                                        }.into_any()
                                    } else {
                                        view! { <span style="display:none;"></span> }.into_any()
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
