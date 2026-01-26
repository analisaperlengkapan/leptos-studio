use super::{AnimationPropertyEditor, EventPropertyEditor};
use crate::builder::property_inputs::{BoolCheckbox, StringInput};
use crate::domain::{SelectComponent, ComponentId};
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn SelectPropertyEditor(
    id: ComponentId,
    select: SelectComponent,
) -> impl IntoView {
    let app_state = AppState::expect_context();
    let canvas_state = app_state.canvas;

    let update_options = move |val: String| {
        canvas_state.update_component(&id, |c| {
            if let crate::domain::CanvasComponent::Select(sel) = c {
                sel.options = val;
            }
        });
    };

    let update_placeholder = move |val: String| {
        canvas_state.update_component(&id, |c| {
            if let crate::domain::CanvasComponent::Select(sel) = c {
                sel.placeholder = val;
            }
        });
    };

    let update_disabled = move |val: bool| {
        canvas_state.update_component(&id, |c| {
            if let crate::domain::CanvasComponent::Select(sel) = c {
                sel.disabled = val;
            }
        });
    };

    let id_clone = id;
    let select_clone = select.clone();
    let id_clone2 = id;
    let select_clone2 = select_clone.clone();

    view! {
        <div class="property-group">
            <h4>"Select Properties"</h4>
            <StringInput
                label="Options (comma separated)".to_string()
                value=select.options
                on_change=update_options
            />
            <StringInput
                label="Placeholder".to_string()
                value=select.placeholder
                on_change=update_placeholder
            />
            <BoolCheckbox
                label="Disabled".to_string()
                checked=select.disabled
                on_change=update_disabled
            />
        </div>

        <EventPropertyEditor
            _id=id_clone2
            event_name="On Change".to_string()
            handler_name=select_clone2.on_change.clone()
            on_change=Callback::new(move |val: String| {
                canvas_state.update_component(&id_clone2, |c| {
                    if let crate::domain::CanvasComponent::Select(sel) = c {
                        sel.on_change = if val.is_empty() { None } else { Some(val) };
                    }
                });
            })
        />

        <AnimationPropertyEditor
            _id=id_clone
            animation=select_clone.animation
            on_change=move |new_anim| {
                canvas_state.update_component(&id_clone, |c| {
                    if let crate::domain::CanvasComponent::Select(sel) = c {
                        sel.animation = new_anim;
                    }
                });
            }
        />
    }
}
