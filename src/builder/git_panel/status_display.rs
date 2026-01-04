use leptos::prelude::*;
use crate::services::RepoStatus;

#[component]
pub fn GitStatusDisplay(
    status: Signal<Option<RepoStatus>>,
    is_loading: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="git-status">
            {move || {
                match status.get() {
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
                            {if is_loading.get() { "Loading status..." } else { "No status loaded." }}
                        </p>
                    }.into_any()
                }
            }}
        </div>
    }
}
