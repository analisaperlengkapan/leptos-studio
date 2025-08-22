
use leptos::*;
use web_sys::wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CanvasComponent {
    Button { label: String },
    Text { content: String },
    Input { placeholder: String },
    Container,
}

#[component]
#[derive(Clone, Debug, Default)]
pub struct SelectedComponent {
    pub idx: Option<usize>,
}

#[component]
pub fn Canvas(selected: RwSignal<SelectedComponent>, components: RwSignal<Vec<CanvasComponent>>) -> impl IntoView {
    let on_drop = move |ev: leptos::ev::DragEvent| {
        let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
        drag_ev.prevent_default();
        if let Some(dt) = drag_ev.data_transfer() {
            if let Ok(data) = dt.get_data("component") {
                if !data.is_empty() {
                    components.update(|c| {
                        match data.as_str() {
                            "Button" => c.push(CanvasComponent::Button { label: "Button".to_string() }),
                            "Text" => c.push(CanvasComponent::Text { content: "Text".to_string() }),
                            "Input" => c.push(CanvasComponent::Input { placeholder: "Input".to_string() }),
                            "Container" => c.push(CanvasComponent::Container),
                            _ => {}
                        }
                    });
                }
            }
        }
    };

    let on_drag_over = move |ev: leptos::ev::DragEvent| {
        let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
        drag_ev.prevent_default();
    };

    view! {
        <div class="canvas" on:drop=on_drop on:dragover=on_drag_over>
            <p>Canvas (drag komponen ke sini)</p>
            <For
                each=move || components.get().into_iter().enumerate()
                key=|(idx, _)| *idx
                children=move |(idx, comp)| {
                    let selected_signal = selected;
                    let is_selected = move || selected_signal.get().idx == Some(idx);
                    let onclick = move |_| selected_signal.set(SelectedComponent { idx: Some(idx) });
                    view! {
                        <div class:canvas-selected=is_selected on:click=onclick>
                            <div>
                                {match comp {
                                    CanvasComponent::Button { label } => view! { <div><button>{label.clone()}</button></div> },
                                    CanvasComponent::Text { content } => view! { <div><span>{content.clone()}</span></div> },
                                    CanvasComponent::Input { placeholder } => view! { <div><input placeholder=placeholder.clone() /></div> },
                                    CanvasComponent::Container => view! { <div class="container">Container</div> },
                                }}
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}
