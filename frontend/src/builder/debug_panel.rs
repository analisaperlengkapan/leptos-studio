use crate::builder::component_library::ComponentRegistry;
use crate::services::analytics_service::AnalyticsService;
use crate::state::app_state::{AppState, Notification};
use leptos::prelude::*;

#[component]
pub fn DebugPanel() -> impl IntoView {
    // Get AppState from context (no props!)
    let app_state = AppState::expect_context();
    let analytics = AnalyticsService::use_context();
    let sync_loading = RwSignal::new(false);

    let sync_analytics = move |_| {
        sync_loading.set(true);
        leptos::task::spawn_local(async move {
            match analytics.flush_to_backend().await {
                Ok(_) => {
                    app_state
                        .ui
                        .notify(Notification::success("Analytics synced".to_string()));
                }
                Err(e) => {
                    app_state
                        .ui
                        .notify(Notification::error(format!("Sync failed: {}", e)));
                }
            }
            sync_loading.set(false);
        });
    };

    // Format timestamp
    let last_sync_display = move || {
        let ts = analytics.last_synced();
        if ts == 0.0 {
            "Never".to_string()
        } else {
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ts));
            format!(
                "{}:{:02}:{:02}",
                date.get_hours(),
                date.get_minutes(),
                date.get_seconds()
            )
        }
    };

    view! {
        <div class="debug-panel">
            <b>"🐛 Debug Panel"</b>
            <div><b>"Components:"</b> {move || format!("{}", app_state.canvas.components.get().len())}</div>
            <div><b>"Custom Components:"</b> {move || {
                let lib = app_state.ui.component_library.get();
                let count = ComponentRegistry::custom_from_library(&lib).len();
                format!("{}", count)
            }}</div>
            <div><b>"Can Undo:"</b> {move || format!("{}", app_state.canvas.history.with(|h| h.can_undo()))}</div>
            <div><b>"Can Redo:"</b> {move || format!("{}", app_state.canvas.history.with(|h| h.can_redo()))}</div>
            <div><b>"Render count:"</b> {move || app_state.ui.render_count.get()}</div>
            <div><b>"Render time:"</b> {move || format!("{:.2} ms", app_state.ui.render_time.get())}</div>

            <div class="debug-separator" style="margin: 0.5rem 0; border-top: 1px solid #ccc;"></div>

            <div><b>"Analytics:"</b></div>
            <div style="font-size: 0.8em; margin-bottom: 4px;">
                "Recorded: " {move || analytics.metrics_summary().total_metrics}
            </div>
            <div style="font-size: 0.8em; margin-bottom: 4px;">
                "Last Sync: " {last_sync_display}
            </div>
            <button
                class="btn btn-sm btn-secondary"
                style="width: 100%; margin-top: 4px;"
                disabled=move || sync_loading.get()
                on:click=sync_analytics
            >
                {move || if sync_loading.get() { "Syncing..." } else { "Sync Stats" }}
            </button>

            <details class="debug-details" style="margin-top: 1rem;"><summary>"📦 Dump State"</summary>
                <pre class="debug-dump">
                    {move || format!("{:#?}", app_state.canvas.components.get())}
                </pre>
            </details>
        </div>
    }
}
