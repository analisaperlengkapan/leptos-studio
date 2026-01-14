use super::{AnimationPropertyEditor, EventPropertyEditor};
use crate::builder::property_inputs::StringInput;
use crate::domain::ImageComponent;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn ImagePropertyEditor(id: crate::domain::ComponentId, image: ImageComponent) -> impl IntoView {
    let app_state = AppState::expect_context();
    let canvas_state = app_state.canvas;

    let update_src = move |new_src: String| {
        canvas_state.update_component(&id, |c| {
            if let crate::domain::CanvasComponent::Image(img) = c {
                img.src = new_src;
            }
        });
    };

    let update_alt = move |new_alt: String| {
        canvas_state.update_component(&id, |c| {
            if let crate::domain::CanvasComponent::Image(img) = c {
                img.alt = new_alt;
            }
        });
    };

    let update_width = move |new_width: String| {
        canvas_state.update_component(&id, |c| {
            if let crate::domain::CanvasComponent::Image(img) = c {
                if new_width.is_empty() {
                    img.width = None;
                } else {
                    img.width = Some(new_width);
                }
            }
        });
    };

    let update_height = move |new_height: String| {
        canvas_state.update_component(&id, |c| {
            if let crate::domain::CanvasComponent::Image(img) = c {
                if new_height.is_empty() {
                    img.height = None;
                } else {
                    img.height = Some(new_height);
                }
            }
        });
    };

    let id_clone = id;
    let img_clone = image.clone();
    let id_clone2 = id;
    let img_clone2 = img_clone.clone();

    view! {
        <div class="property-group">
            <h4 class="group-title">"Image Properties"</h4>
            <StringInput
                label="Source URL".to_string()
                value=image.src.clone()
                on_change=move |val: String| update_src(val)
            />
            <StringInput
                label="Alt Text".to_string()
                value=image.alt.clone()
                on_change=move |val: String| update_alt(val)
            />
            <StringInput
                label="Width (e.g. 100%, 200px)".to_string()
                value=image.width.clone().unwrap_or_default()
                on_change=move |val: String| update_width(val)
            />
            <StringInput
                label="Height (e.g. auto, 150px)".to_string()
                value=image.height.clone().unwrap_or_default()
                on_change=move |val: String| update_height(val)
            />
        </div>

        <EventPropertyEditor
            _id=id_clone2
            event_name="On Click".to_string()
            handler_name=img_clone2.on_click.clone()
            on_change=Callback::new(move |val: String| {
                canvas_state.update_component(&id_clone2, |c| {
                    if let crate::domain::CanvasComponent::Image(img) = c {
                        img.on_click = if val.is_empty() { None } else { Some(val) };
                    }
                });
            })
        />

        <AnimationPropertyEditor
            _id=id_clone
            animation=img_clone.animation
            on_change=move |new_anim| {
                canvas_state.update_component(&id_clone, |c| {
                    if let crate::domain::CanvasComponent::Image(img) = c {
                        img.animation = new_anim;
                    }
                });
            }
        />
    }
}
