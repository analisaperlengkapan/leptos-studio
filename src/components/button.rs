use leptos::*;

#[component]
pub fn Button(label: String) -> impl IntoView {
    view! { <button>{label}</button> }
}
