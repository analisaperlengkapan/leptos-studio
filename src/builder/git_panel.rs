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

    // For file input (import)
    let file_input_ref = NodeRef::<Input>::new();

    // Access global UI state for notifications
    let app_state = expect_context::<AppState>();

    // Initial load
    Effect::new(move |_| {
        let backend = get_git_backend();
        match backend.status() {
            Ok(status) => status_data.set(Some(status)),
            Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
        }
    });

    let load_status = move |_| {
        let backend = get_git_backend();
        match backend.status() {
            Ok(status) => status_data.set(Some(status)),
            Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
        }
    };

    let load_log = move |_| {
        let backend = get_git_backend();
        match backend.log() {
            Ok(logs) => log_data.set(logs),
            Err(e) => {
                app_state.ui.notify(Notification::error(e.user_message()));
            }
        }
    };

    let do_commit = move |_| {
        let message = commit_message.get().trim().to_string();
        if message.is_empty() {
            app_state.ui.notify(Notification::warning("Commit message cannot be empty".to_string()));
            return;
        }

        let backend = get_git_backend();
        let project = app_state.to_project();
        match backend.commit(&project, &message) {
            Ok(()) => {
                app_state.ui.notify(Notification::success(format!("Commit recorded: {}", message)));
                commit_message.set(String::new());
                // Refresh status automatically
                if let Ok(status) = backend.status() {
                    status_data.set(Some(status));
                }
                // If log is showing, refresh it too
                if !log_data.get().is_empty() {
                    if let Ok(logs) = backend.log() {
                        log_data.set(logs);
                    }
                }
            }
            Err(e) => {
                app_state.ui.notify(Notification::error(e.user_message()));
            }
        }
    };

    let do_push = move |_| {
        let backend = get_git_backend();
        match backend.push() {
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
                                let document = web_sys::window().unwrap().document().unwrap();
                                let a = document.create_element("a").unwrap();
                                let _ = a.set_attribute("href", &url);
                                let _ = a.set_attribute("download", filename);

                                if let Some(html_element) = a.dyn_ref::<web_sys::HtmlElement>() {
                                    html_element.click();
                                }

                                // Revoke URL to free memory
                                let _ = web_sys::Url::revoke_object_url(&url);

                                app_state.ui.notify(Notification::success("Repository downloaded".to_string()));
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
    };

    let on_file_select = move |_ev: web_sys::Event| {
        let input = file_input_ref.get();
        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let reader = web_sys::FileReader::new().unwrap();
                    let reader_c = reader.clone();

                    let on_load = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                        if let Ok(result) = reader_c.result() {
                            if let Some(text) = result.as_string() {
                                let backend = get_git_backend();
                                match backend.clone_repo(&text) {
                                    Ok(_) => {
                                        app_state.ui.notify(Notification::success(
                                            "Repository imported successfully".to_string(),
                                        ));
                                        // Refresh status and log
                                        if let Ok(status) = backend.status() {
                                            status_data.set(Some(status));
                                        }
                                        if let Ok(logs) = backend.log() {
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
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
                    on_load.forget(); // Leak memory to keep closure alive until callback
                    reader.read_as_text(&file).unwrap();
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
                <button on:click=load_status class="btn btn-secondary">"Status"</button>
                <button on:click=load_log class="btn btn-secondary">"Log"</button>
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
                        None => view! { <p>"Loading status..."</p> }.into_any()
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
                />

                <div class="git-actions">
                    <button on:click=do_commit class="btn btn-primary">"Commit"</button>
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
                        view! { <p class="no-commits">"No commits found or log not loaded."</p> }.into_any()
                    } else {
                        view! { <span /> }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}
