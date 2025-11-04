use crate::domain::CanvasComponent;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn PropertyEditor() -> impl IntoView {
    // Get app state from context
    let app_state = AppState::use_context();
    let canvas_state = app_state.canvas;

    view! {
        <section class="property-editor">
            <h3>{"Property Editor"}</h3>
            {move || {
                if let Some(selected_id) = canvas_state.selected.get() {
                    if let Some(comp) = canvas_state.get_component(&selected_id) {
                        match comp {
                            CanvasComponent::Button(btn) => {
                                let label_value = btn.label.clone();
                                let comp_id = btn.id.clone();
                                view! {
                                    <div>
                                        <label>
                                            {"Label: "}
                                            <input 
                                                type="text"
                                                prop:value=label_value
                                                on:input=move |ev| {
                                                    let new_label = event_target_value(&ev);
                                                    let mut updated_btn = btn.clone();
                                                    updated_btn.label = new_label;
                                                    canvas_state.update_component(&comp_id, CanvasComponent::Button(updated_btn));
                                                }
                                            />
                                        </label>
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Text(txt) => {
                                let content_value = txt.content.clone();
                                let comp_id = txt.id.clone();
                                view! {
                                    <div>
                                        <label>
                                            {"Content: "}
                                            <input 
                                                type="text"
                                                prop:value=content_value
                                                on:input=move |ev| {
                                                    let new_content = event_target_value(&ev);
                                                    let mut updated_txt = txt.clone();
                                                    updated_txt.content = new_content;
                                                    canvas_state.update_component(&comp_id, CanvasComponent::Text(updated_txt));
                                                }
                                            />
                                        </label>
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Input(inp) => {
                                let placeholder_value = inp.placeholder.clone();
                                let comp_id = inp.id.clone();
                                view! {
                                    <div>
                                        <label>
                                            {"Placeholder: "}
                                            <input 
                                                type="text"
                                                prop:value=placeholder_value
                                                on:input=move |ev| {
                                                    let new_placeholder = event_target_value(&ev);
                                                    let mut updated_inp = inp.clone();
                                                    updated_inp.placeholder = new_placeholder;
                                                    canvas_state.update_component(&comp_id, CanvasComponent::Input(updated_inp));
                                                }
                                            />
                                        </label>
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Container(_) => {
                                view! {
                                    <div><p>{"Container properties"}</p></div>
                                }.into_any()
                            },
                            CanvasComponent::Custom(custom) => {
                                let name_value = custom.name.clone();
                                let template_value = custom.template.clone();
                                let comp_id = custom.id.clone();
                                let custom_for_name = custom.clone();
                                let custom_for_template = custom.clone();
                                let comp_id_for_name = comp_id.clone();
                                let comp_id_for_template = comp_id.clone();
                                
                                view! {
                                    <div>
                                        <label>
                                            {"Name: "}
                                            <input 
                                                type="text"
                                                prop:value=name_value
                                                on:input=move |ev| {
                                                    let new_name = event_target_value(&ev);
                                                    let mut updated_custom = custom_for_name.clone();
                                                    updated_custom.name = new_name;
                                                    canvas_state.update_component(&comp_id_for_name, CanvasComponent::Custom(updated_custom));
                                                }
                                            />
                                        </label>
                                        <label>
                                            {"Template: "}
                                            <textarea
                                                prop:value=template_value
                                                on:input=move |ev| {
                                                    let new_template = event_target_value(&ev);
                                                    let mut updated_custom = custom_for_template.clone();
                                                    updated_custom.template = new_template;
                                                    canvas_state.update_component(&comp_id_for_template, CanvasComponent::Custom(updated_custom));
                                                }
                                            />
                                        </label>
                                    </div>
                                }.into_any()
                            },
                        }
                    } else {
                        view! { <div><p>{"Component not found"}</p></div> }.into_any()
                    }
                } else {
                    view! { <div><p>{"Select a component to edit properties"}</p></div> }.into_any()
                }
            }}
        </section>
    }
}
