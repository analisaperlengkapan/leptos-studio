use crate::builder::breadcrumb::BreadcrumbNavigation;
use crate::builder::canvas::{Canvas, CanvasProps};
use crate::builder::command_palette::CommandPalette;
use crate::builder::design_tokens::{DesignTokenProvider, DesignTokens};
use crate::builder::drag_drop::{DragPreview, DragState};
use crate::builder::keyboard::{get_default_shortcuts, KeyboardAction, KeyboardHandler};
use crate::builder::sidebar::{Sidebar, SidebarProps};
use leptos::prelude::*;

use super::builder::canvas::{CanvasComponent, SelectedComponent};
use super::builder::component_library::LibraryComponent;
use super::builder::component_library::{ResponsiveMode, Theme};
use super::builder::preview::Preview;
use super::builder::property_editor::PropertyEditor;
use super::builder::snackbar::Snackbar;
use std::cell::Cell;
use std::rc::Rc;

#[component]
pub fn App() -> impl IntoView {
    let responsive_mode = RwSignal::new(ResponsiveMode::Desktop);
    let notification = RwSignal::new(None::<String>);
    // State global theme (light/dark)
    let theme = RwSignal::new(Theme::Light);
    // State custom theme color (sinkron dengan sidebar)
    let custom_theme_color = RwSignal::new(String::from("#888"));

    // Design tokens
    let design_tokens = RwSignal::new(DesignTokens::default());

    // Drag state for enhanced drag & drop
    let drag_state = RwSignal::new(DragState::NotDragging);

    // Command palette state
    let show_command_palette = RwSignal::new(false);
    let command_search = RwSignal::new(String::new());

    let selected = RwSignal::new(SelectedComponent { idx: None });
    let components = RwSignal::new(Vec::<CanvasComponent>::new());
    // Undo/Redo stacks
    let undo_stack = RwSignal::new(Vec::<Vec<CanvasComponent>>::new());
    let redo_stack = RwSignal::new(Vec::<Vec<CanvasComponent>>::new());
    // Save/load handlers
    let save_layout = move |_| {
        if let Ok(json) = serde_json::to_string(&components.get()) {
            if web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap()
                .set_item("leptos_studio_layout", &json)
                .is_ok()
            {
                notification.set(Some("Layout berhasil disimpan!".to_string()));
            } else {
                notification.set(Some("Gagal menyimpan layout!".to_string()));
            }
        } else {
            notification.set(Some("Gagal serialisasi layout!".to_string()));
        }
    };
    let load_layout = move |_| {
        if let Ok(Some(json)) = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("leptos_studio_layout")
        {
            if let Ok(data) = serde_json::from_str(&json) {
                // Push current state to undo stack
                undo_stack.update(|stack| stack.push(components.get()));
                redo_stack.set(Vec::new());
                components.set(data);
                notification.set(Some("Layout berhasil dimuat!".to_string()));
            } else {
                notification.set(Some("Gagal parsing JSON layout!".to_string()));
            }
        } else {
            notification.set(Some("Tidak ada layout yang disimpan!".to_string()));
        }
    };
    // Undo handler
    let do_undo = move |_| {
        let mut undo = undo_stack.get();
        if let Some(prev) = undo.pop() {
            redo_stack.update(|stack| stack.push(components.get()));
            components.set(prev);
            undo_stack.set(undo);
            notification.set(Some("Undo berhasil.".to_string()));
        } else {
            notification.set(Some("Tidak ada aksi untuk di-undo.".to_string()));
        }
    };
    // Redo handler
    let do_redo = move |_| {
        let mut redo = redo_stack.get();
        if let Some(next) = redo.pop() {
            undo_stack.update(|stack| stack.push(components.get()));
            components.set(next);
            redo_stack.set(redo);
            notification.set(Some("Redo berhasil.".to_string()));
        } else {
            notification.set(Some("Tidak ada aksi untuk di-redo.".to_string()));
        }
    };
    // State untuk export modal
    let show_export = RwSignal::new(false);
    let export_code = RwSignal::new(String::new());
    use crate::builder::export::ExportPreset;
    let export_template = RwSignal::new("leptos".to_string());

    // Custom komponen tetap ada untuk form
    let custom_components =
        RwSignal::new(Vec::<crate::builder::component_library::LibraryComponent>::new());

    let do_export = move |_| {
        let preset = match export_template.get().as_str() {
            "leptos" => ExportPreset::Plain,
            "html" => ExportPreset::Plain,
            _ => ExportPreset::Plain,
        };
        let code = match export_template.get().as_str() {
            "leptos" => crate::builder::export::generate_leptos_code(
                &components.get(),
                &custom_components.get(),
                preset,
            ),
            "html" => crate::builder::export::generate_html_code(
                &components.get(),
                &custom_components.get(),
                preset,
            ),
            "markdown" => crate::builder::export::generate_markdown_code(
                &components.get(),
                &custom_components.get(),
            ),
            "json" => serde_json::to_string_pretty(&components.get())
                .unwrap_or("// Export failed".to_string()),
            _ => "// Unknown template".to_string(),
        };
        let msg = match export_template.get().as_str() {
            "leptos" => "Export Leptos code berhasil!",
            "html" => "Export HTML berhasil!",
            "markdown" => "Export Markdown berhasil!",
            "json" => "Export JSON berhasil!",
            _ => "Export gagal!",
        };
        export_code.set(code);
        show_export.set(true);
        notification.set(Some(msg.to_string()));
    };

    // Keyboard action handler
    let keyboard_action_handler = move |action: KeyboardAction| {
        match action {
            KeyboardAction::Undo => do_undo(leptos::ev::MouseEvent::new("click").unwrap()),
            KeyboardAction::Redo => do_redo(leptos::ev::MouseEvent::new("click").unwrap()),
            KeyboardAction::Save => save_layout(leptos::ev::MouseEvent::new("click").unwrap()),
            KeyboardAction::Export => do_export(leptos::ev::MouseEvent::new("click").unwrap()),
            KeyboardAction::OpenCommandPalette => show_command_palette.set(true),
            KeyboardAction::Deselect => selected.set(SelectedComponent { idx: None }),
            KeyboardAction::Delete => {
                if let Some(idx) = selected.get().idx {
                    // Push current state to undo stack
                    undo_stack.update(|stack| stack.push(components.get()));
                    redo_stack.set(Vec::new());

                    components.update(|c| {
                        if idx < c.len() {
                            c.remove(idx);
                        }
                    });
                    selected.set(SelectedComponent { idx: None });
                    notification.set(Some("Komponen berhasil dihapus.".to_string()));
                }
            }
            KeyboardAction::SelectAll => {
                // Select last component (simple implementation)
                let count = components.get().len();
                if count > 0 {
                    selected.set(SelectedComponent {
                        idx: Some(count - 1),
                    });
                }
            }
            KeyboardAction::Copy => {
                if let Some(idx) = selected.get().idx {
                    let comps = components.get();
                    if let Some(comp) = comps.get(idx) {
                        // Store in browser clipboard using web-sys
                        if let Ok(json) = serde_json::to_string(comp) {
                            if let Some(window) = web_sys::window() {
                                let clipboard = window.navigator().clipboard();
                                let promise = clipboard.write_text(&json);
                                wasm_bindgen_futures::spawn_local(async move {
                                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                });
                                notification
                                    .set(Some("✂️ Komponen disalin ke clipboard!".to_string()));
                            } else {
                                notification.set(Some("⚠️ Window tidak tersedia.".to_string()));
                            }
                        } else {
                            notification.set(Some("⚠️ Gagal serialisasi komponen.".to_string()));
                        }
                    }
                } else {
                    notification.set(Some("⚠️ Tidak ada komponen yang dipilih.".to_string()));
                }
            }
            KeyboardAction::Paste => {
                if let Some(window) = web_sys::window() {
                    let clipboard = window.navigator().clipboard();
                    let promise = clipboard.read_text();
                    let components = components;
                    let undo_stack = undo_stack;
                    let redo_stack = redo_stack;
                    let notification = notification;
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(result) = wasm_bindgen_futures::JsFuture::from(promise).await {
                            if let Some(text) = result.as_string() {
                                if let Ok(comp) = serde_json::from_str::<CanvasComponent>(&text) {
                                    // Push current state to undo stack
                                    undo_stack.update(|stack| stack.push(components.get()));
                                    redo_stack.set(Vec::new());
                                    components.update(|c| c.push(comp));
                                    notification
                                        .set(Some("📋 Komponen berhasil di-paste!".to_string()));
                                } else {
                                    notification.set(Some(
                                        "⚠️ Clipboard tidak berisi komponen valid.".to_string(),
                                    ));
                                }
                            }
                        }
                    });
                }
            }
            KeyboardAction::Duplicate => {
                if let Some(idx) = selected.get().idx {
                    let comps = components.get();
                    if let Some(comp) = comps.get(idx).cloned() {
                        // Push current state to undo stack
                        undo_stack.update(|stack| stack.push(components.get()));
                        redo_stack.set(Vec::new());
                        components.update(|c| c.push(comp));
                        notification.set(Some("🔄 Komponen berhasil diduplikasi!".to_string()));
                    }
                } else {
                    notification.set(Some("⚠️ Tidak ada komponen yang dipilih.".to_string()));
                }
            }
            KeyboardAction::NewComponent => {
                // Open command palette or show notification
                notification.set(Some(
                    "ℹ️ Drag komponen dari sidebar untuk menambahkan.".to_string(),
                ));
            }
        }
    };

    let close_export = move |_| show_export.set(false);

    // State library komponen (default + custom)
    let default_components = vec![
        LibraryComponent {
            name: "Button".to_string(),
            kind: "Button".to_string(),
            template: None,
            category: "Basic".to_string(),
            props_schema: Some(vec![crate::builder::component_library::PropSchema {
                name: "label".to_string(),
                prop_type: "string".to_string(),
                required: true,
                description: None,
            }]),
            description: None,
        },
        LibraryComponent {
            name: "Text".to_string(),
            kind: "Text".to_string(),
            template: None,
            category: "Basic".to_string(),
            props_schema: Some(vec![crate::builder::component_library::PropSchema {
                name: "content".to_string(),
                prop_type: "string".to_string(),
                required: true,
                description: None,
            }]),
            description: None,
        },
        LibraryComponent {
            name: "Input".to_string(),
            kind: "Input".to_string(),
            template: None,
            category: "Basic".to_string(),
            props_schema: Some(vec![crate::builder::component_library::PropSchema {
                name: "placeholder".to_string(),
                prop_type: "string".to_string(),
                required: false,
                description: None,
            }]),
            description: None,
        },
        LibraryComponent {
            name: "Container".to_string(),
            kind: "Container".to_string(),
            template: None,
            category: "Basic".to_string(),
            props_schema: None,
            description: None,
        },
    ];
    let component_library = RwSignal::new({
        // Coba load dari localStorage
        if let Ok(Some(json)) = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("leptos_studio_component_library")
        {
            if let Ok(val) = serde_json::from_str(&json) {
                val
            } else {
                default_components.clone()
            }
        } else {
            default_components.clone()
        }
    });
    // Auto-save component_library ke localStorage setiap berubah
    Effect::new(move |_| {
        if let Ok(json) = serde_json::to_string(&component_library.get()) {
            _ = web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap()
                .set_item("leptos_studio_component_library", &json);
        }
    });

    #[cfg(debug_assertions)]
    let render_count = Rc::new(Cell::new(0));
    #[cfg(debug_assertions)]
    let render_time = Rc::new(Cell::new(0f64));
    #[cfg(not(debug_assertions))]
    let render_count = Rc::new(Cell::new(0));
    #[cfg(not(debug_assertions))]
    let render_time = Rc::new(Cell::new(0f64));

    view! {
        <DesignTokenProvider tokens=design_tokens>
            <div class="leptos-studio" style=move || {
                let bg = match theme.get() {
                    Theme::Light => "ffffff".to_string(),
                    Theme::Dark => "1a1a1a".to_string(),
                    Theme::Custom => custom_theme_color.get().trim_start_matches('#').to_string(),
                };
                let text = "333333";
                if theme.get() == Theme::Dark {
                    format!("background: #{}; color: #ffffff; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;", bg)
                } else {
                    format!("background: #{}; color: #{}; min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;", bg, text)
                }
            } tabindex="0">
                // Global keyboard handler
                <KeyboardHandler
                    shortcuts=get_default_shortcuts()
                    on_action=keyboard_action_handler
                />

                // Drag preview for enhanced drag & drop
                <DragPreview drag_state=drag_state />

                // Command palette
                <CommandPalette
                    is_open=show_command_palette.read_only()
                    close=show_command_palette.write_only()
                    search=command_search
                    on_action=keyboard_action_handler
                />

                <header style="
                    background: white; 
                    padding: 1rem; 
                    border-bottom: 1px solid var(--color-gray-200);
                    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
                    flex-shrink: 0;
                    height: 80px;
                    display: flex;
                    align-items: center;
                ">
                    <div style="display: flex; align-items: center; justify-content: space-between; width: 100%;">
                        <h1 style="margin: 0; font-size: 1.5rem; font-weight: 600; color: var(--color-gray-900);">{"Leptos Studio"}</h1>
                        <div style="display: flex; align-items: center; gap: 1rem;">
                            <kbd style="
                                font-size: 11px; 
                                background: var(--color-gray-100); 
                                padding: 4px 8px; 
                                border-radius: 4px; 
                                border: 1px solid var(--color-gray-200);
                                color: var(--color-gray-600);
                                font-family: monospace;
                            ">
                                {"⌘K to open command palette"}
                            </kbd>
                        </div>
                    </div>
                </header>

                // Breadcrumb navigation
                <BreadcrumbNavigation
                    components=components
                    selected=selected
                />

                <div class="app-layout">
                    <aside style="width: 300px; background: white;">
                        {Sidebar(SidebarProps {
                            custom_components,
                            theme,
                            responsive_mode,
                            selected,
                            undo_stack,
                            redo_stack,
                            components,
                            render_count: render_count.clone(),
                            render_time: render_time.clone(),
                            notification,
                            component_library,
                        })}
                    </aside>
                    <main>
                        <nav class="main-nav">
                            <div style="display: flex; gap: 0.5rem;">
                                <button on:click=save_layout class="btn btn-primary">{"Save"}</button>
                                <button on:click=load_layout class="btn btn-secondary">{"Load"}</button>
                                <button on:click=do_export class="btn btn-success">{"Export"}</button>
                                <button on:click=do_undo class="btn btn-outline">{"Undo"}</button>
                                <button on:click=do_redo class="btn btn-outline">{"Redo"}</button>
                            </div>
                        </nav>
                        <div class="main-content">
                            <section class="canvas-area">
                                {Canvas(CanvasProps {
                                    selected,
                                    components,
                                    undo_stack,
                                    redo_stack,
                                    theme,
                                    responsive_mode,
                                    custom_theme_color,
                                    custom_components,
                                })}
                            </section>
                            <aside class="property-panel">
                                <div class="property-editor-section">
                                    <PropertyEditor
                                        selected=selected
                                        components=components
                                        component_library=component_library
                                        notification=notification
                                        custom_components=custom_components
                                    />
                                </div>
                                <div class="preview-section">
                                    <Preview
                                        components=components
                                        theme=theme
                                        responsive_mode=responsive_mode
                                        custom_components=custom_components
                                    />
                                </div>
                            </aside>
                        </div>
                    </main>
                </div>
                {move || if show_export.get() {
                    view! {
                        <div style="position:fixed;top:0;left:0;width:100vw;height:100vh;background:rgba(0,0,0,0.3);z-index:1000;display:flex;align-items:center;justify-content:center;">
                            <div style="background:#fff;padding:2rem;border-radius:8px;min-width:400px;max-width:90vw;">
                                <h3>{"Export Code"}</h3>
                                <label for="export-template">{"Template: "}</label>
                                <select id="export-template" prop:value=export_template on:input=move |ev| export_template.set(event_target_value(&ev)) style="margin-bottom:1em;">
                                    <option value="leptos">{"Leptos Component"}</option>
                                    <option value="html">{"HTML"}</option>
                                    <option value="markdown">{"Markdown"}</option>
                                    <option value="json">{"Raw JSON"}</option>
                                </select>
                                <textarea style="width:100%;height:300px;" readonly>{export_code.get()}</textarea>
                                <div style="text-align:right;margin-top:1rem;">
                                    <button on:click=close_export>{"Tutup"}</button>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
                <Snackbar notification=notification />
            </div>
        </DesignTokenProvider>
    }
}
