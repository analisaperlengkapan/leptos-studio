use crate::services::git_factory::get_git_backend;
use crate::services::{CommitInfo, GitBackend, RepoStatus};
use crate::state::{AppState, Notification};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct UseGitReturn {
    pub status: Signal<Option<RepoStatus>>,
    pub logs: Signal<Vec<CommitInfo>>,
    pub is_loading_status: Signal<bool>,
    pub is_loading_log: Signal<bool>,
    pub is_committing: Signal<bool>,
    pub load_status: Callback<()>,
    pub load_log: Callback<()>,
    pub commit: Callback<String>,
    pub discard: Callback<()>,
    pub reset: Callback<()>,
    pub push: Callback<()>,
    pub import: Callback<web_sys::File>,
}

/// A hook to encapsulate Git operations
pub fn use_git() -> UseGitReturn {
    let app_state = expect_context::<AppState>();

    let status_data = RwSignal::new(Option::<RepoStatus>::None);
    let log_data = RwSignal::new(Vec::<CommitInfo>::new());

    let is_loading_status = RwSignal::new(false);
    let is_loading_log = RwSignal::new(false);
    let is_committing = RwSignal::new(false);

    // Initial status check with debounce support (logic ported from GitPanel)
    let debounce_token = StoredValue::new(0usize);

    Effect::new(move |_| {
        // Track dependencies via last_modified signal
        let _ = app_state.last_modified.get();

        // Increment token
        debounce_token.update_value(|t| *t = t.wrapping_add(1));
        let current_token = debounce_token.get_value();

        // Spawn async wait
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(500).await;

            if debounce_token.get_value() == current_token {
                let backend = get_git_backend();
                let project = app_state.to_project();

                // Silent update (no spinner)
                if let Ok(status) = backend.status(Some(&project)).await {
                    status_data.set(Some(status));
                }
            }
        });
    });

    let load_status_fn = move || {
        is_loading_status.set(true);
        let backend = get_git_backend();
        let project = app_state.to_project();

        wasm_bindgen_futures::spawn_local(async move {
            match backend.status(Some(&project)).await {
                Ok(status) => status_data.set(Some(status)),
                Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
            }
            is_loading_status.set(false);
        });
    };

    let load_log_fn = move || {
        is_loading_log.set(true);
        let backend = get_git_backend();
        wasm_bindgen_futures::spawn_local(async move {
            match backend.log().await {
                Ok(logs) => log_data.set(logs),
                Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
            }
            is_loading_log.set(false);
        });
    };

    let commit_fn = move |message: String| {
        let message = message.trim().to_string();
        if message.is_empty() {
            app_state.ui.notify(Notification::warning(
                "Commit message cannot be empty".to_string(),
            ));
            return;
        }

        is_committing.set(true);
        let backend = get_git_backend();
        let project = app_state.to_project();

        wasm_bindgen_futures::spawn_local(async move {
            match backend.commit(&project, &message).await {
                Ok(()) => {
                    app_state.ui.notify(Notification::success(format!(
                        "Commit recorded: {}",
                        message
                    )));

                    // Refresh status
                    if let Ok(status) = backend.status(Some(&project)).await {
                        status_data.set(Some(status));
                    }
                    // Refresh logs if they are already loaded/visible
                    // (Simple heuristic: if logs vector is not empty or we just want to ensure consistency)
                    if !log_data.get().is_empty() {
                        if let Ok(logs) = backend.log().await {
                            log_data.set(logs);
                        }
                    }
                }
                Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
            }
            is_committing.set(false);
        });
    };

    let discard_fn = move || {
        let backend = get_git_backend();
        wasm_bindgen_futures::spawn_local(async move {
            match backend.restore_head().await {
                Ok(Some(project)) => {
                    app_state.apply_project(project);
                    app_state.ui.notify(Notification::success(
                        "Changes discarded. Reverted to HEAD.".to_string(),
                    ));
                    // Refresh status
                    if let Ok(status) = backend.status(Some(&app_state.to_project())).await {
                        status_data.set(Some(status));
                    }
                }
                Ok(None) => app_state.ui.notify(Notification::warning(
                    "No commits to revert to.".to_string(),
                )),
                Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
            }
        });
    };

    let reset_fn = move || {
        let backend = get_git_backend();
        wasm_bindgen_futures::spawn_local(async move {
            match backend.reset().await {
                Ok(()) => {
                    app_state.ui.notify(Notification::success(
                        "Repository reset successfully.".to_string(),
                    ));
                    if let Ok(status) = backend.status(Some(&app_state.to_project())).await {
                        status_data.set(Some(status));
                    }
                    log_data.set(Vec::new()); // Clear logs
                }
                Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
            }
        });
    };

    let push_fn = move || {
        let backend = get_git_backend();
        wasm_bindgen_futures::spawn_local(async move {
            match backend.push().await {
                Ok(Some(json)) => {
                    let filename = "leptos_studio_repo.json";
                    match crate::utils::file::download_file(&json, filename, "application/json") {
                        Ok(_) => app_state
                            .ui
                            .notify(Notification::success("Repository downloaded".to_string())),
                        Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
                    }
                }
                Ok(None) => app_state
                    .ui
                    .notify(Notification::success("Push successful".to_string())),
                Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
            }
        });
    };

    let import_fn = move |file: web_sys::File| {
        let backend = get_git_backend();
        wasm_bindgen_futures::spawn_local(async move {
            match crate::utils::file::read_file_as_text(&file).await {
                Ok(text) => {
                    match backend.clone_repo(&text).await {
                        Ok(_) => {
                            app_state.ui.notify(Notification::success(
                                "Repository imported successfully".to_string(),
                            ));
                            // Refresh
                            let project = app_state.to_project();
                            if let Ok(status) = backend.status(Some(&project)).await {
                                status_data.set(Some(status));
                            }
                            if let Ok(logs) = backend.log().await {
                                log_data.set(logs);
                            }
                        }
                        Err(e) => app_state.ui.notify(Notification::error(format!(
                            "Import failed: {}",
                            e.user_message()
                        ))),
                    }
                }
                Err(e) => app_state.ui.notify(Notification::error(format!(
                    "Failed to read file: {}",
                    e.user_message()
                ))),
            }
        });
    };

    UseGitReturn {
        status: status_data.into(),
        logs: log_data.into(),
        is_loading_status: is_loading_status.into(),
        is_loading_log: is_loading_log.into(),
        is_committing: is_committing.into(),
        load_status: Callback::new(move |_| load_status_fn()),
        load_log: Callback::new(move |_| load_log_fn()),
        commit: Callback::new(move |msg| commit_fn(msg)),
        discard: Callback::new(move |_| discard_fn()),
        reset: Callback::new(move |_| reset_fn()),
        push: Callback::new(move |_| push_fn()),
        import: Callback::new(move |file| import_fn(file)),
    }
}
