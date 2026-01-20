use super::AnimationPropertyEditor;
use crate::builder::component_library::PropType;
use crate::builder::property_inputs::{BoolCheckbox, EnumSelect, StringInput};
use crate::builder::variable_binding::VariableBinding;
use crate::domain::{CanvasComponent, ComponentId, InputType, PropValue};
use crate::services::update_input_prop;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn InputPropertyEditor(
    id: ComponentId,
    #[prop(into)] input: crate::domain::InputComponent,
) -> impl IntoView {
    let app_state = AppState::expect_context();
    let ui_state = app_state.ui;
    let canvas_state = app_state.canvas;

    let input_schema = ui_state
        .component_library
        .get()
        .into_iter()
        .find(|c| c.kind == "Input")
        .and_then(|c| c.props_schema)
        .unwrap_or_default();

    let comp_id = id;

    let apply_update = move |id: ComponentId, updated: CanvasComponent, prop_name: String| {
        if let Err(e) = updated.validate() {
            ui_state.notify(crate::state::Notification::error(e.user_message()));
        } else {
            // Overwrite using new signature (closure that assigns the value)
            canvas_state.update_component_with_snapshot(
                &id,
                updated,
                &format!("Update Input {}", prop_name),
            );
        }
    };

    view! {
        <div class="property-group">
                <div class="group-title">"Input Properties"</div>
                {input_schema.into_iter().map(|prop| {
                let prop_name = prop.name.clone();
                let prop_type = prop.prop_type.clone();
                let label_text = prop.name.clone();
                let comp_id_field = comp_id;
                let inp_for_field = input.clone();

                match prop_type {
                    PropType::String => {
                        let value = match prop_name.as_str() {
                            "placeholder" => inp_for_field.placeholder.clone(),
                            _ => String::new(),
                        };

                        let prop_name_for_input = prop_name.clone();
                        let inp_for_input = inp_for_field.clone();
                        let comp_id_input = comp_id_field;

                        let prop_name_for_bind = prop_name.clone();
                        let inp_for_bind_read = inp_for_field.clone();
                        let inp_for_bind_closure = inp_for_field.clone();
                        let comp_id_bind = comp_id_field;

                        view! {
                            <div style="display: flex; align-items: flex-end;">
                                <div style="flex-grow: 1;">
                                    <StringInput
                                        value=value
                                        label=label_text
                                        on_change=move |new_val| {
                                            let updated_inp = update_input_prop(inp_for_input.clone(), prop_name_for_input.as_str(), PropValue::String(new_val));
                                            apply_update(comp_id_input, CanvasComponent::Input(updated_inp), prop_name_for_input.clone());
                                        }
                                    />
                                </div>
                                {
                                    if prop_name_for_bind.as_str() == "placeholder" {
                                        let binding_val = inp_for_bind_read.bindings.get("placeholder").cloned();

                                        view! {
                                            <div style="margin-bottom: 8px;">
                                                <VariableBinding
                                                    value=binding_val
                                                    on_change=move |new_bind| {
                                                        let mut updated = inp_for_bind_closure.clone();
                                                        if let Some(v) = new_bind {
                                                            updated.bindings.insert("placeholder".to_string(), v);
                                                        } else {
                                                            updated.bindings.remove("placeholder");
                                                        }
                                                        apply_update(comp_id_bind, CanvasComponent::Input(updated), "placeholder binding".to_string());
                                                    }
                                                />
                                            </div>
                                        }.into_any()
                                    } else {
                                        ().into_any()
                                    }
                                }
                            </div>
                        }.into_any()
                    },
                        PropType::Enum { options } => {
                        let value = match prop_name.as_str() {
                            "input_type" => match inp_for_field.input_type {
                                InputType::Text => "Text",
                                InputType::Password => "Password",
                                InputType::Email => "Email",
                                InputType::Number => "Number",
                                InputType::Tel => "Tel",
                            }.to_string(),
                            _ => String::new(),
                        };
                        let prop_name_closure = prop_name.clone();
                        view! {
                            <EnumSelect
                                value=value
                                label=label_text
                                options=options
                                on_change=move |new_val| {
                                    let updated_inp = update_input_prop(inp_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                    apply_update(comp_id_field, CanvasComponent::Input(updated_inp), prop_name_closure.clone());
                                }
                            />
                        }.into_any()
                    },
                    PropType::Bool => {
                        let checked = match prop_name.as_str() {
                            "required" => inp_for_field.required,
                            "disabled" => inp_for_field.disabled,
                            _ => false,
                        };
                        let prop_name_closure = prop_name.clone();
                            view! {
                            <BoolCheckbox
                                checked=checked
                                label=label_text
                                on_change=move |new_val| {
                                    let updated_inp = update_input_prop(inp_for_field.clone(), prop_name.as_str(), PropValue::Boolean(new_val));
                                    apply_update(comp_id_field, CanvasComponent::Input(updated_inp), prop_name_closure.clone());
                                }
                            />
                        }.into_any()
                    },
                    _ => ().into_any()
                }
                }).collect::<Vec<_>>()}
        </div>

        <AnimationPropertyEditor
            _id=comp_id
            animation=input.animation.clone()
            on_change=move |new_anim| {
                let mut updated = input.clone();
                updated.animation = new_anim;
                apply_update(comp_id, CanvasComponent::Input(updated), "animation".to_string());
            }
        />
    }
}
