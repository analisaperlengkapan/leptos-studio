use leptos::prelude::*;

use crate::services::{GitBackend, NoopGitBackend};

#[component]
pub fn GitPanel() -> impl IntoView {
    // In browser-only mode we use the NoopGitBackend. Real implementations
    // (HTTP backend, Tauri, etc.) can later provide their own GitBackend.
    let status_text = RwSignal::new(
        "🔧 Git integration requires a backend server.\nThis is a client-side app. Git features will work when running with a server backend or a desktop wrapper.".to_string(),
    );
    let log_text = RwSignal::new(String::new());
    let commit_message = RwSignal::new(String::new());

    let load_status = move |_| {
        let backend = NoopGitBackend;
        match backend.status() {
            Ok(text) => status_text.set(text),
            Err(e) => status_text.set(e.user_message()),
        }
    };

    let load_log = move |_| {
        let backend = NoopGitBackend;
        match backend.log() {
            Ok(text) => log_text.set(text),
            Err(e) => log_text.set(e.user_message()),
        }
    };

    let do_commit = move |_| {
        let message = commit_message.get().trim().to_string();
        if message.is_empty() {
            status_text.set("⚠️ Commit message is empty".to_string());
            return;
        }

        let backend = NoopGitBackend;
        match backend.commit(&message) {
            Ok(()) => {
                status_text.set(format!("✅ Commit recorded: {}", message));
                commit_message.set(String::new());
            }
            Err(e) => status_text.set(format!("❌ {}", e.user_message())),
        }
    };

    let do_push = move |_| {
        let backend = NoopGitBackend;
        match backend.push() {
            Ok(()) => status_text.set("✅ Push completed (or simulated)".to_string()),
            Err(e) => status_text.set(format!("❌ {}", e.user_message())),
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
