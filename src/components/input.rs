use leptos::prelude::*;

#[component]
pub fn Input(placeholder: String) -> impl IntoView {
    view! { <input placeholder=placeholder /> }
}
