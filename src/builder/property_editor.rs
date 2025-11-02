use super::canvas::{CanvasComponent, SelectedComponent};
use super::component_library::LibraryComponent;
use leptos::*;

#[component]
pub fn PropertyEditor(
    selected: RwSignal<SelectedComponent>,
    components: RwSignal<Vec<CanvasComponent>>,
    component_library: RwSignal<Vec<LibraryComponent>>,
    notification: RwSignal<Option<String>>,
    custom_components: RwSignal<Vec<LibraryComponent>>, // or whatever the type is
) -> impl IntoView {
    let idx = move || selected.get().idx;
    let comp = move || idx().and_then(|i| components.get().get(i).cloned());

    view! {
        <section class="property-editor">
            <h3>Property Editor</h3>
            {move || match comp() {
                Some(CanvasComponent::Button { label }) => {
                    let schema = component_library.get().iter().find(|c| c.kind == "Button").and_then(|c| c.props_schema.clone());
                    let mut errors = vec![];
                    if let Some(props) = &schema {
                        for prop in props {
                            if prop.required && label.trim().is_empty() {
                                let msg = format!("{} wajib diisi", prop.name);
                                errors.push(msg.clone());
                                notification.set(Some(msg));
                            }
                        }
                    }
                    view! {
                        <div>
                            <label>Label: <input value=label on:input=move |ev| {
                                let val = event_target_value(&ev);
                                if let Some(i) = idx() {
                                    components.update(|c| {
                                        if let CanvasComponent::Button { label } = &mut c[i] {
                                            *label = val.clone();
                                        }
                                    });
                                }
                            } /></label>
                            <ul style="color:red;list-style:circle;margin:0.5em 0 0 1em;">
                                {errors.into_iter().map(|e| view!{ <li>{e}</li> }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }
                },
                Some(CanvasComponent::Text { content }) => {
                    let schema = component_library.get().iter().find(|c| c.kind == "Text").and_then(|c| c.props_schema.clone());
                    let mut errors = vec![];
                    if let Some(props) = &schema {
                        for prop in props {
                            if prop.required && content.trim().is_empty() {
                                let msg = format!("{} wajib diisi", prop.name);
                                errors.push(msg.clone());
                                notification.set(Some(msg));
                            }
                        }
                    }
                    view! {
                        <div>
                            <label>Content: <input value=content on:input=move |ev| {
                                let val = event_target_value(&ev);
                                if let Some(i) = idx() {
                                    components.update(|c| {
                                        if let CanvasComponent::Text { content } = &mut c[i] {
                                            *content = val.clone();
                                        }
                                    });
                                }
                            } /></label>
                            <ul style="color:red;list-style:circle;margin:0.5em 0 0 1em;">
                                {errors.into_iter().map(|e| view!{ <li>{e}</li> }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }
                },
                Some(CanvasComponent::Input { placeholder }) => {
                    let schema = component_library.get().iter().find(|c| c.kind == "Input").and_then(|c| c.props_schema.clone());
                    let mut errors = vec![];
                    if let Some(props) = &schema {
                        for prop in props {
                            if prop.required && placeholder.trim().is_empty() {
                                let msg = format!("{} wajib diisi", prop.name);
                                errors.push(msg.clone());
                                notification.set(Some(msg));
                            }
                        }
                    }
                    view! {
                        <div>
                            <label>Placeholder: <input value=placeholder on:input=move |ev| {
                                let val = event_target_value(&ev);
                                if let Some(i) = idx() {
                                    components.update(|c| {
                                        if let CanvasComponent::Input { placeholder } = &mut c[i] {
                                            *placeholder = val.clone();
                                        }
                                    });
                                }
                            } /></label>
                            <ul style="color:red;list-style:circle;margin:0.5em 0 0 1em;">
                                {errors.into_iter().map(|e| view!{ <li>{e}</li> }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }
                },
                Some(CanvasComponent::Custom { name }) => {
                    let template = custom_components.get().iter().find(|c| c.name == *name).and_then(|c| c.template.clone()).unwrap_or_default();
                    let schema = component_library.get().iter().find(|c| c.kind == "Custom" && c.name == *name).and_then(|c| c.props_schema.clone());
                    let mut errors = vec![];
                    if let Some(props) = &schema {
                        for prop in props {
                            if prop.required && template.trim().is_empty() {
                                let msg = format!("{} wajib diisi", prop.name);
                                errors.push(msg.clone());
                                notification.set(Some(msg));
                            }
                        }
                    }
                    view! {
                        <div>
                            <label>Template: <textarea value=template.clone() on:input=move |ev| {
                                let val = event_target_value(&ev);
                                custom_components.update(|cc| {
                                    if let Some(comp) = cc.iter_mut().find(|c| c.name == *name) {
                                        comp.template = Some(val.clone());
                                    }
                                });
                            } /></label>
                            <ul style="color:red;list-style:circle;margin:0.5em 0 0 1em;">
                                {errors.into_iter().map(|e| view!{ <li>{e}</li> }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }
                },
                _ => view! { <div><p>Pilih komponen untuk mengedit properti.</p></div> },
            }}
        </section>
    }
}
