use crate::builder::component_library::PropType;
use crate::builder::property_inputs::{BoolCheckbox, EnumSelect, NumberInput, StringInput};
use crate::domain::{
    ButtonSize, ButtonVariant, CanvasComponent, Component, InputType, PropValue, TextStyle, TextTag,
};
use crate::services::{
    update_button_prop, update_container_prop, update_input_prop, update_text_prop,
};
use crate::state::{AppState, Notification};
use leptos::prelude::*;

#[component]
pub fn PropertyEditor() -> impl IntoView {
    // Get app state from context
    let app_state = AppState::use_context();
    let canvas_state = app_state.canvas;
    let ui_state = app_state.ui;

    view! {
        <section class="property-editor">
            <h3>{"Property Editor"}</h3>
            {move || {
                if let Some(selected_id) = canvas_state.selected.get() {
                    if let Some(comp) = canvas_state.get_component(&selected_id) {
                        match comp {
                            CanvasComponent::Button(btn) => {
                                let button_schema = ui_state
                                    .component_library
                                    .get()
                                    .into_iter()
                                    .find(|c| c.kind == "Button")
                                    .and_then(|c| c.props_schema)
                                    .unwrap_or_default();

                                let comp_id = btn.id.clone();

                                view! {
                                    <div class="property-group">
                                        <div class="group-title">"Appearance"</div>
                                        {button_schema.into_iter().map(|prop| {
                                            let prop_name = prop.name.clone();
                                            let prop_type = prop.prop_type.clone();
                                            let label_text = prop.name.clone();
                                            let comp_id_field = comp_id.clone();
                                            let btn_for_field = btn.clone();

                                            match prop_type {
                                                PropType::String => {
                                                    // Determine current value
                                                    let value = match prop_name.as_str() {
                                                        "label" => btn.label.clone(),
                                                        _ => String::new(),
                                                    };
                                                    view! {
                                                        <StringInput
                                                            value=value
                                                            label=label_text
                                                            on_change=move |new_val| {
                                                                let updated_btn = update_button_prop(btn_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                                                // Validation
                                                                if let Err(e) = updated_btn.validate() {
                                                                     ui_state.notify(Notification::error(e.user_message()));
                                                                } else {
                                                                     canvas_state.update_component(&comp_id_field, CanvasComponent::Button(updated_btn));
                                                                }
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                PropType::Enum { options } => {
                                                     let value = match prop_name.as_str() {
                                                        "variant" => match btn.variant {
                                                            ButtonVariant::Primary => "Primary",
                                                            ButtonVariant::Secondary => "Secondary",
                                                            ButtonVariant::Outline => "Outline",
                                                            ButtonVariant::Ghost => "Ghost",
                                                        }.to_string(),
                                                        "size" => match btn.size {
                                                            ButtonSize::Small => "Small",
                                                            ButtonSize::Medium => "Medium",
                                                            ButtonSize::Large => "Large",
                                                        }.to_string(),
                                                        _ => String::new(),
                                                    };
                                                    view! {
                                                        <EnumSelect
                                                            value=value
                                                            label=label_text
                                                            options=options
                                                            on_change=move |new_val| {
                                                                let updated_btn = update_button_prop(btn_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Button(updated_btn));
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                PropType::Bool => {
                                                    let checked = match prop_name.as_str() {
                                                        "disabled" => btn.disabled,
                                                        _ => false,
                                                    };
                                                    view! {
                                                        <BoolCheckbox
                                                            checked=checked
                                                            label=label_text
                                                            on_change=move |new_val| {
                                                                let updated_btn = update_button_prop(btn_for_field.clone(), prop_name.as_str(), PropValue::Boolean(new_val));
                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Button(updated_btn));
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                _ => ().into_any(),
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Text(txt) => {
                                let text_schema = ui_state
                                    .component_library
                                    .get()
                                    .into_iter()
                                    .find(|c| c.kind == "Text")
                                    .and_then(|c| c.props_schema)
                                    .unwrap_or_default();

                                let comp_id = txt.id.clone();

                                view! {
                                    <div class="property-group">
                                         <div class="group-title">"Text Properties"</div>
                                         {text_schema.into_iter().map(|prop| {
                                            let prop_name = prop.name.clone();
                                            let prop_type = prop.prop_type.clone();
                                            let label_text = prop.name.clone();
                                            let comp_id_field = comp_id.clone();
                                            let txt_for_field = txt.clone();

                                            match prop_type {
                                                PropType::String => {
                                                    let value = match prop_name.as_str() {
                                                        "content" => txt.content.clone(),
                                                        _ => String::new(),
                                                    };
                                                    view! {
                                                        <StringInput
                                                            value=value
                                                            label=label_text
                                                            on_change=move |new_val| {
                                                                let updated_txt = update_text_prop(txt_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                                                // Validation
                                                                if let Err(e) = updated_txt.validate() {
                                                                     ui_state.notify(Notification::error(e.user_message()));
                                                                } else {
                                                                    canvas_state.update_component(&comp_id_field, CanvasComponent::Text(updated_txt));
                                                                }
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                PropType::Enum { options } => {
                                                    let value = match prop_name.as_str() {
                                                        "style" => match txt.style {
                                                            TextStyle::Heading1 => "Heading1",
                                                            TextStyle::Heading2 => "Heading2",
                                                            TextStyle::Heading3 => "Heading3",
                                                            TextStyle::Body => "Body",
                                                            TextStyle::Caption => "Caption",
                                                        }.to_string(),
                                                        "tag" => match txt.tag {
                                                            TextTag::H1 => "H1",
                                                            TextTag::H2 => "H2",
                                                            TextTag::H3 => "H3",
                                                            TextTag::P => "P",
                                                            TextTag::Span => "Span",
                                                        }.to_string(),
                                                        _ => String::new(),
                                                    };
                                                    view! {
                                                        <EnumSelect
                                                            value=value
                                                            label=label_text
                                                            options=options
                                                            on_change=move |new_val| {
                                                                let updated_txt = update_text_prop(txt_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Text(updated_txt));
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                _ => ().into_any()
                                            }
                                         }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            },
                             CanvasComponent::Input(inp) => {
                                let input_schema = ui_state
                                    .component_library
                                    .get()
                                    .into_iter()
                                    .find(|c| c.kind == "Input")
                                    .and_then(|c| c.props_schema)
                                    .unwrap_or_default();

                                let comp_id = inp.id.clone();

                                view! {
                                    <div class="property-group">
                                         <div class="group-title">"Input Properties"</div>
                                         {input_schema.into_iter().map(|prop| {
                                            let prop_name = prop.name.clone();
                                            let prop_type = prop.prop_type.clone();
                                            let label_text = prop.name.clone();
                                            let comp_id_field = comp_id.clone();
                                            let inp_for_field = inp.clone();

                                            match prop_type {
                                                PropType::String => {
                                                    let value = match prop_name.as_str() {
                                                        "placeholder" => inp.placeholder.clone(),
                                                        _ => String::new(),
                                                    };
                                                    view! {
                                                        <StringInput
                                                            value=value
                                                            label=label_text
                                                            on_change=move |new_val| {
                                                                let updated_inp = update_input_prop(inp_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                                                if let Err(e) = updated_inp.validate() {
                                                                     ui_state.notify(Notification::error(e.user_message()));
                                                                } else {
                                                                     canvas_state.update_component(&comp_id_field, CanvasComponent::Input(updated_inp));
                                                                }
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                 PropType::Enum { options } => {
                                                    let value = match prop_name.as_str() {
                                                        "input_type" => match inp.input_type {
                                                            InputType::Text => "Text",
                                                            InputType::Password => "Password",
                                                            InputType::Email => "Email",
                                                            InputType::Number => "Number",
                                                            InputType::Tel => "Tel",
                                                        }.to_string(),
                                                        _ => String::new(),
                                                    };
                                                    view! {
                                                        <EnumSelect
                                                            value=value
                                                            label=label_text
                                                            options=options
                                                            on_change=move |new_val| {
                                                                let updated_inp = update_input_prop(inp_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                                                if let Err(e) = updated_inp.validate() {
                                                                     ui_state.notify(Notification::error(e.user_message()));
                                                                } else {
                                                                     canvas_state.update_component(&comp_id_field, CanvasComponent::Input(updated_inp));
                                                                }
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                PropType::Bool => {
                                                    let checked = match prop_name.as_str() {
                                                        "required" => inp.required,
                                                        "disabled" => inp.disabled,
                                                        _ => false,
                                                    };
                                                     view! {
                                                        <BoolCheckbox
                                                            checked=checked
                                                            label=label_text
                                                            on_change=move |new_val| {
                                                                let updated_inp = update_input_prop(inp_for_field.clone(), prop_name.as_str(), PropValue::Boolean(new_val));
                                                                if let Err(e) = updated_inp.validate() {
                                                                     ui_state.notify(Notification::error(e.user_message()));
                                                                } else {
                                                                     canvas_state.update_component(&comp_id_field, CanvasComponent::Input(updated_inp));
                                                                }
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                _ => ().into_any()
                                            }
                                         }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Container(container) => {
                                let container_schema = ui_state
                                    .component_library
                                    .get()
                                    .into_iter()
                                    .find(|c| c.kind == "Container")
                                    .and_then(|c| c.props_schema)
                                    .unwrap_or_default();

                                let comp_id = container.id.clone();

                                view! {
                                     <div class="property-group">
                                         <div class="group-title">"Layout"</div>
                                         {container_schema.into_iter().map(|prop| {
                                            let prop_name = prop.name.clone();
                                            let prop_type = prop.prop_type.clone();
                                            let label_text = prop.name.clone();
                                            let comp_id_field = comp_id.clone();
                                            let container_for_field = container.clone();

                                            match prop_type {
                                                PropType::Enum { options } => {
                                                     let value = match prop_name.as_str() {
                                                        "layout" => match &container.layout {
                                                            crate::domain::LayoutType::Flex { direction, .. } => match direction {
                                                                crate::domain::FlexDirection::Row => "FlexRow",
                                                                crate::domain::FlexDirection::Column => "FlexColumn",
                                                            },
                                                            crate::domain::LayoutType::Grid { .. } => "Grid",
                                                            crate::domain::LayoutType::Stack => "Stack",
                                                        }.to_string(),
                                                        _ => String::new(),
                                                    };
                                                    view! {
                                                        <EnumSelect
                                                            value=value
                                                            label=label_text
                                                            options=options
                                                            on_change=move |new_val| {
                                                                let updated_container = update_container_prop(container_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                                                if let Err(e) = updated_container.validate() {
                                                                     ui_state.notify(Notification::error(e.user_message()));
                                                                } else {
                                                                     canvas_state.update_component(&comp_id_field, CanvasComponent::Container(updated_container));
                                                                }
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                PropType::Number => {
                                                    let value = match prop_name.as_str() {
                                                        "gap" => container.gap as f64,
                                                        "padding_top" => container.padding.top as f64,
                                                        "padding_right" => container.padding.right as f64,
                                                        "padding_bottom" => container.padding.bottom as f64,
                                                        "padding_left" => container.padding.left as f64,
                                                        _ => 0.0,
                                                    };
                                                     view! {
                                                        <NumberInput
                                                            value=value
                                                            label=label_text
                                                            on_change=move |new_val| {
                                                                let updated_container = update_container_prop(container_for_field.clone(), prop_name.as_str(), PropValue::Number(new_val));
                                                                if let Err(e) = updated_container.validate() {
                                                                     ui_state.notify(Notification::error(e.user_message()));
                                                                } else {
                                                                     canvas_state.update_component(&comp_id_field, CanvasComponent::Container(updated_container));
                                                                }
                                                            }
                                                        />
                                                    }.into_any()
                                                },
                                                _ => ().into_any()
                                            }
                                         }).collect::<Vec<_>>()}
                                     </div>
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
                                    <div class="property-group">
                                        <div class="group-title">"Custom Component"</div>
                                        <StringInput
                                            value=name_value
                                            label="Name".to_string()
                                            on_change=move |new_name| {
                                                let mut updated_custom = custom_for_name.clone();
                                                updated_custom.name = new_name;
                                                // Validation
                                                if let Err(e) = updated_custom.validate() {
                                                     ui_state.notify(Notification::error(e.user_message()));
                                                } else {
                                                     canvas_state.update_component(&comp_id_for_name, CanvasComponent::Custom(updated_custom));
                                                }
                                            }
                                        />
                                        <div class="property-field">
                                            <label>
                                                {"Template"}
                                                <textarea
                                                    prop:value=template_value
                                                    on:input=move |ev| {
                                                        let new_template = event_target_value(&ev);
                                                        let mut updated_custom = custom_for_template.clone();
                                                        updated_custom.template = new_template;
                                                        // Validation for template
                                                        if let Err(e) = updated_custom.validate() {
                                                             ui_state.notify(Notification::error(e.user_message()));
                                                        } else {
                                                             canvas_state.update_component(&comp_id_for_template, CanvasComponent::Custom(updated_custom));
                                                        }
                                                    }
                                                />
                                            </label>
                                        </div>
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
