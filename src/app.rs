use leptos::*;
use crate::builder::sidebar::{Sidebar, SidebarProps};
use crate::builder::canvas::{Canvas, CanvasProps};


use std::cell::Cell;
use std::rc::Rc;
use super::builder::snackbar::Snackbar;
use super::builder::component_library::{Theme, ResponsiveMode};
use super::builder::component_library::LibraryComponent;
use super::builder::canvas::{SelectedComponent, CanvasComponent};
use super::builder::property_editor::PropertyEditor;
use super::builder::preview::Preview;

#[component]
pub fn App() -> impl IntoView {
    web_sys::console::log_1(&"App: before view!".into());
    let responsive_mode = create_rw_signal(ResponsiveMode::Desktop);
    let notification = create_rw_signal(None::<String>);
    // State global theme (light/dark)
    let theme = create_rw_signal(Theme::Light);
    // State custom theme color (sinkron dengan sidebar)
    let custom_theme_color = create_rw_signal(String::from("#888"));
    // Debug log
    web_sys::console::log_1(&"🔥 Leptos Studio App component loaded".into());
    web_sys::console::log_1(&"🔥 Starting component initialization".into());
    
    let selected = create_rw_signal(SelectedComponent { idx: None });
    let components = create_rw_signal(Vec::<CanvasComponent>::new());
    // Undo/Redo stacks
    let undo_stack = create_rw_signal(Vec::<Vec<CanvasComponent>>::new());
    let redo_stack = create_rw_signal(Vec::<Vec<CanvasComponent>>::new());
    
    web_sys::console::log_1(&"🔥 State signals created".into());
    // Save/load handlers
    let notification = notification.clone();
    let save_layout = move |_| {
        if let Ok(json) = serde_json::to_string(&components.get()) {
            if web_sys::window().unwrap().local_storage().unwrap().unwrap().set_item("leptos_studio_layout", &json).is_ok() {
                notification.set(Some("Layout berhasil disimpan!".to_string()));
            } else {
                notification.set(Some("Gagal menyimpan layout!".to_string()));
            }
        } else {
            notification.set(Some("Gagal serialisasi layout!".to_string()));
        }
    };
    let notification = notification.clone();
    let load_layout = {
        let components = components.clone();
        let undo_stack = undo_stack.clone();
        let redo_stack = redo_stack.clone();
        let notification = notification.clone();
        move |_| {
            if let Ok(Some(json)) = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("leptos_studio_layout") {
                if let Ok(data) = serde_json::from_str(&json) {
                    // Push current state to undo stack before loading
                    undo_stack.update(|stack| stack.push(components.get()));
                    redo_stack.set(Vec::new());
                    components.set(data);
                    notification.set(Some("Layout berhasil dimuat!".to_string()));
                } else {
                    notification.set(Some("Gagal parsing layout!".to_string()));
                }
            } else {
                notification.set(Some("Tidak ada layout yang tersimpan!".to_string()));
            }
        }
    };

    // Undo handler
    let notification = notification.clone();
    let do_undo = {
        let components = components.clone();
        let undo_stack = undo_stack.clone();
        let redo_stack = redo_stack.clone();
        let notification = notification.clone();
        move |_| {
            let mut undo = undo_stack.get();
            if let Some(prev) = undo.pop() {
                redo_stack.update(|stack| stack.push(components.get()));
                components.set(prev);
                undo_stack.set(undo);
                notification.set(Some("Undo berhasil.".to_string()));
            } else {
                notification.set(Some("Tidak ada aksi untuk di-undo.".to_string()));
            }
        }
    };
    // Redo handler
    let notification = notification.clone();
    let do_redo = {
        let components = components.clone();
        let undo_stack = undo_stack.clone();
        let redo_stack = redo_stack.clone();
        let notification = notification.clone();
        move |_| {
            let mut redo = redo_stack.get();
            if let Some(next) = redo.pop() {
                undo_stack.update(|stack| stack.push(components.get()));
                components.set(next);
                redo_stack.set(redo);
                notification.set(Some("Redo berhasil.".to_string()));
            } else {
                notification.set(Some("Tidak ada aksi untuk di-redo.".to_string()));
            }
        }
    };
    // State untuk export modal
    let show_export = create_rw_signal(false);
    let export_code = create_rw_signal(String::new());
    let export_template = create_rw_signal("leptos".to_string());

    // Custom komponen tetap ada untuk form
    let custom_components = create_rw_signal(Vec::<crate::builder::component_library::LibraryComponent>::new());

    let do_export = {
        let components = components.clone();
        let export_code = export_code.clone();
        let export_template = export_template.clone();
        let show_export = show_export.clone();
        let notification = notification.clone();
        let custom_components = custom_components.clone();
        move |_| {
            let code = match export_template.get().as_str() {
                "leptos" => crate::builder::export::generate_leptos_code(&components.get(), &custom_components.get()),
                "html" => crate::builder::export::generate_html_code(&components.get(), &custom_components.get()),
                "markdown" => crate::builder::export::generate_markdown_code(&components.get(), &custom_components.get()),
                "json" => serde_json::to_string_pretty(&components.get()).unwrap_or("// Export failed".to_string()),
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
            props_schema: Some(vec![
                crate::builder::component_library::PropSchema { name: "label".to_string(), prop_type: "string".to_string(), required: true, description: None },
            ]),
            description: None,
        },
        LibraryComponent {
            name: "Text".to_string(),
            kind: "Text".to_string(),
            template: None,
            category: "Basic".to_string(),
            props_schema: Some(vec![
                crate::builder::component_library::PropSchema { name: "content".to_string(), prop_type: "string".to_string(), required: true, description: None },
            ]),
            description: None,
        },
        LibraryComponent {
            name: "Input".to_string(),
            kind: "Input".to_string(),
            template: None,
            category: "Basic".to_string(),
            props_schema: Some(vec![
                crate::builder::component_library::PropSchema { name: "placeholder".to_string(), prop_type: "string".to_string(), required: false, description: None },
            ]),
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
    let component_library = create_rw_signal({
        // Coba load dari localStorage
        if let Ok(Some(json)) = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("leptos_studio_component_library") {
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
    {
        let component_library = component_library.clone();
        create_effect(move |_| {
            if let Ok(json) = serde_json::to_string(&component_library.get()) {
                let _ = web_sys::window().unwrap().local_storage().unwrap().unwrap().set_item("leptos_studio_component_library", &json);
            }
        });
    }
    // Custom komponen tetap ada untuk form
    let custom_components = create_rw_signal(Vec::<crate::builder::component_library::LibraryComponent>::new());

    // Performance monitoring
    let render_count = Rc::new(Cell::new(0));
    let render_start = Rc::new(Cell::new(0f64));
    let render_time = Rc::new(Cell::new(0f64));
    // Set start time on first render
    {
        let render_start = render_start.clone();
        create_effect(move |_| {
            if render_start.get() == 0.0 {
                render_start.set(js_sys::Date::now());
            }
        });
    }
    // Count renders and measure time
    {
        let render_count = render_count.clone();
        let render_time = render_time.clone();
        let render_start = render_start.clone();
        create_effect(move |_| {
            render_count.set(render_count.get() + 1);
            render_time.set(js_sys::Date::now() - render_start.get());
        });
    }
    view! {
        <div class="leptos-studio" style="background: #fffbe6; min-height: 100vh; border: 8px dashed #ff9800; box-shadow:0 0 20px #ff9800;">
                    <h1 style="color: red; font-size: 24px; text-align: center; background: yellow; padding: 10px; margin: 0;">"🔥 LEPTOS STUDIO - DEBUG MODE 🔥"</h1>
                    <div style="border: 3px solid blue; background: lightcyan;">
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
                    </div>
                    <div style="display: flex; flex-direction: column; flex: 1; border: 3px solid green; background: lightgreen;">
                        <div style="padding: 0.5rem; background: #e0e0e0; border-bottom: 1px solid #ddd;">
                            <button on:click=save_layout style="background: orange; padding: 10px; color: white; border: none; margin: 5px;">Save Layout</button>
                            <button on:click=load_layout style="margin-left: 0.5rem; background: purple; padding: 10px; color: white; border: none; margin: 5px;">Load Layout</button>
                            <button on:click=do_export style="margin-left: 0.5rem; background: red; padding: 10px; color: white; border: none; margin: 5px;">Export</button>
                            <button on:click=do_undo style="margin-left: 0.5rem; background: #555; padding: 10px; color: white; border: none; margin: 5px;">Undo</button>
                            <button on:click=do_redo style="margin-left: 0.5rem; background: #888; padding: 10px; color: white; border: none; margin: 5px;">Redo</button>
                        </div>
                        <div style="border: 2px solid orange; background: lightyellow;">
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
                        </div>
                        <div style="margin-top:1rem; padding:1rem; border: 2px solid pink; background: lightpink;">
                            <h4>Live Preview</h4>
                                <Preview 
                                    components=components 
                                    theme=theme 
                                    responsive_mode=responsive_mode
                                    custom_components=custom_components
                                />
                        </div>
                    </div>
                    <div style="border: 3px solid purple; background: lavender;">
                        <PropertyEditor selected=selected components=components component_library=component_library notification=notification custom_components=custom_components />
                    </div>
                    {move || if show_export.get() {
                view! {
                    <div style="position:fixed;top:0;left:0;width:100vw;height:100vh;background:rgba(0,0,0,0.3);z-index:1000;display:flex;align-items:center;justify-content:center;">
                        <div style="background:#fff;padding:2rem;border-radius:8px;min-width:400px;max-width:90vw;">
                            <h3>Export Code</h3>
                            <label for="export-template">Template: </label>
                            <select id="export-template" value=export_template on:input=move |ev| export_template.set(event_target_value(&ev)) style="margin-bottom:1em;">
                                <option value="leptos">Leptos Component</option>
                                <option value="html">HTML</option>
                                <option value="markdown">Markdown</option>
                                <option value="json">Raw JSON</option>
                            </select>
                            <textarea style="width:100%;height:300px;" readonly>{export_code.get()}</textarea>
                            <div style="text-align:right;margin-top:1rem;">
                                <button on:click=close_export>Tutup</button>
                            </div>
                        </div>
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
            <Snackbar notification=notification />
        </div>
    }
}
