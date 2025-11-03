use leptos::prelude::*;

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="container">{children()}</div> }
}
