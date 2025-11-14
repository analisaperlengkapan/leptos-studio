use crate::domain::{
    ButtonSize,
    ButtonVariant,
    CanvasComponent,
    InputType,
    PropValue,
    TextStyle,
    TextTag,
};
use crate::services::{
    update_button_prop,
    update_input_prop,
    update_text_prop,
};
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
                                let variant_str = match btn.variant.clone() {
                                    ButtonVariant::Primary => "Primary",
                                    ButtonVariant::Secondary => "Secondary",
                                    ButtonVariant::Outline => "Outline",
                                    ButtonVariant::Ghost => "Ghost",
                                };
                                let size_str = match btn.size.clone() {
                                    ButtonSize::Small => "Small",
                                    ButtonSize::Medium => "Medium",
                                    ButtonSize::Large => "Large",
                                };
                                let disabled_value = btn.disabled;

                                let comp_id = btn.id.clone();
                                let btn_for_variant = btn.clone();
                                let btn_for_size = btn.clone();
                                let btn_for_disabled = btn.clone();
                                let comp_id_for_variant = comp_id.clone();
                                let comp_id_for_size = comp_id.clone();
                                let comp_id_for_disabled = comp_id.clone();

                                view! {
                                    <div>
                                        <label>
                                            {"Label: "}
                                            <input 
                                                type="text"
                                                prop:value=label_value
                                                on:input=move |ev| {
                                                    let new_label = event_target_value(&ev);
                                                    let updated_btn = update_button_prop(
                                                        btn.clone(),
                                                        "label",
                                                        PropValue::String(new_label),
                                                    );
                                                    canvas_state.update_component(&comp_id, CanvasComponent::Button(updated_btn));
                                                }
                                            />
                                        </label>

                                        <label>
                                            {"Variant: "}
                                            <select
                                                on:change=move |ev| {
                                                    let value = event_target_value(&ev);
                                                    let updated_btn = update_button_prop(
                                                        btn_for_variant.clone(),
                                                        "variant",
                                                        PropValue::String(value),
                                                    );
                                                    canvas_state.update_component(&comp_id_for_variant, CanvasComponent::Button(updated_btn));
                                                }
                                            >
                                                <option
                                                    value="Primary"
                                                    selected={variant_str == "Primary"}
                                                >{"Primary"}</option>
                                                <option
                                                    value="Secondary"
                                                    selected={variant_str == "Secondary"}
                                                >{"Secondary"}</option>
                                                <option
                                                    value="Outline"
                                                    selected={variant_str == "Outline"}
                                                >{"Outline"}</option>
                                                <option
                                                    value="Ghost"
                                                    selected={variant_str == "Ghost"}
                                                >{"Ghost"}</option>
                                            </select>
                                        </label>

                                        <label>
                                            {"Size: "}
                                            <select
                                                on:change=move |ev| {
                                                    let value = event_target_value(&ev);
                                                    let updated_btn = update_button_prop(
                                                        btn_for_size.clone(),
                                                        "size",
                                                        PropValue::String(value),
                                                    );
                                                    canvas_state.update_component(&comp_id_for_size, CanvasComponent::Button(updated_btn));
                                                }
                                            >
                                                <option
                                                    value="Small"
                                                    selected={size_str == "Small"}
                                                >{"Small"}</option>
                                                <option
                                                    value="Medium"
                                                    selected={size_str == "Medium"}
                                                >{"Medium"}</option>
                                                <option
                                                    value="Large"
                                                    selected={size_str == "Large"}
                                                >{"Large"}</option>
                                            </select>
                                        </label>

                                        <label>
                                            {"Disabled: "}
                                            <input
                                                type="checkbox"
                                                prop:checked=disabled_value
                                                on:change=move |_| {
                                                    let updated_btn = update_button_prop(
                                                        btn_for_disabled.clone(),
                                                        "disabled",
                                                        PropValue::Boolean(!disabled_value),
                                                    );
                                                    canvas_state.update_component(&comp_id_for_disabled, CanvasComponent::Button(updated_btn));
                                                }
                                            />
                                        </label>
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Text(txt) => {
                                let content_value = txt.content.clone();
                                let style_str = match txt.style.clone() {
                                    TextStyle::Heading1 => "Heading1",
                                    TextStyle::Heading2 => "Heading2",
                                    TextStyle::Heading3 => "Heading3",
                                    TextStyle::Body => "Body",
                                    TextStyle::Caption => "Caption",
                                };
                                let tag_str = match txt.tag.clone() {
                                    TextTag::H1 => "H1",
                                    TextTag::H2 => "H2",
                                    TextTag::H3 => "H3",
                                    TextTag::P => "P",
                                    TextTag::Span => "Span",
                                };

                                let comp_id = txt.id.clone();
                                let txt_for_style = txt.clone();
                                let txt_for_tag = txt.clone();
                                let comp_id_for_style = comp_id.clone();
                                let comp_id_for_tag = comp_id.clone();

                                view! {
                                    <div>
                                        <label>
                                            {"Content: "}
                                            <input 
                                                type="text"
                                                prop:value=content_value
                                                on:input=move |ev| {
                                                    let new_content = event_target_value(&ev);
                                                    let updated_txt = update_text_prop(
                                                        txt.clone(),
                                                        "content",
                                                        PropValue::String(new_content),
                                                    );
                                                    canvas_state.update_component(&comp_id, CanvasComponent::Text(updated_txt));
                                                }
                                            />
                                        </label>

                                        <label>
                                            {"Style: "}
                                            <select
                                                on:change=move |ev| {
                                                    let value = event_target_value(&ev);
                                                    let updated_txt = update_text_prop(
                                                        txt_for_style.clone(),
                                                        "style",
                                                        PropValue::String(value),
                                                    );
                                                    canvas_state.update_component(&comp_id_for_style, CanvasComponent::Text(updated_txt));
                                                }
                                            >
                                                <option value="Heading1" selected={style_str == "Heading1"}>{"Heading 1"}</option>
                                                <option value="Heading2" selected={style_str == "Heading2"}>{"Heading 2"}</option>
                                                <option value="Heading3" selected={style_str == "Heading3"}>{"Heading 3"}</option>
                                                <option value="Body" selected={style_str == "Body"}>{"Body"}</option>
                                                <option value="Caption" selected={style_str == "Caption"}>{"Caption"}</option>
                                            </select>
                                        </label>

                                        <label>
                                            {"Tag: "}
                                            <select
                                                on:change=move |ev| {
                                                    let value = event_target_value(&ev);
                                                    let updated_txt = update_text_prop(
                                                        txt_for_tag.clone(),
                                                        "tag",
                                                        PropValue::String(value),
                                                    );
                                                    canvas_state.update_component(&comp_id_for_tag, CanvasComponent::Text(updated_txt));
                                                }
                                            >
                                                <option value="H1" selected={tag_str == "H1"}>{"H1"}</option>
                                                <option value="H2" selected={tag_str == "H2"}>{"H2"}</option>
                                                <option value="H3" selected={tag_str == "H3"}>{"H3"}</option>
                                                <option value="P" selected={tag_str == "P"}>{"P"}</option>
                                                <option value="Span" selected={tag_str == "Span"}>{"Span"}</option>
                                            </select>
                                        </label>
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Input(inp) => {
                                let placeholder_value = inp.placeholder.clone();
                                let type_str = match inp.input_type.clone() {
                                    InputType::Text => "Text",
                                    InputType::Password => "Password",
                                    InputType::Email => "Email",
                                    InputType::Number => "Number",
                                    InputType::Tel => "Tel",
                                };
                                let required_value = inp.required;
                                let disabled_value = inp.disabled;

                                let comp_id = inp.id.clone();
                                let inp_for_type = inp.clone();
                                let inp_for_required = inp.clone();
                                let inp_for_disabled = inp.clone();
                                let comp_id_for_type = comp_id.clone();
                                let comp_id_for_required = comp_id.clone();
                                let comp_id_for_disabled = comp_id.clone();

                                view! {
                                    <div>
                                        <label>
                                            {"Placeholder: "}
                                            <input 
                                                type="text"
                                                prop:value=placeholder_value
                                                on:input=move |ev| {
                                                    let new_placeholder = event_target_value(&ev);
                                                    let updated_inp = update_input_prop(
                                                        inp.clone(),
                                                        "placeholder",
                                                        PropValue::String(new_placeholder),
                                                    );
                                                    canvas_state.update_component(&comp_id, CanvasComponent::Input(updated_inp));
                                                }
                                            />
                                        </label>

                                        <label>
                                            {"Type: "}
                                            <select
                                                on:change=move |ev| {
                                                    let value = event_target_value(&ev);
                                                    let updated_inp = update_input_prop(
                                                        inp_for_type.clone(),
                                                        "input_type",
                                                        PropValue::String(value),
                                                    );
                                                    canvas_state.update_component(&comp_id_for_type, CanvasComponent::Input(updated_inp));
                                                }
                                            >
                                                <option value="Text" selected={type_str == "Text"}>{"Text"}</option>
                                                <option value="Password" selected={type_str == "Password"}>{"Password"}</option>
                                                <option value="Email" selected={type_str == "Email"}>{"Email"}</option>
                                                <option value="Number" selected={type_str == "Number"}>{"Number"}</option>
                                                <option value="Tel" selected={type_str == "Tel"}>{"Tel"}</option>
                                            </select>
                                        </label>

                                        <label>
                                            {"Required: "}
                                            <input
                                                type="checkbox"
                                                prop:checked=required_value
                                                on:change=move |_| {
                                                    let updated_inp = update_input_prop(
                                                        inp_for_required.clone(),
                                                        "required",
                                                        PropValue::Boolean(!required_value),
                                                    );
                                                    canvas_state.update_component(&comp_id_for_required, CanvasComponent::Input(updated_inp));
                                                }
                                            />
                                        </label>

                                        <label>
                                            {"Disabled: "}
                                            <input
                                                type="checkbox"
                                                prop:checked=disabled_value
                                                on:change=move |_| {
                                                    let updated_inp = update_input_prop(
                                                        inp_for_disabled.clone(),
                                                        "disabled",
                                                        PropValue::Boolean(!disabled_value),
                                                    );
                                                    canvas_state.update_component(&comp_id_for_disabled, CanvasComponent::Input(updated_inp));
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
