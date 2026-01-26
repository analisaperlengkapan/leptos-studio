use super::AnimationPropertyEditor;
use crate::builder::component_library::PropType;
use crate::builder::property_inputs::{EnumSelect, StringInput};
use crate::builder::variable_binding::VariableBinding;
use crate::domain::{CanvasComponent, ComponentId, PropValue, TextStyle, TextTag};
use crate::services::update_text_prop;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn TextPropertyEditor(
    id: ComponentId,
    #[prop(into)] text: crate::domain::TextComponent,
) -> impl IntoView {
    let app_state = AppState::expect_context();
    let ui_state = app_state.ui;
    let canvas_state = app_state.canvas;

    let text_schema = ui_state
        .component_library
        .get()
        .into_iter()
        .find(|c| c.kind == "Text")
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
                &format!("Update Text {}", prop_name),
            );
        }
    };

    view! {
        <div class="property-group">
                <div class="group-title">"Text Properties"</div>
                {text_schema.into_iter().map(|prop| {
                let prop_name = prop.name.clone();
                let prop_type = prop.prop_type.clone();
                let label_text = prop.name.clone();
                let comp_id_field = comp_id;
                let txt_for_field = text.clone();

                match prop_type {
                    PropType::String => {
                        let value = match prop_name.as_str() {
                            "content" => txt_for_field.content.clone(),
                            _ => String::new(),
                        };

                        let prop_name_for_input = prop_name.clone();
                        let txt_for_input = txt_for_field.clone();
                        let comp_id_input = comp_id_field;

                        let prop_name_for_bind = prop_name.clone();
                        let txt_for_bind_read = txt_for_field.clone();
                        let txt_for_bind_closure = txt_for_field.clone();
                        let comp_id_bind = comp_id_field;

                        view! {
                            <div style="display: flex; align-items: flex-end;">
                                <div style="flex-grow: 1;">
                                    <StringInput
                                        value=value
                                        label=label_text
                                        on_change=move |new_val| {
                                            let updated_txt = update_text_prop(txt_for_input.clone(), prop_name_for_input.as_str(), PropValue::String(new_val));
                                            apply_update(comp_id_input, CanvasComponent::Text(updated_txt), prop_name_for_input.clone());
                                        }
                                    />
                                </div>
                                {
                                    if prop_name_for_bind.as_str() == "content" {
                                        let binding_val = txt_for_bind_read.bindings.get("content").cloned();

                                        view! {
                                            <div style="margin-bottom: 8px;">
                                                <VariableBinding
                                                    value=binding_val
                                                    on_change=move |new_bind| {
                                                        let mut updated = txt_for_bind_closure.clone();
                                                        if let Some(v) = new_bind {
                                                            updated.bindings.insert("content".to_string(), v);
                                                        } else {
                                                            updated.bindings.remove("content");
                                                        }
                                                        apply_update(comp_id_bind, CanvasComponent::Text(updated), "content binding".to_string());
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
                            "style" => match txt_for_field.style {
                                TextStyle::Heading1 => "Heading1",
                                TextStyle::Heading2 => "Heading2",
                                TextStyle::Heading3 => "Heading3",
                                TextStyle::Body => "Body",
                                TextStyle::Caption => "Caption",
                            }.to_string(),
                            "tag" => match txt_for_field.tag {
                                TextTag::H1 => "H1",
                                TextTag::H2 => "H2",
                                TextTag::H3 => "H3",
                                TextTag::P => "P",
                                TextTag::Span => "Span",
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
                                    let updated_txt = update_text_prop(txt_for_field.clone(), prop_name.as_str(), PropValue::String(new_val));
                                    apply_update(comp_id_field, CanvasComponent::Text(updated_txt), prop_name_closure.clone());
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
            animation=text.animation.clone()
            on_change=move |new_anim| {
                let mut updated_txt = text.clone();
                updated_txt.animation = new_anim;
                apply_update(comp_id, CanvasComponent::Text(updated_txt), "animation".to_string());
            }
        />
    }
}
