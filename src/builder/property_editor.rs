use crate::builder::component_library::PropType;
use crate::domain::{
    ButtonSize, ButtonVariant, CanvasComponent, InputType, PropValue, TextStyle, TextTag,
};
use crate::services::{
    update_button_prop, update_container_prop, update_input_prop, update_text_prop,
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
                                // Read Button schema from component_library
                                let button_schema = app_state
                                    .ui
                                    .component_library
                                    .get()
                                    .into_iter()
                                    .find(|c| c.kind == "Button")
                                    .and_then(|c| c.props_schema)
                                    .unwrap_or_default();

                                let comp_id = btn.id.clone();

                                view! {
                                    <div>
                                        {button_schema
                                            .into_iter()
                                            .map(|prop| {
                                                let prop_name = prop.name.clone();
                                                let prop_type = prop.prop_type.clone();
                                                let label_text = prop.name.clone();

                                                // Capture current values for this field
                                                let current_string = match prop_name.as_str() {
                                                    "label" => Some(btn.label.clone()),
                                                    "variant" => Some(match btn.variant.clone() {
                                                        ButtonVariant::Primary => "Primary".to_string(),
                                                        ButtonVariant::Secondary => "Secondary".to_string(),
                                                        ButtonVariant::Outline => "Outline".to_string(),
                                                        ButtonVariant::Ghost => "Ghost".to_string(),
                                                    }),
                                                    "size" => Some(match btn.size.clone() {
                                                        ButtonSize::Small => "Small".to_string(),
                                                        ButtonSize::Medium => "Medium".to_string(),
                                                        ButtonSize::Large => "Large".to_string(),
                                                    }),
                                                    _ => None,
                                                };

                                                let current_bool = match prop_name.as_str() {
                                                    "disabled" => Some(btn.disabled),
                                                    _ => None,
                                                };

                                                let comp_id_field = comp_id.clone();
                                                let btn_for_field = btn.clone();

                                                view! {
                                                    <div class="property-field">
                                                        <label>
                                                            {label_text.clone()}
                                                            {match prop_type {
                                                                PropType::String => {
                                                                    let value = current_string.unwrap_or_default();
                                                                    view! {
                                                                        <input
                                                                            type="text"
                                                                            prop:value=value.clone()
                                                                            on:input=move |ev| {
                                                                                let new_value = event_target_value(&ev);
                                                                                let updated_btn = update_button_prop(
                                                                                    btn_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::String(new_value),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Button(updated_btn));
                                                                            }
                                                                        />
                                                                    }.into_any()
                                                                },
                                                                PropType::Bool => {
                                                                    let checked = current_bool.unwrap_or(false);
                                                                    view! {
                                                                        <input
                                                                            type="checkbox"
                                                                            prop:checked=checked
                                                                            on:change=move |_| {
                                                                                let updated_btn = update_button_prop(
                                                                                    btn_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::Boolean(!checked),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Button(updated_btn));
                                                                            }
                                                                        />
                                                                    }.into_any()
                                                                },
                                                                PropType::Enum { options } => {
                                                                    let selected_value = current_string.unwrap_or_default();
                                                                    view! {
                                                                        <select
                                                                            on:change=move |ev| {
                                                                                let value = event_target_value(&ev);
                                                                                let updated_btn = update_button_prop(
                                                                                    btn_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::String(value),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Button(updated_btn));
                                                                            }
                                                                        >
                                                                            {options.into_iter().map(|opt| {
                                                                                let opt_clone = opt.clone();
                                                                                let is_selected = opt_clone == selected_value;
                                                                                view! {
                                                                                    <option value=opt_clone selected=is_selected>{opt}</option>
                                                                                }
                                                                            }).collect::<Vec<_>>()}
                                                                        </select>
                                                                    }.into_any()
                                                                },
                                                                _ => {
                                                                    view! { <div></div> }.into_any()
                                                                }
                                                            }}
                                                        </label>
                                                    </div>
                                                }.into_any()
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Text(txt) => {
                                // Read Text schema from component_library
                                let text_schema = app_state
                                    .ui
                                    .component_library
                                    .get()
                                    .into_iter()
                                    .find(|c| c.kind == "Text")
                                    .and_then(|c| c.props_schema)
                                    .unwrap_or_default();

                                let comp_id = txt.id.clone();

                                view! {
                                    <div>
                                        {text_schema
                                            .into_iter()
                                            .map(|prop| {
                                                let prop_name = prop.name.clone();
                                                let prop_type = prop.prop_type.clone();
                                                let label_text = prop.name.clone();

                                                let current_string = match prop_name.as_str() {
                                                    "content" => Some(txt.content.clone()),
                                                    "style" => Some(match txt.style.clone() {
                                                        TextStyle::Heading1 => "Heading1".to_string(),
                                                        TextStyle::Heading2 => "Heading2".to_string(),
                                                        TextStyle::Heading3 => "Heading3".to_string(),
                                                        TextStyle::Body => "Body".to_string(),
                                                        TextStyle::Caption => "Caption".to_string(),
                                                    }),
                                                    "tag" => Some(match txt.tag.clone() {
                                                        TextTag::H1 => "H1".to_string(),
                                                        TextTag::H2 => "H2".to_string(),
                                                        TextTag::H3 => "H3".to_string(),
                                                        TextTag::P => "P".to_string(),
                                                        TextTag::Span => "Span".to_string(),
                                                    }),
                                                    _ => None,
                                                };

                                                let comp_id_field = comp_id.clone();
                                                let txt_for_field = txt.clone();

                                                view! {
                                                    <div class="property-field">
                                                        <label>
                                                            {label_text.clone()}
                                                            {match prop_type {
                                                                PropType::String => {
                                                                    let value = current_string.unwrap_or_default();
                                                                    view! {
                                                                        <input
                                                                            type="text"
                                                                            prop:value=value.clone()
                                                                            on:input=move |ev| {
                                                                                let new_value = event_target_value(&ev);
                                                                                let updated_txt = update_text_prop(
                                                                                    txt_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::String(new_value),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Text(updated_txt));
                                                                            }
                                                                        />
                                                                    }.into_any()
                                                                },
                                                                PropType::Enum { options } => {
                                                                    let selected_value = current_string.unwrap_or_default();
                                                                    view! {
                                                                        <select
                                                                            on:change=move |ev| {
                                                                                let value = event_target_value(&ev);
                                                                                let updated_txt = update_text_prop(
                                                                                    txt_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::String(value),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Text(updated_txt));
                                                                            }
                                                                        >
                                                                            {options.into_iter().map(|opt| {
                                                                                let opt_clone = opt.clone();
                                                                                let is_selected = opt_clone == selected_value;
                                                                                view! {
                                                                                    <option value=opt_clone selected=is_selected>{opt}</option>
                                                                                }
                                                                            }).collect::<Vec<_>>()}
                                                                        </select>
                                                                    }.into_any()
                                                                },
                                                                _ => {
                                                                    view! { <div></div> }.into_any()
                                                                }
                                                            }}
                                                        </label>
                                                    </div>
                                                }.into_any()
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Input(inp) => {
                                // Read Input schema from component_library
                                let input_schema = app_state
                                    .ui
                                    .component_library
                                    .get()
                                    .into_iter()
                                    .find(|c| c.kind == "Input")
                                    .and_then(|c| c.props_schema)
                                    .unwrap_or_default();

                                let comp_id = inp.id.clone();

                                view! {
                                    <div>
                                        {input_schema
                                            .into_iter()
                                            .map(|prop| {
                                                let prop_name = prop.name.clone();
                                                let prop_type = prop.prop_type.clone();
                                                let label_text = prop.name.clone();

                                                let current_string = match prop_name.as_str() {
                                                    "placeholder" => Some(inp.placeholder.clone()),
                                                    "input_type" => Some(match inp.input_type.clone() {
                                                        InputType::Text => "Text".to_string(),
                                                        InputType::Password => "Password".to_string(),
                                                        InputType::Email => "Email".to_string(),
                                                        InputType::Number => "Number".to_string(),
                                                        InputType::Tel => "Tel".to_string(),
                                                    }),
                                                    _ => None,
                                                };

                                                let current_bool = match prop_name.as_str() {
                                                    "required" => Some(inp.required),
                                                    "disabled" => Some(inp.disabled),
                                                    _ => None,
                                                };

                                                let comp_id_field = comp_id.clone();
                                                let inp_for_field = inp.clone();

                                                view! {
                                                    <div class="property-field">
                                                        <label>
                                                            {label_text.clone()}
                                                            {match prop_type {
                                                                PropType::String => {
                                                                    let value = current_string.unwrap_or_default();
                                                                    view! {
                                                                        <input
                                                                            type="text"
                                                                            prop:value=value.clone()
                                                                            on:input=move |ev| {
                                                                                let new_value = event_target_value(&ev);
                                                                                let updated_inp = update_input_prop(
                                                                                    inp_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::String(new_value),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Input(updated_inp));
                                                                            }
                                                                        />
                                                                    }.into_any()
                                                                },
                                                                PropType::Bool => {
                                                                    let checked = current_bool.unwrap_or(false);
                                                                    view! {
                                                                        <input
                                                                            type="checkbox"
                                                                            prop:checked=checked
                                                                            on:change=move |_| {
                                                                                let updated_inp = update_input_prop(
                                                                                    inp_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::Boolean(!checked),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Input(updated_inp));
                                                                            }
                                                                        />
                                                                    }.into_any()
                                                                },
                                                                PropType::Enum { options } => {
                                                                    let selected_value = current_string.unwrap_or_default();
                                                                    view! {
                                                                        <select
                                                                            on:change=move |ev| {
                                                                                let value = event_target_value(&ev);
                                                                                let updated_inp = update_input_prop(
                                                                                    inp_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::String(value),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Input(updated_inp));
                                                                            }
                                                                        >
                                                                            {options.into_iter().map(|opt| {
                                                                                let opt_clone = opt.clone();
                                                                                let is_selected = opt_clone == selected_value;
                                                                                view! {
                                                                                    <option value=opt_clone selected=is_selected>{opt}</option>
                                                                                }
                                                                            }).collect::<Vec<_>>()}
                                                                        </select>
                                                                    }.into_any()
                                                                },
                                                                _ => {
                                                                    view! { <div></div> }.into_any()
                                                                }
                                                            }}
                                                        </label>
                                                    </div>
                                                }.into_any()
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            },
                            CanvasComponent::Container(container) => {
                                // Read Container schema from component_library
                                let container_schema = app_state
                                    .ui
                                    .component_library
                                    .get()
                                    .into_iter()
                                    .find(|c| c.kind == "Container")
                                    .and_then(|c| c.props_schema)
                                    .unwrap_or_default();

                                let comp_id = container.id.clone();

                                view! {
                                    <div>
                                        {container_schema
                                            .into_iter()
                                            .map(|prop| {
                                                let prop_name = prop.name.clone();
                                                let prop_type = prop.prop_type.clone();
                                                let label_text = prop.name.clone();

                                                let comp_id_field = comp_id.clone();
                                                let container_for_field = container.clone();

                                                view! {
                                                    <div class="property-field">
                                                        <label>
                                                            {label_text.clone()}
                                                            {match prop_type {
                                                                PropType::Enum { options } => {
                                                                    let selected_value = match prop_name.as_str() {
                                                                        "layout" => {
                                                                            match &container_for_field.layout {
                                                                                crate::domain::LayoutType::Flex { direction, .. } => {
                                                                                    match direction {
                                                                                        crate::domain::FlexDirection::Row => "FlexRow".to_string(),
                                                                                        crate::domain::FlexDirection::Column => "FlexColumn".to_string(),
                                                                                    }
                                                                                }
                                                                                crate::domain::LayoutType::Grid { .. } => "Grid".to_string(),
                                                                                crate::domain::LayoutType::Stack => "Stack".to_string(),
                                                                            }
                                                                        }
                                                                        _ => String::new(),
                                                                    };

                                                                    view! {
                                                                        <select
                                                                            on:change=move |ev| {
                                                                                let value = event_target_value(&ev);
                                                                                let updated_container = update_container_prop(
                                                                                    container_for_field.clone(),
                                                                                    prop_name.as_str(),
                                                                                    PropValue::String(value),
                                                                                );
                                                                                canvas_state.update_component(&comp_id_field, CanvasComponent::Container(updated_container));
                                                                            }
                                                                        >
                                                                            {options.into_iter().map(|opt| {
                                                                                let opt_clone = opt.clone();
                                                                                let is_selected = opt_clone == selected_value;
                                                                                view! {
                                                                                    <option value=opt_clone selected=is_selected>{opt}</option>
                                                                                }
                                                                            }).collect::<Vec<_>>()}
                                                                        </select>
                                                                    }.into_any()
                                                                },
                                                                PropType::Number => {
                                                                    let value = match prop_name.as_str() {
                                                                        "gap" => container_for_field.gap.to_string(),
                                                                        "padding_top" => container_for_field.padding.top.to_string(),
                                                                        "padding_right" => container_for_field.padding.right.to_string(),
                                                                        "padding_bottom" => container_for_field.padding.bottom.to_string(),
                                                                        "padding_left" => container_for_field.padding.left.to_string(),
                                                                        _ => String::new(),
                                                                    };

                                                                    view! {
                                                                        <input
                                                                            type="number"
                                                                            prop:value=value.clone()
                                                                            on:input=move |ev| {
                                                                                let raw = event_target_value(&ev);
                                                                                if let Ok(parsed) = raw.parse::<f64>() {
                                                                                    let updated_container = update_container_prop(
                                                                                        container_for_field.clone(),
                                                                                        prop_name.as_str(),
                                                                                        PropValue::Number(parsed),
                                                                                    );
                                                                                    canvas_state.update_component(&comp_id_field, CanvasComponent::Container(updated_container));
                                                                                }
                                                                            }
                                                                        />
                                                                    }.into_any()
                                                                },
                                                                _ => {
                                                                    ().into_any()
                                                                }
                                                            }}
                                                        </label>
                                                    </div>
                                                }.into_any()
                                            })
                                            .collect::<Vec<_>>()}
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
