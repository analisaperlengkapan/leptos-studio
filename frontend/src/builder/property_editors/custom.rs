use crate::builder::property_inputs::StringInput;
use crate::builder::styling_system::StyleEditor;
use crate::domain::{CanvasComponent, ComponentId};
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn CustomPropertyEditor(
    #[prop(into)] id: ComponentId,
    #[prop(into)] custom: crate::domain::CustomComponent,
) -> impl IntoView {
    let app_state = AppState::expect_context();
    let canvas_state = app_state.canvas;

    let update_name = move |new_name: String| {
         if let Some(CanvasComponent::Custom(mut c)) = canvas_state.get_component(&id) {
             c.name = new_name;
             canvas_state.update_component_with_snapshot(&id, CanvasComponent::Custom(c), "Update Custom Name");
         }
    };

    let update_template = move |new_template: String| {
         if let Some(CanvasComponent::Custom(mut c)) = canvas_state.get_component(&id) {
             c.template = new_template;
             canvas_state.update_component_with_snapshot(&id, CanvasComponent::Custom(c), "Update Custom Template");
         }
    };

    let update_style = move |new_style| {
         if let Some(CanvasComponent::Custom(mut c)) = canvas_state.get_component(&id) {
             c.style = new_style;
             canvas_state.update_component_with_snapshot(&id, CanvasComponent::Custom(c), "Update Custom Style");
         }
    };

    let current_name = custom.name.clone();
    let current_template = custom.template.clone();
    let current_style = custom.style.clone();

    view! {
        <div class="property-group">
            <div class="group-title">"Custom Component"</div>
            <StringInput
                value=current_name
                label="Name".to_string()
                on_change=update_name
            />
            <div class="property-field">
                <label>
                    {"Template"}
                    <textarea
                        class="property-input"
                        prop:value=current_template
                        on:input=move |ev| {
                            update_template(event_target_value(&ev));
                        }
                        style="width: 100%; min-height: 100px; font-family: monospace;"
                    />
                </label>
            </div>

            <StyleEditor
                style=current_style
                on_change=update_style
            />
        </div>
    }
}
