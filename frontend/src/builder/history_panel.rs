use crate::state::app_state::AppState;
use leptos::prelude::*;

#[component]
pub fn HistoryPanel() -> impl IntoView {
    let app_state = AppState::expect_context();
    let history = app_state.canvas.history;

    // Helper to format timestamp
    let format_time = |timestamp: f64| -> String {
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp));
        format!(
            "{:02}:{:02}:{:02}",
            date.get_hours(),
            date.get_minutes(),
            date.get_seconds()
        )
    };

    // Memoize the history list to avoid issues inside view macro
    let history_list = Memo::new(move |_| {
        let stack = history.get().get_undo_stack();
        // We want to show latest at top, but preserve original index for restoration
        stack
            .into_iter()
            .enumerate()
            .rev() // Reverse iterator to show latest first
            .collect::<Vec<_>>()
    });

    let restore = move |index: usize| {
        // Index comes from the enumeration of the original stack order
        if let Some(snapshot) = history.write().restore_to_index(index) {
            app_state.canvas.apply_snapshot(&snapshot);
            app_state
                .ui
                .notify(crate::state::app_state::Notification::info(
                    "Restored history state".to_string(),
                ));
        }
    };

    view! {
        <div class="history-panel">
            <h4 class="panel-title">"History"</h4>
            <div class="history-list">
                <For
                    each=move || history_list.get()
                    key=|(i, _)| *i
                    children=move |(i, snapshot)| {
                        view! {
                            <div
                                class="history-item clickable"
                                on:click=move |_| restore(i)
                                title="Click to restore this state"
                            >
                                <span class="history-time">{format_time(snapshot.timestamp)}</span>
                                <span class="history-desc">{snapshot.description}</span>
                            </div>
                        }
                    }
                />
            </div>
            {move || {
                let h = history.get();
                if !h.can_undo() && !h.can_redo() {
                    view! { <div class="history-empty">"No history yet"</div> }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
