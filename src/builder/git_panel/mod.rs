use leptos::prelude::*;
use leptos::html::Input;

use crate::services::{RepoStatus, CommitInfo}; // We still need the types for the props if passed down
use crate::builder::hooks::use_git::{use_git, UseGitReturn}; // Import the hook

mod status_display;
mod log_list;
use status_display::GitStatusDisplay;
use log_list::GitLogList;

#[component]
#[allow(clippy::collapsible_if)]
pub fn GitPanel() -> impl IntoView {
    // Consume the hook
    let UseGitReturn {
        status,
        logs,
        is_loading_status,
        is_loading_log,
        is_committing,
        load_status,
        load_log,
        commit,
        discard,
        reset,
        push,
        import
    } = use_git();

    let commit_message = RwSignal::new(String::new());
    let file_input_ref = NodeRef::<Input>::new();

    // Handlers that bridge UI events to Hook actions
    let do_commit = move |_| {
        let msg = commit_message.get();
        if !msg.trim().is_empty() {
            commit.run(msg);
            commit_message.set(String::new());
        }
    };

    let on_file_select = move |_ev: web_sys::Event| {
        let input = file_input_ref.get();
        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                     import.run(file);
                }
            }
        }
    };

    let trigger_import = move |_| {
        if let Some(input) = file_input_ref.get() {
            input.click();
        }
    };

    // Load initial data is handled by the hook's Effect or we can trigger it manually here if needed.
    // The hook has an Effect that runs on mount/updates, so we don't need to duplicate it here.
    // But we might want to trigger `load_status` on mount explicitly?
    // The hook's effect waits 500ms.

    view! {
        <div class="git-panel-content">
            <div class="git-actions">
                <button
                    on:click=move |_| load_status.run(())
                    class="btn btn-secondary"
                    disabled=move || is_loading_status.get()
                >
                    {move || if is_loading_status.get() { "Checking..." } else { "Status" }}
                </button>
                <button
                    on:click=move |_| load_log.run(())
                    class="btn btn-secondary"
                    disabled=move || is_loading_log.get()
                >
                    {move || if is_loading_log.get() { "Loading..." } else { "Log" }}
                </button>
            </div>

            <GitStatusDisplay status=status.into() is_loading=is_loading_status.into() />

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
                    <button on:click=move |_| discard.run(()) class="btn btn-danger" title="Discard all uncommitted changes">"Discard Changes"</button>
                    <button on:click=move |_| reset.run(()) class="btn btn-danger" title="Reset repository (delete all history)">"Reset Repo"</button>
                    <button on:click=move |_| push.run(()) class="btn btn-secondary" title="Download Repository JSON">"Push (Download)"</button>
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

            <GitLogList logs=logs.into() is_loading=is_loading_log.into() />
        </div>
    }
}
