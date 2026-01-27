use leptos::prelude::*;
use crate::builder::property_inputs::StringInput;

#[component]
pub fn EventPropertyEditor(
    #[prop(into)] _id: crate::domain::ComponentId, // Unused for now, but kept for consistency
    #[prop(into)] event_name: String,
    #[prop(into)] handler_name: Option<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="property-group">
            <div class="group-title">"Events"</div>
            <StringInput
                value=handler_name.unwrap_or_default()
                label=event_name
                placeholder="e.g. handle_click".to_string()
                on_change=move |val| on_change.run(val)
            />
        </div>
    }
}
