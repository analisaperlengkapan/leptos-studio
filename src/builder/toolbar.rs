use crate::builder::hooks::use_export_actions::use_export_actions;
use crate::state::app_state::{AppState, Notification};
use leptos::prelude::*;

#[component]
pub fn Toolbar(
    show_template_gallery: WriteSignal<bool>,
    show_export: WriteSignal<bool>,
    export_code: WriteSignal<String>,
    export_template: ReadSignal<String>,
) -> impl IntoView {
    let app_state = AppState::expect_context();

    // Save/Load handlers
    let save_layout = move |_| {
        if let Err(e) = app_state.save() {
            app_state
                .ui
                .notify(Notification::error(format!("❌ {}", e.user_message())));
        } else {
            app_state
                .ui
                .notify(Notification::success("💾 Layout saved!".to_string()));
        }
    };

    let load_layout = move |_| {
        if let Err(e) = app_state.load() {
            app_state
                .ui
                .notify(Notification::error(format!("❌ {}", e.user_message())));
        } else {
            app_state
                .ui
                .notify(Notification::success("📂 Layout loaded!".to_string()));
        }
    };

    // Export handler
    let do_export = use_export_actions(show_export, export_code, export_template);

    // Undo/Redo handlers
    let do_undo = move |_| {
        if let Some(snapshot) = app_state.canvas.history.write().undo() {
            app_state.canvas.apply_snapshot(&snapshot);
            app_state
                .ui
                .notify(Notification::info("↪️ Undo".to_string()));
        }
    };

    let do_redo = move |_| {
        if let Some(snapshot) = app_state.canvas.history.write().redo() {
            app_state.canvas.apply_snapshot(&snapshot);
            app_state
                .ui
                .notify(Notification::info("↪️ Redo".to_string()));
        }
    };

    // History state tracking
    let can_undo = Memo::new(move |_| app_state.canvas.history.with(|h| h.can_undo()));
    let can_redo = Memo::new(move |_| app_state.canvas.history.with(|h| h.can_redo()));

    // Preview mode
    let is_preview = app_state.ui.preview_mode;
    let toggle_preview = move |_| {
        is_preview.update(|p| *p = !*p);
        if is_preview.get() {
            app_state
                .ui
                .notify(Notification::info("👁️ Preview Mode On".to_string()));
            app_state.canvas.selected.set(None); // Clear selection
        } else {
            app_state
                .ui
                .notify(Notification::info("✏️ Edit Mode On".to_string()));
        }
    };

    view! {
        <header class="app-header">
            <div class="header-left">
                <div class="logo-area">
                    <h1>{"Leptos Studio"}</h1>
                    <span class="version-badge">"Beta"</span>
                </div>

                <div class="divider-vertical"></div>

                <div class="toolbar-group">
                    <button
                        on:click=save_layout
                        class="btn btn-ghost btn-sm"
                        title="Save layout (Ctrl+S)"
                    >
                        <span class="icon">"💾"</span>
                        <span class="label">"Save"</span>
                    </button>
                    <button
                        on:click=load_layout
                        class="btn btn-ghost btn-sm"
                        title="Load layout (Ctrl+O)"
                    >
                        <span class="icon">"📂"</span>
                        <span class="label">"Load"</span>
                    </button>
                </div>

                <div class="divider-vertical"></div>

                <div class="toolbar-group">
                    <button
                        on:click=do_undo
                        class="btn btn-ghost btn-sm"
                        disabled=move || !can_undo.get()
                        title="Undo (Ctrl+Z)"
                    >
                        <span class="icon">"↩️"</span>
                    </button>
                    <button
                        on:click=do_redo
                        class="btn btn-ghost btn-sm"
                        disabled=move || !can_redo.get()
                        title="Redo (Ctrl+Y)"
                    >
                        <span class="icon">"↪️"</span>
                    </button>
                </div>

                 <div class="divider-vertical"></div>

                 <div class="toolbar-group">
                    <button
                        on:click=do_export
                        class="btn btn-ghost btn-sm"
                        title="Export code"
                    >
                        <span class="icon">"📤"</span>
                        <span class="label">"Export"</span>
                    </button>
                 </div>
            </div>

            <div class="header-right">
                <button
                    class=move || if is_preview.get() { "btn btn-primary btn-sm toggle-active" } else { "btn btn-outline btn-sm" }
                    on:click=toggle_preview
                    title="Toggle Preview Mode"
                >
                    <span class="icon">{move || if is_preview.get() { "👁️" } else { "✏️" }}</span>
                    <span class="label">{move || if is_preview.get() { "Preview" } else { "Edit" }}</span>
                </button>

                <div class="divider-vertical"></div>

                <button
                    class="btn btn-ghost btn-sm"
                    on:click=move |_| show_template_gallery.set(true)
                    title="Open template gallery"
                >
                    <span class="icon">"📑"</span>
                    <span class="label">"Templates"</span>
                </button>
            </div>
        </header>
    }
}
