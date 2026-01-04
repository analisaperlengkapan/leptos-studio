use leptos::prelude::*;
use leptos::html::Input;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::services::{GitBackend, CommitInfo, RepoStatus};
use crate::services::git_factory::get_git_backend;
use crate::state::{AppState, Notification};

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

        wasm_bindgen_futures::spawn_local(async move {
            match backend.restore_head().await {
                Ok(Some(project)) => {
                     app_state.apply_project(project);
                     app_state.ui.notify(Notification::success("Changes discarded. Reverted to HEAD.".to_string()));
                     // Refresh status
                     if let Ok(status) = backend.status(Some(&app_state.to_project())).await {
                         status_data.set(Some(status));
                     }
                }
                Ok(None) => {
                     app_state.ui.notify(Notification::warning("No commits to revert to.".to_string()));
                }
                Err(e) => {
                     app_state.ui.notify(Notification::error(e.user_message()));
                }
            }
        });
    };

    let do_push = move |_| {
        let backend = get_git_backend();

        wasm_bindgen_futures::spawn_local(async move {
            match backend.push().await {
                Ok(Some(json)) => {
                    // Trigger download using Blob (Best Practice)
                    let filename = "leptos_studio_repo.json";

                    let array = js_sys::Array::new();
                    array.push(&json.into());

                    let blob_options = web_sys::BlobPropertyBag::new();
                    blob_options.set_type("application/json");

                    match web_sys::Blob::new_with_str_sequence_and_options(&array, &blob_options) {
                        Ok(blob) => {
                            match web_sys::Url::create_object_url_with_blob(&blob) {
                                Ok(url) => {
                                    if let Some(window) = web_sys::window() {
                                        if let Some(document) = window.document() {
                                            if let Ok(a) = document.create_element("a") {
                                                let _ = a.set_attribute("href", &url);
                                                let _ = a.set_attribute("download", filename);

                                                if let Some(html_element) = a.dyn_ref::<web_sys::HtmlElement>() {
                                                    html_element.click();
                                                }

                                                // Revoke URL to free memory
                                                let _ = web_sys::Url::revoke_object_url(&url);

                                                app_state.ui.notify(Notification::success("Repository downloaded".to_string()));
                                            }
                                        }
                                    }
                                },
                                Err(e) => {
                                    let err_str = e.as_string().unwrap_or("Unknown URL error".to_string());
                                    app_state.ui.notify(Notification::error(format!("Failed to create download URL: {}", err_str)));
                                }
                            }
                        },
                        Err(e) => {
                            let err_str = e.as_string().unwrap_or("Unknown Blob error".to_string());
                            app_state.ui.notify(Notification::error(format!("Failed to create blob: {}", err_str)));
                        }
                    }
                }
                Ok(None) => {
                    app_state.ui.notify(Notification::success("Push successful".to_string()));
                }
                Err(e) => {
                    app_state.ui.notify(Notification::error(e.user_message()));
                }
            }
        });
    };

    let on_file_select = move |_ev: web_sys::Event| {
        let input = file_input_ref.get();
        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    match web_sys::FileReader::new() {
                        Ok(reader) => {
                            let reader_c = reader.clone();
                            let on_load = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                                if let Ok(result) = reader_c.result() {
                                    if let Some(text) = result.as_string() {
                                        let backend = get_git_backend();

                                        wasm_bindgen_futures::spawn_local(async move {
                                            match backend.clone_repo(&text).await {
                                                Ok(_) => {
                                                    app_state.ui.notify(Notification::success(
                                                        "Repository imported successfully".to_string(),
                                                    ));
                                                    // Refresh status and log
                                                    let project = app_state.to_project();
                                                    if let Ok(status) = backend.status(Some(&project)).await {
                                                        status_data.set(Some(status));
                                                    }
                                                    if let Ok(logs) = backend.log().await {
                                                        log_data.set(logs);
                                                    }
                                                }
                                                Err(e) => {
                                                    app_state.ui.notify(Notification::error(format!(
                                                        "Import failed: {}",
                                                        e.user_message()
                                                    )));
                                                }
                                            }
                                        });
                                    }
                                }
                            }) as Box<dyn FnMut(_)>);

                            reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
                            on_load.forget(); // Leak memory to keep closure alive until callback
                            if let Err(e) = reader.read_as_text(&file) {
                                let err_str = e.as_string().unwrap_or("Unknown File API error".to_string());
                                app_state.ui.notify(Notification::error(format!("Failed to read file: {}", err_str)));
                            }
                        },
                        Err(e) => {
                            let err_str = e.as_string().unwrap_or("Unknown FileReader error".to_string());
                            app_state.ui.notify(Notification::error(format!("Failed to create FileReader: {}", err_str)));
                        }
                    }
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

            <div class="git-status">
                {move || {
                    match status_data.get() {
                        Some(status) => view! {
                            <div class="status-box">
                                <p>"Branch: " <b>{status.branch}</b></p>
                                <p>"Commits: " {status.commit_count}</p>
                                <p class:text-red-500=move || status.has_changes class:text-green-500=move || !status.has_changes>
                                    {if status.has_changes { "Changes not staged" } else { "Working tree clean" }}
                                </p>
                            </div>
                        }.into_any(),
                        None => view! {
                            <p class:text-gray-500=true>
                                {if is_loading_status.get() { "Loading status..." } else { "No status loaded." }}
                            </p>
                        }.into_any()
                    }
                }}
            </div>

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

            <div class="git-log-container">
                <h4>"Commit History"</h4>
                <div class="git-log-list">
                    <For
                        each=move || log_data.get()
                        key=|commit| commit.id.clone()
                        children=move |commit| {
                            view! {
                                <div class="git-commit-item">
                                    <div class="commit-header">
                                        <span class="commit-id" title={commit.id.clone()}>{commit.id.chars().take(7).collect::<String>()}</span>
                                        <span class="commit-date">{commit.timestamp.format("%Y-%m-%d %H:%M").to_string()}</span>
                                    </div>
                                    <div class="commit-message">{commit.message}</div>
                                </div>
                            }
                        }
                    />
                    {move || if log_data.get().is_empty() {
                        view! {
                            <p class="no-commits">
                                {if is_loading_log.get() { "Loading commits..." } else { "No commits found or log not loaded." }}
                            </p>
                        }.into_any()
                    } else {
                        view! { <span /> }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}
