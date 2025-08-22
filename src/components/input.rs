use leptos::*;

#[component]
pub fn Input(placeholder: String) -> impl IntoView {
    view! { <input placeholder=placeholder /> }
}
