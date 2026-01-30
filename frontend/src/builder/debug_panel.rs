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
        <div class="sidebar-section debug-panel">
            <h3 class="panel-title">"🐛 Debug & Metrics"</h3>

            <div class="debug-group">
                <h4 class="debug-subtitle">"State"</h4>
                <div class="debug-row">
                    <span>"Components:"</span>
                    <span class="debug-val">{move || format!("{}", app_state.canvas.components.get().len())}</span>
                </div>
                <div class="debug-row">
                    <span>"History:"</span>
                    <span class="debug-val">{move || format!("U:{} / R:{}",
                        if app_state.canvas.history.with(|h| h.can_undo()) { "Yes" } else { "No" },
                        if app_state.canvas.history.with(|h| h.can_redo()) { "Yes" } else { "No" }
                    )}</span>
                </div>
            </div>

            <div class="debug-group">
                <h4 class="debug-subtitle">"Performance"</h4>
                <div class="debug-row">
                    <span>"Render Time:"</span>
                    <span class="debug-val">{move || format!("{:.2} ms", app_state.ui.render_time.get())}</span>
                </div>
                <div class="debug-row">
                    <span>"Renders:"</span>
                    <span class="debug-val">{move || app_state.ui.render_count.get()}</span>
                </div>
            </div>

            <div class="debug-group">
                <h4 class="debug-subtitle">"Session Analytics"</h4>
                <div class="debug-row">
                    <span>"Recorded:"</span>
                    <span class="debug-val">{move || analytics.metrics_summary().total_metrics}</span>
                </div>
                <div class="debug-row">
                    <span>"Avg Render:"</span>
                    <span class="debug-val">{move || format!("{:.2} ms", analytics.metrics_summary().avg_render_time)}</span>
                </div>
                <div class="debug-row">
                    <span>"Last Sync:"</span>
                    <span class="debug-val">{last_sync_display}</span>
                </div>

                <div class="session-stats">
                    <div class="stat-badge" title="Components Created">
                        "➕ " {move || analytics.session_info().components_created}
                    </div>
                    <div class="stat-badge" title="Saves">
                        "💾 " {move || analytics.session_info().saves_count}
                    </div>
                    <div class="stat-badge" title="Undos">
                        "↩️ " {move || analytics.session_info().undo_count}
                    </div>
                </div>

                <button
                    class="btn btn-sm btn-outline width-full mt-2"
                    disabled=move || sync_loading.get()
                    on:click=sync_analytics
                >
                    {move || if sync_loading.get() { "Syncing..." } else { "Force Sync Stats" }}
                </button>
            </div>

            <details class="debug-details">
                <summary>"📦 Raw State Dump"</summary>
                <pre class="debug-dump">
                    {move || format!("{:#?}", app_state.canvas.components.get())}
                </pre>
            </details>
        </div>
    }
}
