use crate::services::CommitInfo;
use leptos::prelude::*;

#[component]
pub fn GitLogList(logs: Signal<Vec<CommitInfo>>, is_loading: Signal<bool>) -> impl IntoView {
    view! {
        <div class="git-log-container">
            <h4>"Commit History"</h4>
            <div class="git-log-list">
                <For
                    each=move || logs.get()
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
                {move || if logs.get().is_empty() {
                    view! {
                        <p class="no-commits">
                            {if is_loading.get() { "Loading commits..." } else { "No commits found or log not loaded." }}
                        </p>
                    }.into_any()
                } else {
                    view! { <span /> }.into_any()
                }}
            </div>
        </div>
    }
}
