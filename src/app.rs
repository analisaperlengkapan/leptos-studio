use leptos::prelude::*;

use crate::builder::canvas::Canvas;
use crate::builder::sidebar::Sidebar;
use crate::builder::property_editor::PropertyEditor;
use crate::builder::preview::Preview;
use crate::builder::design_tokens::{DesignTokenProvider, DesignTokens};
use crate::builder::snackbar::Snackbar;
use crate::builder::breadcrumb::BreadcrumbNavigation;
use crate::builder::drag_drop::DragPreview;
use crate::builder::keyboard::{KeyboardHandler, KeyboardAction, get_default_shortcuts};
use crate::builder::command_palette::CommandPalette;
use crate::state::app_state::{AppState, Notification};
use crate::services::export_service::{CodeGenerator, LeptosCodeGenerator, HtmlCodeGenerator, MarkdownCodeGenerator};
use crate::domain::component::CanvasComponent;

#[component]
pub fn App() -> impl IntoView {
    // Initialize global AppState context
    AppState::provide_context();
    let app_state = AppState::use_context();

    // Design tokens
    let design_tokens = RwSignal::new(DesignTokens::default());

    // Export modal (local UI state)
    let show_export = RwSignal::new(false);
    let export_code = RwSignal::new(String::new());
    let export_template = RwSignal::new("leptos".to_string());

    // Keyboard action handler
    let keyboard_action_handler = move |action: KeyboardAction| {
        match action {
            KeyboardAction::Undo => {
                if let Some(snapshot) = app_state.canvas.history.write().undo() {
                    app_state.canvas.apply_snapshot(&snapshot);
                    app_state.ui.notification.set(Some(Notification::info("↩️ Undo".to_string())));
                } else {
                    app_state.ui.notification.set(Some(Notification::warning("⚠️ Nothing to undo".to_string())));
                }
            }
            KeyboardAction::Redo => {
                if let Some(snapshot) = app_state.canvas.history.write().redo() {
                    app_state.canvas.apply_snapshot(&snapshot);
                    app_state.ui.notification.set(Some(Notification::info("↪️ Redo".to_string())));
                } else {
                    app_state.ui.notification.set(Some(Notification::warning("⚠️ Nothing to redo".to_string())));
                }
            }
            KeyboardAction::Save => {
                if let Err(e) = app_state.save() {
                    app_state.ui.notification.set(Some(Notification::error(format!("❌ Save failed: {}", e))));
                } else {
                    app_state.ui.notification.set(Some(Notification::success("💾 Layout saved!".to_string())));
                }
            }
            KeyboardAction::Delete => {
                if let Some(selected_id) = app_state.canvas.selected.get() {
                    app_state.canvas.record_snapshot();
                    app_state.canvas.remove_component(&selected_id);
                    app_state.canvas.selected.set(None);
                    app_state.ui.notification.set(Some(Notification::success("🗑️ Component deleted".to_string())));
                } else {
                    app_state.ui.notification.set(Some(Notification::warning("⚠️ No component selected".to_string())));
                }
            }
            KeyboardAction::Copy => {
                if let Some(selected_id) = app_state.canvas.selected.get() {
                    if let Some(comp) = app_state.canvas.get_component(&selected_id)
                        && let Ok(json) = serde_json::to_string(&comp)
                            && let Some(window) = web_sys::window() {
                                let clipboard = window.navigator().clipboard();
                                let promise = clipboard.write_text(&json);
                                wasm_bindgen_futures::spawn_local(async move {
                                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                });
                                app_state.ui.notification.set(Some(Notification::success("📋 Component copied!".to_string())));
                            }
                } else {
                    app_state.ui.notification.set(Some(Notification::warning("⚠️ No component selected".to_string())));
                }
            }
            KeyboardAction::Paste => {
                if let Some(window) = web_sys::window() {
                    let clipboard = window.navigator().clipboard();
                    let promise = clipboard.read_text();
                    let app_state_clone = app_state;
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(result) = wasm_bindgen_futures::JsFuture::from(promise).await
                            && let Some(text) = result.as_string() {
                                if let Ok(comp) = serde_json::from_str::<CanvasComponent>(&text) {
                                    app_state_clone.canvas.record_snapshot();
                                    app_state_clone.canvas.add_component(comp);
                                    app_state_clone.ui.notification.set(Some(Notification::success("📋 Component pasted!".to_string())));
                                } else {
                                    app_state_clone.ui.notification.set(Some(Notification::error("⚠️ Invalid clipboard content".to_string())));
                                }
                            }
                    });
                }
            }
            KeyboardAction::Duplicate => {
                if let Some(selected_id) = app_state.canvas.selected.get() {
                    if let Some(comp) = app_state.canvas.get_component(&selected_id) {
                        app_state.canvas.record_snapshot();
                        app_state.canvas.add_component(comp);
                        app_state.ui.notification.set(Some(Notification::success("🔄 Component duplicated!".to_string())));
                    }
                } else {
                    app_state.ui.notification.set(Some(Notification::warning("⚠️ No component selected".to_string())));
                }
            }
            KeyboardAction::NewComponent => {
                app_state.ui.notification.set(Some(Notification::info("ℹ️ Drag component from sidebar to add".to_string())));
            }
            KeyboardAction::OpenCommandPalette => {
                app_state.ui.show_command_palette.set(true);
            }
            KeyboardAction::Deselect => {
                app_state.canvas.selected.set(None);
            }
            KeyboardAction::Export => {
                let comps = app_state.canvas.components.get();
                let generator = LeptosCodeGenerator::new(crate::state::ExportPreset::Plain);
                let code = generator.generate(&comps).unwrap_or_else(|e| format!("Error: {}", e));
                export_code.set(code);
                show_export.set(true);
            }
            _ => {}
        }
    };

    // Export handler
    let do_export = move |_| {
        let comps = app_state.canvas.components.get();
        
        let code = match export_template.get().as_str() {
            "leptos" => {
                let generator = LeptosCodeGenerator::new(crate::state::ExportPreset::Plain);
                generator.generate(&comps).unwrap_or_else(|e| format!("Error: {}", e))
            }
            "html" => {
                let generator = HtmlCodeGenerator;
                generator.generate(&comps).unwrap_or_else(|e| format!("Error: {}", e))
            }
            "markdown" => {
                let generator = MarkdownCodeGenerator;
                generator.generate(&comps).unwrap_or_else(|e| format!("Error: {}", e))
            }
            "json" => {
                serde_json::to_string_pretty(&comps).unwrap_or_else(|e| format!("Error: {}", e))
            }
            _ => "Unknown template".to_string(),
        };
        
        export_code.set(code);
        show_export.set(true);
    };

    // Save/Load handlers
    let save_layout = move |_| {
        if let Err(e) = app_state.save() {
            app_state.ui.notification.set(Some(Notification::error(format!("❌ Save failed: {}", e))));
        } else {
            app_state.ui.notification.set(Some(Notification::success("💾 Layout saved!".to_string())));
        }
    };

    let load_layout = move |_| {
        if let Err(e) = app_state.load() {
            app_state.ui.notification.set(Some(Notification::error(format!("❌ Load failed: {}", e))));
        } else {
            app_state.ui.notification.set(Some(Notification::success("📂 Layout loaded!".to_string())));
        }
    };

    // Undo/Redo handlers
    let do_undo = move |_| {
        if let Some(snapshot) = app_state.canvas.history.write().undo() {
            app_state.canvas.components.set(snapshot.components);
            app_state.canvas.selected.set(snapshot.selected);
            app_state.ui.notification.set(Some(Notification::info("↩️ Undo".to_string())));
        }
    };

    let do_redo = move |_| {
        if let Some(snapshot) = app_state.canvas.history.write().redo() {
            app_state.canvas.components.set(snapshot.components);
            app_state.canvas.selected.set(snapshot.selected);
            app_state.ui.notification.set(Some(Notification::info("↪️ Redo".to_string())));
        }
    };

    let close_export = move |_| show_export.set(false);

    view! {
        <DesignTokenProvider tokens=design_tokens>
            <div class="leptos-studio" tabindex="0">
                <KeyboardHandler
                    shortcuts=get_default_shortcuts()
                    on_action=keyboard_action_handler
                />

                <DragPreview drag_state=app_state.canvas.drag_state />

                <CommandPalette
                    is_open=app_state.ui.show_command_palette.read_only()
                    close=app_state.ui.show_command_palette.write_only()
                    search=RwSignal::new(String::new())
                    on_action=keyboard_action_handler
                />

                <header class="app-header">
                    <h1>{"Leptos Studio"}</h1>
                </header>

                <BreadcrumbNavigation />

                <div class="app-layout">
                    <aside class="sidebar-panel">
                        <Sidebar />
                    </aside>
                    <main>
                        <nav class="main-nav">
                            <div class="nav-actions">
                                <button on:click=save_layout class="btn btn-primary">{"Save"}</button>
                                <button on:click=load_layout class="btn btn-secondary">{"Load"}</button>
                                <button on:click=do_export class="btn btn-success">{"Export"}</button>
                                <button on:click=do_undo class="btn btn-outline">{"Undo"}</button>
                                <button on:click=do_redo class="btn btn-outline">{"Redo"}</button>
                            </div>
                        </nav>
                        <div class="main-content">
                            <section class="canvas-area">
                                <Canvas />
                            </section>
                            <aside class="property-panel">
                                <div class="property-editor-section">
                                    <PropertyEditor />
                                </div>
                                <div class="preview-section">
                                    <Preview />
                                </div>
                            </aside>
                        </div>
                    </main>
                </div>

                {move || if show_export.get() {
                    view! {
                        <div style="position:fixed;top:0;left:0;width:100vw;height:100vh;background:rgba(0,0,0,0.3);z-index:1000;display:flex;align-items:center;justify-content:center;">
                            <div style="background:#fff;padding:2rem;border-radius:8px;min-width:400px;">
                                <h3>{"Export Code"}</h3>
                                <select
                                    prop:value=export_template
                                    on:input=move |ev| export_template.set(event_target_value(&ev))
                                    style="margin-bottom:1em;"
                                >
                                    <option value="leptos">{"Leptos Component"}</option>
                                    <option value="html">{"HTML"}</option>
                                    <option value="markdown">{"Markdown"}</option>
                                    <option value="json">{"Raw JSON"}</option>
                                </select>
                                <textarea style="width:100%;height:300px;" readonly>
                                    {export_code.get()}
                                </textarea>
                                <button on:click=close_export>{"Close"}</button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                <Snackbar notification=app_state.ui.notification />
            </div>
        </DesignTokenProvider>
    }
}
