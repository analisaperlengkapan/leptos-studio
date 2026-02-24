use super::AnimationPropertyEditor;
use crate::builder::component_library::PropType;
use crate::builder::property_inputs::{BoolCheckbox, EnumSelect, StringInput};
use crate::builder::styling_system::StyleEditor;
use crate::domain::{ButtonSize, ButtonVariant, CanvasComponent, ComponentId, PropValue};
use crate::services::update_button_prop;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn ButtonPropertyEditor(
    id: ComponentId,
    #[prop(into)] button: crate::domain::ButtonComponent,
) -> impl IntoView {
    let app_state = AppState::expect_context();
    let ui_state = app_state.ui;
    let canvas_state = app_state.canvas;

    let button_schema = ui_state
        .component_library
        .get()
        .into_iter()
        .find(|c| c.kind == "Button")
        .and_then(|c| c.props_schema)
        .unwrap_or_default();

    let comp_id = id;

    // Helper to update component
    let apply_update = move |id: ComponentId, updated: CanvasComponent, prop_name: String| {
        if let Err(e) = updated.validate() {
            ui_state.notify(crate::state::Notification::error(e.user_message()));
        } else {
            // We need to overwrite the component
            canvas_state.update_component_with_snapshot(
                &id,
                updated,
                &format!("Update Button {}", prop_name),
            );
        }
    };

    let btn_style = button.style.clone();
    let btn_animation = button.animation.clone();

    view! {
        <div class="property-group">
            <div class="group-title">"Appearance"</div>
            {button_schema.into_iter().map(|prop| {
                let prop_name = prop.name.clone();
                let prop_type = prop.prop_type.clone();
                let label_text = prop.name.clone();
                let comp_id_field = comp_id;

                // Capture values for view rendering, but NOT for closures
                let current_label = button.label.clone();
                let current_variant = button.variant.clone();
                let current_size = button.size.clone();
                let current_disabled = button.disabled;

                match prop_type {
                    PropType::String => {
                        let value = match prop_name.as_str() {
                            "label" => current_label.clone(),
                            _ => String::new(),
                        };
                        let prop_name_closure = prop_name.clone();
                        view! {
                            <StringInput
                                value=value
                                label=label_text
                                on_change=move |new_val| {
                                    if let Some(CanvasComponent::Button(latest_btn)) = canvas_state.get_component(&comp_id_field) {
                                        let updated_btn = update_button_prop(latest_btn, prop_name_closure.as_str(), PropValue::String(new_val));
                                        apply_update(comp_id_field, CanvasComponent::Button(updated_btn), prop_name_closure.clone());
                                    }
                                }
                            />
                        }.into_any()
                    },
                    PropType::Enum { options } => {
                        let value = match prop_name.as_str() {
                            "variant" => match current_variant {
                                ButtonVariant::Primary => "Primary",
                                ButtonVariant::Secondary => "Secondary",
                                ButtonVariant::Outline => "Outline",
                                ButtonVariant::Ghost => "Ghost",
                            }.to_string(),
                            "size" => match current_size {
                                ButtonSize::Small => "Small",
                                ButtonSize::Medium => "Medium",
                                ButtonSize::Large => "Large",
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
                                    if let Some(CanvasComponent::Button(latest_btn)) = canvas_state.get_component(&comp_id_field) {
                                        let updated_btn = update_button_prop(latest_btn, prop_name_closure.as_str(), PropValue::String(new_val));
                                        apply_update(comp_id_field, CanvasComponent::Button(updated_btn), prop_name_closure.clone());
                                    }
                                }
                            />
                        }.into_any()
                    },
                    PropType::Bool => {
                        let checked = match prop_name.as_str() {
                            "disabled" => current_disabled,
                            _ => false,
                        };
                        let prop_name_closure = prop_name.clone();
                        view! {
                            <BoolCheckbox
                                checked=checked
                                label=label_text
                                on_change=move |new_val| {
                                    if let Some(CanvasComponent::Button(latest_btn)) = canvas_state.get_component(&comp_id_field) {
                                        let updated_btn = update_button_prop(latest_btn, prop_name_closure.as_str(), PropValue::Boolean(new_val));
                                        apply_update(comp_id_field, CanvasComponent::Button(updated_btn), prop_name_closure.clone());
                                    }
                                }
                            />
                        }.into_any()
                    },
                    _ => ().into_any(),
                }
            }).collect::<Vec<_>>()}
        </div>

        <StyleEditor
            style=btn_style
            on_change=move |new_style| {
                if let Some(CanvasComponent::Button(mut latest_btn)) = canvas_state.get_component(&comp_id) {
                    latest_btn.style = new_style;
                    apply_update(comp_id, CanvasComponent::Button(latest_btn), "style".to_string());
                }
            }
        />

        <AnimationPropertyEditor
            _id=comp_id
            animation=btn_animation
            on_change=move |new_anim| {
                if let Some(CanvasComponent::Button(mut latest_btn)) = canvas_state.get_component(&comp_id) {
                    latest_btn.animation = new_anim;
                    apply_update(comp_id, CanvasComponent::Button(latest_btn), "animation".to_string());
                }
            }
        />
    }
}
