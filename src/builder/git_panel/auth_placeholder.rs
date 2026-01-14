use leptos::prelude::*;
use crate::state::app_state::AppState;

#[component]
pub fn GitAuthPlaceholder() -> impl IntoView {
    let app_state = AppState::expect_context();
    let show_auth = app_state.ui.show_auth_modal;

    view! {
        <div class="git-panel-content" style="text-align: center; padding: 2rem;">
            <p style="margin-bottom: 1rem; color: var(--color-gray-600);">
                "You need to login to access Git features."
            </p>
            <button class="btn btn-primary" on:click=move |_| show_auth.set(true)>
                "Login"
            </button>
        </div>
    }
}
