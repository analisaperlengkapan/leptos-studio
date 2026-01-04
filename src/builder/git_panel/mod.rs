use leptos::prelude::*;
use leptos::html::Input;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::services::{GitBackend, CommitInfo, RepoStatus};
use crate::services::git_factory::get_git_backend;
use crate::state::{AppState, Notification};
use crate::utils::spawn_result_task;

mod status_display;
mod log_list;
use status_display::GitStatusDisplay;
use log_list::GitLogList;

#[component]
#[allow(clippy::collapsible_if)]
pub fn GitPanel() -> impl IntoView {
    // Uses get_git_backend factory.
    let status_data = RwSignal::new(Option::<RepoStatus>::None);
    let log_data = RwSignal::new(Vec::<CommitInfo>::new());
    let commit_message = RwSignal::new(String::new());

    // Loading states for better UX
    let is_loading_status = RwSignal::new(false);
    let is_loading_log = RwSignal::new(false);
    let is_committing = RwSignal::new(false);

    // For file input (import)
    let file_input_ref = NodeRef::<Input>::new();

    // Access global UI state for notifications
    let app_state = expect_context::<AppState>();

    // Debounced status check using generation token pattern
    // This avoids storing non-Send/Sync types like Timeout/Closure in StoredValue
    let debounce_token = StoredValue::new(0usize);

    Effect::new(move |_| {
        // Track dependencies via last_modified signal (Optimization)
        let _ = app_state.last_modified.get();

        // Increment token
        debounce_token.update_value(|t| *t = t.wrapping_add(1));
        let current_token = debounce_token.get_value();

        // Spawn async wait
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(500).await;

            // Check if we are still the latest generation
            if debounce_token.get_value() == current_token {
                let backend = get_git_backend();
                let project = app_state.to_project();

                // Don't show global spinner for background status check,
                // but we could set a local one if we wanted.
                match backend.status(Some(&project)).await {
                    Ok(status) => status_data.set(Some(status)),
                    Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
                }
            }
        });
    });

    let load_status = move |_| {
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

    let load_log = move |_| {
        is_loading_log.set(true);
        let backend = get_git_backend();
        wasm_bindgen_futures::spawn_local(async move {
            match backend.log().await {
                Ok(logs) => log_data.set(logs),
                Err(e) => {
                    app_state.ui.notify(Notification::error(e.user_message()));
                }
            }
            is_loading_log.set(false);
        });
    };

    let do_commit = move |_| {
        let message = commit_message.get().trim().to_string();
        if message.is_empty() {
            app_state.ui.notify(Notification::warning("Commit message cannot be empty".to_string()));
            return;
        }

        is_committing.set(true);
        let backend = get_git_backend();
        let project = app_state.to_project();

        wasm_bindgen_futures::spawn_local(async move {
            match backend.commit(&project, &message).await {
                Ok(()) => {
                    app_state.ui.notify(Notification::success(format!("Commit recorded: {}", message)));
                    commit_message.set(String::new());
                    // Refresh status automatically
                    if let Ok(status) = backend.status(Some(&project)).await {
                        status_data.set(Some(status));
                    }
                    // If log is showing, refresh it too
                    if !log_data.get().is_empty() {
                        if let Ok(logs) = backend.log().await {
                            log_data.set(logs);
                        }
                    }
                }
                Err(e) => {
                    app_state.ui.notify(Notification::error(e.user_message()));
                }
            }
            is_committing.set(false);
        });
    };

    let do_discard = move |_| {
        let backend = get_git_backend();

        spawn_result_task(
            async move {
                backend.restore_head().await
            },
            app_state,
            move |opt_project| {
                if let Some(project) = opt_project {
                     app_state.apply_project(project);
                     app_state.ui.notify(Notification::success("Changes discarded. Reverted to HEAD.".to_string()));

                     // Refresh status (spawn another task or nested? Nested is fine here since it's fire-and-forget)
                     wasm_bindgen_futures::spawn_local(async move {
                         let backend = get_git_backend();
                         if let Ok(status) = backend.status(Some(&app_state.to_project())).await {
                             status_data.set(Some(status));
                         }
                     });
                } else {
                     app_state.ui.notify(Notification::warning("No commits to revert to.".to_string()));
                }
            }
        );
    };

    let do_reset_repo = move |_| {
        let backend = get_git_backend();

        spawn_result_task(
            async move {
                backend.reset().await
            },
            app_state,
            move |_| {
                 app_state.ui.notify(Notification::success("Repository reset successfully.".to_string()));

                 // Refresh status and log
                 wasm_bindgen_futures::spawn_local(async move {
                     let backend = get_git_backend();
                     if let Ok(status) = backend.status(Some(&app_state.to_project())).await {
                         status_data.set(Some(status));
                     }
                     if let Ok(logs) = backend.log().await {
                         log_data.set(logs);
                     }
                 });
            }
        );
    };

    let do_push = move |_| {
        let backend = get_git_backend();

        spawn_result_task(
            async move {
                backend.push().await
            },
            app_state,
            move |res| {
                if let Some(json) = res {
                    let filename = "leptos_studio_repo.json";
                    match crate::utils::file::download_file(&json, filename, "application/json") {
                        Ok(_) => {
                            app_state.ui.notify(Notification::success("Repository downloaded".to_string()));
                        }
                        Err(e) => {
                             app_state.ui.notify(Notification::error(e.user_message()));
                        }
                    }
                } else {
                    app_state.ui.notify(Notification::success("Push successful".to_string()));
                }
            }
        );
    };

    let on_file_select = move |_ev: web_sys::Event| {
        let input = file_input_ref.get();
        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    spawn_result_task(
                        async move {
                            crate::utils::file::read_file_as_text(file).await
                        },
                        app_state,
                        move |text| {
                            spawn_result_task(
                                async move {
                                    get_git_backend().clone_repo(&text).await
                                },
                                app_state,
                                move |_| {
                                    app_state.ui.notify(Notification::success(
                                        "Repository imported successfully".to_string(),
                                    ));
                                    // Refresh status and log
                                    let project = app_state.to_project();
                                    let backend = get_git_backend();
                                    wasm_bindgen_futures::spawn_local(async move {
                                        if let Ok(status) = backend.status(Some(&project)).await {
                                            status_data.set(Some(status));
                                        }
                                        if let Ok(logs) = backend.log().await {
                                            log_data.set(logs);
                                        }
                                    });
                                }
                            );
                        }
                    );
                }
            }
        }
    };

    let trigger_import = move |_| {
        if let Some(input) = file_input_ref.get() {
            input.click();
        }
    };

    view! {
        <div class="git-panel-content">
            <div class="git-actions">
                <button
                    on:click=load_status
                    class="btn btn-secondary"
                    disabled=move || is_loading_status.get()
                >
                    {move || if is_loading_status.get() { "Checking..." } else { "Status" }}
                </button>
                <button
                    on:click=load_log
                    class="btn btn-secondary"
                    disabled=move || is_loading_log.get()
                >
                    {move || if is_loading_log.get() { "Loading..." } else { "Log" }}
                </button>
            </div>

            <GitStatusDisplay status=status_data.into() is_loading=is_loading_status.into() />

            <div class="git-commit-area">
                <input
                    class="git-commit-input"
                    type="text"
                    placeholder="Commit message"
                    prop:value=move || commit_message.get()
                    on:input=move |ev| commit_message.set(event_target_value(&ev))
                    prop:disabled=move || is_committing.get()
                />

                <div class="git-actions">
                    <button
                        on:click=do_commit
                        class="btn btn-primary"
                        disabled=move || is_committing.get()
                    >
                        {move || if is_committing.get() { "Committing..." } else { "Commit" }}
                    </button>
                    <button on:click=do_discard class="btn btn-danger" title="Discard all uncommitted changes">"Discard Changes"</button>
                    <button on:click=do_reset_repo class="btn btn-danger" title="Reset repository (delete all history)">"Reset Repo"</button>
                    <button on:click=do_push class="btn btn-secondary" title="Download Repository JSON">"Push (Download)"</button>
                    <button on:click=trigger_import class="btn btn-secondary" title="Import Repository JSON">"Clone (Import)"</button>
                </div>
            </div>

            // Hidden file input for import
            <input
                type="file"
                node_ref=file_input_ref
                style="display:none"
                accept=".json"
                on:change=on_file_select
            />

            <GitLogList logs=log_data.into() is_loading=is_loading_log.into() />
        </div>
    }
}
