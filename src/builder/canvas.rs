use super::component_library::{LibraryComponent, ResponsiveMode, Theme};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CanvasComponent {
    Button { label: String },
    Text { content: String },
    Input { placeholder: String },
    Container { children: Vec<CanvasComponent> },
    Custom { name: String },
}

#[derive(Clone, Debug, Default)]
pub struct SelectedComponent {
    pub idx: Option<usize>,
}

#[component]
pub fn canvas(
    selected: RwSignal<SelectedComponent>,
    components: RwSignal<Vec<CanvasComponent>>,
    undo_stack: RwSignal<Vec<Vec<CanvasComponent>>>,
    redo_stack: RwSignal<Vec<Vec<CanvasComponent>>>,
    theme: RwSignal<Theme>,
    responsive_mode: RwSignal<ResponsiveMode>,
    custom_theme_color: RwSignal<String>,
    custom_components: RwSignal<Vec<LibraryComponent>>,
) -> impl IntoView {
    let on_drop = move |ev: leptos::ev::DragEvent| {
        let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
        drag_ev.prevent_default();
        if let Some(dt) = drag_ev.data_transfer() {
            if let Ok(data) = dt.get_data("component") {
                if !data.is_empty() {
                    // Push current state to undo stack
                    undo_stack.update(|stack| stack.push(components.get()));
                    redo_stack.set(Vec::new());
                    components.update(|c| {
                        if let Some(rest) = data.strip_prefix("Custom::") {
                            let mut parts = rest.splitn(2, "::");
                            let name = parts.next().unwrap_or("").to_string();
                            c.push(CanvasComponent::Custom { name });
                        } else {
                            match data.as_str() {
                                "Button" => c.push(CanvasComponent::Button {
                                    label: "Button".to_string(),
                                }),
                                "Text" => c.push(CanvasComponent::Text {
                                    content: "Text".to_string(),
                                }),
                                "Input" => c.push(CanvasComponent::Input {
                                    placeholder: "Input".to_string(),
                                }),
                                "Container" => {
                                    c.push(CanvasComponent::Container { children: vec![] })
                                }
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
        _selected: RwSignal<SelectedComponent>,
        components: RwSignal<Vec<CanvasComponent>>,
        undo_stack: RwSignal<Vec<Vec<CanvasComponent>>>,
        redo_stack: RwSignal<Vec<Vec<CanvasComponent>>>,
        custom_components: RwSignal<Vec<LibraryComponent>>,
    ) -> AnyView {
        match comp {
            CanvasComponent::Button { label } => {
                view! { <div><button>{label}</button></div> }.into_any()
            }
            CanvasComponent::Text { content } => {
                view! { <div><span>{content}</span></div> }.into_any()
            }
            CanvasComponent::Input { placeholder } => {
                view! { <div><input placeholder=placeholder /></div> }.into_any()
            }
            CanvasComponent::Custom { name } => {
                let template = custom_components
                    .get()
                    .iter()
                    .find(|c| c.name == *name)
                    .and_then(|c| c.template.clone())
                    .unwrap_or("<i>Template not found</i>".to_string());
                view! { <div><span style="color:#7b1fa2;">Custom: {name.clone()}</span><div>{template}</div></div> }.into_any()
            }
            CanvasComponent::Container { children } => {
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
                                    // Push current state to undo stack
                                    undo_stack.update(|stack| stack.push(components.get()));
                                    redo_stack.set(Vec::new());
                                    components.update(|c| {
                                        if let CanvasComponent::Container { children } = &mut c[idx]
                                        {
                                            if let Some(rest) = data.strip_prefix("Custom::") {
                                                let mut parts = rest.splitn(2, "::");
                                                let name = parts.next().unwrap_or("").to_string();
                                                children.push(CanvasComponent::Custom { name });
                                            } else {
                                                match data.as_str() {
                                                    "Button" => {
                                                        children.push(CanvasComponent::Button {
                                                            label: "Button".to_string(),
                                                        })
                                                    }
                                                    "Text" => {
                                                        children.push(CanvasComponent::Text {
                                                            content: "Text".to_string(),
                                                        })
                                                    }
                                                    "Input" => {
                                                        children.push(CanvasComponent::Input {
                                                            placeholder: "Input".to_string(),
                                                        })
                                                    }
                                                    "Container" => {
                                                        children.push(CanvasComponent::Container {
                                                            children: vec![],
                                                        })
                                                    }
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
                                render_component(child, None, _selected, components, undo_stack, redo_stack, custom_components)
                            }
                        />
                    </div>
                }.into_any()
            }
        }
    }

    let custom_color = custom_theme_color.get();
    let (bg, fg) = match theme.get() {
        Theme::Light => ("#fff", "#222"),
        Theme::Dark => ("#222", "#eee"),
        Theme::Custom => (custom_color.as_str(), "#1a237e"),
    };
    // Responsive style
    let (width, height, scale, label) = match responsive_mode.get() {
        ResponsiveMode::Desktop => ("100%", "700px", 1.0, "Desktop (100% x 700px)"),
        ResponsiveMode::Tablet => ("768px", "1024px", 0.85, "Tablet (768px x 1024px)"),
        ResponsiveMode::Mobile => ("375px", "667px", 0.7, "Mobile (375px x 667px)"),
    };
    view! {
        <div class="canvas" on:drop=on_drop on:dragover=on_drag_over
            style={format!("min-height:{}; background:{}; color:{}; border: 1px solid #aaa; width:{}; height:{}; transform:scale({}); transform-origin:top left; position:relative;", height, bg, fg, width, height, scale)}>
            <div style="position:absolute;top:8px;right:12px;padding:2px 10px;background:#333;color:#fff;border-radius:6px;font-size:0.95em;opacity:0.85;z-index:10;">
                {label}
            </div>
            <p>Canvas (drag komponen ke sini)</p>
            <For
                each=move || components.get().into_iter().enumerate()
                key=|(idx, _)| *idx
                children=move |(idx, comp)| {
                    let selected_signal = selected;
                    let is_selected = move || selected_signal.get().idx == Some(idx);
                    let onclick = move |_| selected_signal.set(SelectedComponent { idx: Some(idx) });
                    let custom_components = custom_components;
                    view! {
                        <div class:canvas-selected=is_selected on:click=onclick>
                            {render_component(comp.clone(), Some(idx), selected, components, undo_stack, redo_stack, custom_components)}
                        </div>
                    }
                }
            />
        </div>
    }
}
