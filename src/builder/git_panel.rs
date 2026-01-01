use leptos::prelude::*;

use crate::services::{GitBackend, LocalStorageGitBackend};
use crate::state::{AppState, Notification};

#[component]
pub fn GitPanel() -> impl IntoView {
    // Uses LocalStorageGitBackend for persistence in the browser.
    let status_text = RwSignal::new("Ready to commit.".to_string());
    let log_text = RwSignal::new(String::new());
    let commit_message = RwSignal::new(String::new());

    // Access global UI state for notifications
    let app_state = expect_context::<AppState>();

    // Initialize with status
    Effect::new(move |_| {
        let backend = LocalStorageGitBackend::new();
        match backend.status() {
            Ok(text) => status_text.set(text),
            Err(e) => status_text.set(e.user_message()),
        }
    });

    let load_status = move |_| {
        let backend = LocalStorageGitBackend::new();
        match backend.status() {
            Ok(text) => status_text.set(text),
            Err(e) => status_text.set(e.user_message()),
        }
    };

    let load_log = move |_| {
        let backend = LocalStorageGitBackend::new();
        match backend.log() {
            Ok(text) => log_text.set(text),
            Err(e) => {
                app_state.ui.notify(Notification::error(e.user_message()));
                log_text.set(e.user_message());
            }
        }
    };

    let do_commit = move |_| {
        let message = commit_message.get().trim().to_string();
        if message.is_empty() {
            app_state.ui.notify(Notification::warning("Commit message cannot be empty".to_string()));
            return;
        }

        let backend = LocalStorageGitBackend::new();
        match backend.commit(&message) {
            Ok(()) => {
                app_state.ui.notify(Notification::success(format!("Commit recorded: {}", message)));
                commit_message.set(String::new());
                // Refresh status automatically
                if let Ok(text) = backend.status() {
                    status_text.set(text);
                }
            }
            Err(e) => {
                app_state.ui.notify(Notification::error(e.user_message()));
            }
        }
    };

    let do_push = move |_| {
        let backend = LocalStorageGitBackend::new();
        match backend.push() {
            Ok(()) => {
                app_state.ui.notify(Notification::success("Repo synced (local storage)".to_string()));
                status_text.set("✅ Repo synced (local storage)".to_string());
            }
            Err(e) => {
                app_state.ui.notify(Notification::error(e.user_message()));
            }
        }
    };

    view! {
        <div class="git-panel-content">
            <div class="git-actions">
                <button on:click=load_status class="btn btn-secondary">"Status"</button>
                <button on:click=load_log class="btn btn-secondary">"Log"</button>
            </div>

            <div class="git-status">
                {move || status_text.get()}
            </div>

            <input
                class="git-commit-input"
                type="text"
                placeholder="Commit message"
                prop:value=move || commit_message.get()
                on:input=move |ev| commit_message.set(event_target_value(&ev))
            />

            <div class="git-actions">
                <button on:click=do_commit class="btn btn-secondary">"Commit"</button>
                <button on:click=do_push class="btn btn-secondary">"Push"</button>
            </div>

            <div class="git-log">
                {move || {
                    let log = log_text.get();
                    if log.is_empty() {
                        "No log loaded yet".to_string()
                    } else {
                        log
                    }
                }}
            </div>
        </div>
    }
}
