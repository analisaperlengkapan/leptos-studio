
use leptos::*;
use web_sys::wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CanvasComponent {
    Button { label: String },
    Text { content: String },
    Input { placeholder: String },
    Container { children: Vec<CanvasComponent> },
    Custom { name: String, template: String },
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
                        if let Some(rest) = data.strip_prefix("Custom::") {
                            let mut parts = rest.splitn(2, "::");
                            let name = parts.next().unwrap_or("").to_string();
                            let template = parts.next().unwrap_or("").to_string();
                            c.push(CanvasComponent::Custom { name, template });
                        } else {
                            match data.as_str() {
                                "Button" => c.push(CanvasComponent::Button { label: "Button".to_string() }),
                                "Text" => c.push(CanvasComponent::Text { content: "Text".to_string() }),
                                "Input" => c.push(CanvasComponent::Input { placeholder: "Input".to_string() }),
                                "Container" => c.push(CanvasComponent::Container { children: vec![] }),
                                _ => {}
                            }
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

    // Fungsi rekursif untuk render komponen (termasuk children Container)
    fn render_component(
        comp: CanvasComponent,
        parent_idx: Option<usize>,
        selected: RwSignal<SelectedComponent>,
        components: RwSignal<Vec<CanvasComponent>>,
    ) -> leptos::View {
        match comp {
            CanvasComponent::Button { label } => view! { <div><button>{label}</button></div> }.into_view(),
            CanvasComponent::Text { content } => view! { <div><span>{content}</span></div> }.into_view(),
            CanvasComponent::Input { placeholder } => view! { <div><input placeholder=placeholder /></div> }.into_view(),
            CanvasComponent::Custom { name, template } => view! { <div><span style="color:#7b1fa2;">Custom: {name.clone()}</span><div>{template.clone()}</div></div> }.into_view(),
            CanvasComponent::Container { children } => {
                let components = components.clone();
                let on_drag_over = move |ev: leptos::ev::DragEvent| {
                    let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
                    drag_ev.prevent_default();
                };
                let on_drop = move |ev: leptos::ev::DragEvent| {
                    let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
                    drag_ev.prevent_default();
                    if let Some(dt) = drag_ev.data_transfer() {
                        if let Ok(data) = dt.get_data("component") {
                            if !data.is_empty() {
                                if let Some(idx) = parent_idx {
                                    components.update(|c| {
                                        if let CanvasComponent::Container { children } = &mut c[idx] {
                                            if let Some(rest) = data.strip_prefix("Custom::") {
                                                let mut parts = rest.splitn(2, "::");
                                                let name = parts.next().unwrap_or("").to_string();
                                                let template = parts.next().unwrap_or("").to_string();
                                                children.push(CanvasComponent::Custom { name, template });
                                            } else {
                                                match data.as_str() {
                                                    "Button" => children.push(CanvasComponent::Button { label: "Button".to_string() }),
                                                    "Text" => children.push(CanvasComponent::Text { content: "Text".to_string() }),
                                                    "Input" => children.push(CanvasComponent::Input { placeholder: "Input".to_string() }),
                                                    "Container" => children.push(CanvasComponent::Container { children: vec![] }),
                                                    _ => {}
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }
                };
                view! {
                    <div class="container" on:drop=on_drop on:dragover=on_drag_over>
                        <div style="font-size:0.9em;color:#888;">Container</div>
                        <For
                            each=move || children.clone().into_iter().enumerate()
                            key=|(i, _)| *i
                            children=move |(_i, child)| {
                                render_component(child, None, selected, components)
                            }
                        />
                    </div>
                }.into_view()
            }
        }
    }

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
                            {render_component(comp.clone(), Some(idx), selected, components)}
                        </div>
                    }
                }
            />
        </div>
    }
}
