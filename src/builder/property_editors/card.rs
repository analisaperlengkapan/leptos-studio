use crate::builder::property_inputs::{BoolCheckbox, NumberInput};
use crate::domain::CardComponent;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn CardPropertyEditor(id: crate::domain::ComponentId, card: CardComponent) -> impl IntoView {
    let app_state = AppState::expect_context();
    let canvas_state = app_state.canvas;

    let update_padding = move |val: f64| {
        #[allow(clippy::collapsible_if)]
        if let Some(mut comp) = canvas_state.get_component(&id) {
            if let crate::domain::CanvasComponent::Card(ref mut c) = comp {
                c.padding = val as u32;
                canvas_state.update_component(&id, comp);
            }
        }
    };

    let update_radius = move |val: f64| {
        #[allow(clippy::collapsible_if)]
        if let Some(mut comp) = canvas_state.get_component(&id) {
            if let crate::domain::CanvasComponent::Card(ref mut c) = comp {
                c.border_radius = val as u32;
                canvas_state.update_component(&id, comp);
            }
        }
    };

    let update_shadow = move |val: bool| {
        #[allow(clippy::collapsible_if)]
        if let Some(mut comp) = canvas_state.get_component(&id) {
            if let crate::domain::CanvasComponent::Card(ref mut c) = comp {
                c.shadow = val;
                canvas_state.update_component(&id, comp);
            }
        }
    };

    let update_border = move |val: bool| {
        #[allow(clippy::collapsible_if)]
        if let Some(mut comp) = canvas_state.get_component(&id) {
            if let crate::domain::CanvasComponent::Card(ref mut c) = comp {
                c.border = val;
                canvas_state.update_component(&id, comp);
            }
        }
    };

    view! {
        <div class="property-group">
            <h4 class="group-title">"Card Properties"</h4>
            <NumberInput
                label="Padding (px)".to_string()
                value=card.padding as f64
                on_change=move |val| update_padding(val)
            />
            <NumberInput
                label="Border Radius (px)".to_string()
                value=card.border_radius as f64
                on_change=move |val| update_radius(val)
            />
            <BoolCheckbox
                label="Show Shadow".to_string()
                checked=card.shadow
                on_change=move |val| update_shadow(val)
            />
            <BoolCheckbox
                label="Show Border".to_string()
                checked=card.border
                on_change=move |val| update_border(val)
            />
        </div>
    }
}
