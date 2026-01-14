use leptos::prelude::*;
use crate::state::app_state::AppState;
use crate::state::app_state::Notification;

#[component]
pub fn AuthModal() -> impl IntoView {
    let app_state = AppState::expect_context();
    let auth_service = app_state.auth;
    let show = app_state.ui.show_auth_modal;

    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let on_submit = move |ev: leptos::web_sys::SubmitEvent| {
        ev.prevent_default();
        let u = username.get();
        let p = password.get();
        if auth_service.login(&u, &p) {
            app_state.ui.notify(Notification::success(format!("Welcome back, {}!", u)));
            show.set(false);
            username.set(String::new());
            password.set(String::new());
        } else {
            app_state.ui.notify(Notification::error("Invalid username".to_string()));
        }
    };

    let on_close = move |ev: leptos::web_sys::MouseEvent| {
        ev.prevent_default();
        show.set(false);
    };

    view! {
        {move || if show.get() {
            view! {
                <div class="modal-overlay" on:click=move |_| show.set(false)>
                    <div class="modal-content" on:click=move |e| e.stop_propagation()>
                        <div class="welcome-header">
                            <h2>"Login"</h2>
                        </div>
                        <form on:submit=on_submit>
                            <div class="welcome-body">
                                <div class="style-control">
                                    <label>"Username"</label>
                                    <input
                                        type="text"
                                        class="text-input"
                                        placeholder="Enter username"
                                        prop:value=move || username.get()
                                        on:input=move |ev| username.set(event_target_value(&ev))
                                        name="username"
                                        required
                                    />
                                </div>
                                <div class="style-control">
                                    <label>"Password"</label>
                                    <input
                                        type="password"
                                        class="text-input"
                                        placeholder="Enter password"
                                        prop:value=move || password.get()
                                        on:input=move |ev| password.set(event_target_value(&ev))
                                        name="password"
                                    />
                                </div>
                            </div>
                            <div class="welcome-footer" style="gap: 1rem; flex-direction: column;">
                                <button type="submit" class="btn btn-primary">"Login"</button>
                                <button type="button" class="btn btn-secondary" on:click=on_close>"Cancel"</button>
                            </div>
                        </form>
                    </div>
                </div>
            }.into_any()
        } else {
            view! {}.into_any()
        }}
    }
}
