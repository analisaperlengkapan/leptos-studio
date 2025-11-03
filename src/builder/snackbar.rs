use leptos::prelude::*;

#[component]
pub fn Snackbar(notification: RwSignal<Option<String>>) -> impl IntoView {
    view! {
        {move || notification.get().as_ref().map(|msg| view! {
            <div style="position:fixed;bottom:2rem;right:2rem;z-index:9999;background:#323232;color:white;padding:1em 2em;border-radius:8px;box-shadow:0 2px 8px rgba(0,0,0,0.2);font-size:1.1em;animation:fadein 0.2s;">
                {msg.clone()}
            </div>
        })}
    }
}
