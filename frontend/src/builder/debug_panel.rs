use crate::builder::component_library::ComponentRegistry;
use crate::state::app_state::AppState;
use leptos::prelude::*;

#[component]
pub fn DebugPanel() -> impl IntoView {
    // Get AppState from context (no props!)
    let app_state = AppState::expect_context();

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
            <details class="debug-details"><summary>"📦 Dump State"</summary>
                <pre class="debug-dump">
                    {move || format!("{:#?}", app_state.canvas.components.get())}
                </pre>
            </details>
        </div>
    }
}
