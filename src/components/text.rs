use leptos::prelude::*;

#[component]
pub fn Text(content: String) -> impl IntoView {
    view! { <span>{content}</span> }
}
