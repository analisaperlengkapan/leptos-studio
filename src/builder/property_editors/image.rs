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
    }
}
