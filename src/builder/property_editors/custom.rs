use leptos::prelude::*;
use crate::builder::property_inputs::StringInput;
use crate::domain::{CanvasComponent, ComponentId};
use crate::state::AppState;

#[component]
pub fn CustomPropertyEditor(
    #[prop(into)]
    _id: ComponentId,
    #[prop(into)]
    custom: crate::domain::CustomComponent,
) -> impl IntoView {
    let app_state = AppState::use_context();
    let ui_state = app_state.ui;
    let canvas_state = app_state.canvas;

    let apply_update = move |id: ComponentId, updated: CanvasComponent, prop_name: String| {
        if let Err(e) = updated.validate() {
            ui_state.notify(crate::state::Notification::error(e.user_message()));
        } else {
            canvas_state.update_component_with_snapshot(&id, updated, &format!("Update Custom {}", prop_name));
        }
    };

    let name_value = custom.name.clone();
    let template_value = custom.template.clone();
    let comp_id = custom.id;
    let custom_for_name = custom.clone();
    let custom_for_template = custom.clone();
    let comp_id_for_name = comp_id;
    let comp_id_for_template = comp_id;
    let apply_update_name = apply_update;
    let apply_update_template = apply_update;

    view! {
        <div class="property-group">
            <div class="group-title">"Custom Component"</div>
            <StringInput
                value=name_value
                label="Name".to_string()
                on_change=move |new_name| {
                    let mut updated_custom = custom_for_name.clone();
                    updated_custom.name = new_name;
                    apply_update_name(comp_id_for_name, CanvasComponent::Custom(updated_custom), "Name".to_string());
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
                            apply_update_template(comp_id_for_template, CanvasComponent::Custom(updated_custom), "Template".to_string());
                        }
                    />
                </label>
            </div>
        </div>
    }
}
