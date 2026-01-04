use leptos::prelude::*;
use crate::builder::property_inputs::{EnumSelect, NumberInput};
use crate::domain::{CanvasComponent, ComponentId, PropValue};
use crate::services::update_container_prop;
use crate::state::AppState;
use crate::builder::component_library::PropType;

#[component]
pub fn ContainerPropertyEditor(
    id: ComponentId,
    #[prop(into)]
    container: crate::domain::ContainerComponent,
) -> impl IntoView {
    let app_state = AppState::use_context();
    let ui_state = app_state.ui;
    let canvas_state = app_state.canvas;

    let container_schema = ui_state
        .component_library
        .get()
        .into_iter()
        .find(|c| c.kind == "Container")
        .and_then(|c| c.props_schema)
        .unwrap_or_default();

    let comp_id = id.clone();

    let apply_update = move |id: ComponentId, updated: CanvasComponent| {
        if let Err(e) = updated.validate() {
            ui_state.notify(crate::state::Notification::error(e.user_message()));
        } else {
            canvas_state.update_component(&id, updated);
        }
    };

    view! {
            <div class="property-group">
                <div class="group-title">"Layout"</div>
                {container_schema.into_iter().map(|prop| {
                let prop_name = prop.name.clone();
                let prop_type = prop.prop_type.clone();
                let label_text = prop.name.clone();
                let comp_id_field = comp_id.clone();
                let container_for_field = container.clone();
                let apply_update = apply_update.clone();

                match prop_type {
                    PropType::Enum { options } => {
                            let value = match prop_name.as_str() {
                            "layout" => match &container_for_field.layout {
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
                                    apply_update(comp_id_field.clone(), CanvasComponent::Container(updated_container));
                                }
                            />
                        }.into_any()
                    },
                    PropType::Number => {
                        let value = match prop_name.as_str() {
                            "gap" => container_for_field.gap as f64,
                            "padding_top" => container_for_field.padding.top as f64,
                            "padding_right" => container_for_field.padding.right as f64,
                            "padding_bottom" => container_for_field.padding.bottom as f64,
                            "padding_left" => container_for_field.padding.left as f64,
                            _ => 0.0,
                        };
                            view! {
                            <NumberInput
                                value=value
                                label=label_text
                                on_change=move |new_val| {
                                    let updated_container = update_container_prop(container_for_field.clone(), prop_name.as_str(), PropValue::Number(new_val));
                                    apply_update(comp_id_field.clone(), CanvasComponent::Container(updated_container));
                                }
                            />
                        }.into_any()
                    },
                    _ => ().into_any()
                }
                }).collect::<Vec<_>>()}
            </div>
    }
}
