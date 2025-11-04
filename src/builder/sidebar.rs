use leptos::prelude::*;
use web_sys::window;

use crate::state::app_state::{AppState, Notification, ExportPreset};
use crate::services::export_service::{CodeGenerator, LeptosCodeGenerator};
use crate::domain::validation::{ComponentNameValidator, HtmlTemplateValidator, Validator};
use crate::domain::error::ValidationError;
use super::component_library::{LibraryComponent, ResponsiveMode, Theme};
use super::drag_drop::DragState;
use crate::builder::debug_panel::DebugPanel;
use crate::builder::git_panel::GitPanel;

/// Convert ValidationError to user-friendly Indonesian message
fn validation_error_to_message(error: ValidationError) -> String {
    match error {
        ValidationError::EmptyName => "Nama komponen wajib diisi.".to_string(),
        ValidationError::InvalidName(_) => "Nama komponen hanya boleh huruf, angka, dan underscore.".to_string(),
        ValidationError::EmptyTemplate => "Template wajib diisi.".to_string(),
        ValidationError::InvalidTemplate(msg) => format!("Template tidak valid: {}", msg),
        _ => "Input tidak valid.".to_string(),
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let app_state = AppState::use_context();
    
    let custom_theme_color = RwSignal::new(String::from("#888"));
    let show_add_form = RwSignal::new(false);
    let new_name = RwSignal::new(String::new());
    let new_template = RwSignal::new(String::new());
    let filter_query = RwSignal::new(String::new());
    let error_msg = RwSignal::new(String::new());
    let editing_idx = RwSignal::new(None::<usize>);
    let edit_name = RwSignal::new(String::new());
    let edit_template = RwSignal::new(String::new());
    
    let add_custom_component = move |_| {
        let name = new_name.get().trim().to_string();
        let template = new_template.get().trim().to_string();
        
        // Validate component name using domain validator
        let name_validator = ComponentNameValidator;
        if let Err(e) = name_validator.validate(&name) {
            error_msg.set(validation_error_to_message(e));
            return;
        }
        
        // Check for duplicate names
        if app_state.ui.custom_components.get().iter().any(|c| c.name == name) {
            error_msg.set("Nama komponen sudah ada.".to_string());
            return;
        }
        
        // Validate template using domain validator
        let template_validator = HtmlTemplateValidator;
        if let Err(e) = template_validator.validate(&template) {
            error_msg.set(validation_error_to_message(e));
            return;
        }
        
        let new_component = LibraryComponent {
            name: name.clone(),
            kind: "Custom".to_string(),
            template: Some(template.clone()),
            category: "Custom".to_string(),
            props_schema: None,
            description: None,
        };
        
        app_state.ui.custom_components.update(|cc| cc.push(new_component.clone()));
        app_state.ui.component_library.update(|lib| lib.push(new_component));
        
        new_name.set(String::new());
        new_template.set(String::new());
        error_msg.set(String::new());
        show_add_form.set(false);
        
        app_state.ui.notification.set(Some(Notification::success("✅ Komponen berhasil ditambahkan".to_string())));
    };
    
    let delete_custom_component = move |idx: usize| {
        let name = app_state.ui.custom_components.get().get(idx).map(|c| c.name.clone());
        
        app_state.ui.custom_components.update(|cc| {
            if idx < cc.len() {
                cc.remove(idx);
            }
        });
        
        if let Some(name) = name {
            app_state.ui.component_library.update(|lib| {
                if let Some(pos) = lib.iter().position(|c| c.name == name) {
                    lib.remove(pos);
                }
            });
        }
        
        app_state.ui.notification.set(Some(Notification::success("🗑️ Komponen dihapus".to_string())));
    };
    
    let start_edit_custom_component = move |idx: usize| {
        if let Some(c) = app_state.ui.custom_components.get().get(idx) {
            edit_name.set(c.name.clone());
            edit_template.set(c.template.as_deref().unwrap_or("").to_string());
            editing_idx.set(Some(idx));
            error_msg.set(String::new());
        }
    };
    
    let save_edit_custom_component = move |_| {
        let idx = match editing_idx.get() {
            Some(i) => i,
            None => return,
        };
        
        let name = edit_name.get().trim().to_string();
        let template = edit_template.get().trim().to_string();
        
        // Validate component name using domain validator
        let name_validator = ComponentNameValidator;
        if let Err(e) = name_validator.validate(&name) {
            error_msg.set(validation_error_to_message(e));
            return;
        }
        
        // Check for duplicate names (excluding current component)
        let existing_custom = app_state.ui.custom_components.get();
        if existing_custom.iter().enumerate().any(|(i, c)| i != idx && c.name == name) {
            error_msg.set("Nama komponen sudah ada.".to_string());
            return;
        }
        
        // Validate template using domain validator
        let template_validator = HtmlTemplateValidator;
        if let Err(e) = template_validator.validate(&template) {
            error_msg.set(validation_error_to_message(e));
            return;
        }
        
        let old_name = existing_custom.get(idx).map(|c| c.name.clone());
        
        app_state.ui.custom_components.update(|cc| {
            if let Some(item) = cc.get_mut(idx) {
                item.name = name.clone();
                item.template = Some(template.clone());
            }
        });
        
        if let Some(old_name) = old_name {
            app_state.ui.component_library.update(|lib| {
                if let Some(item) = lib.iter_mut().find(|c| c.name == old_name) {
                    item.name = name.clone();
                    item.template = Some(template.clone());
                }
            });
        }
        
        editing_idx.set(None);
        edit_name.set(String::new());
        edit_template.set(String::new());
        error_msg.set(String::new());
        
        app_state.ui.notification.set(Some(Notification::success("✅ Komponen diperbarui".to_string())));
    };
    
    let cancel_edit_custom_component = move |_| {
        editing_idx.set(None);
        error_msg.set(String::new());
    };
    
    let export_code = move |_| {
        let components = app_state.canvas.components.get();
        let preset = app_state.settings.with(|s| s.export_preset.clone());
        let generator = LeptosCodeGenerator::new(preset);
        
        match generator.generate(&components) {
            Ok(code) => {
                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    let _ = clipboard.write_text(&code);
                }
                app_state.ui.notification.set(Some(Notification::success("✅ Kode diekspor!".to_string())));
            }
            Err(e) => {
                app_state.ui.notification.set(Some(Notification::error(format!("❌ Error: {}", e))));
            }
        }
    };
    
    view! {
        <aside class="sidebar-content">
            <DebugPanel />
            
            <div class="sidebar-section git-panel-wrapper">
                <b>"Git Panel"</b>
                <GitPanel />
            </div>
            
            <h2>"Sidebar"</h2>
            
            <div class="sidebar-section">
                <b>"Theme:"</b>
                <div class="theme-buttons">
                    <button
                        on:click=move |_| app_state.settings.update(|s| s.theme = Theme::Light)
                        disabled=move || app_state.settings.with(|s| s.theme == Theme::Light)
                    >"Light"</button>
                    <button
                        on:click=move |_| app_state.settings.update(|s| s.theme = Theme::Dark)
                        disabled=move || app_state.settings.with(|s| s.theme == Theme::Dark)
                    >"Dark"</button>
                    <button
                        on:click=move |_| app_state.settings.update(|s| s.theme = Theme::Custom)
                        disabled=move || app_state.settings.with(|s| s.theme == Theme::Custom)
                    >"Custom"</button>
                </div>
                {move || {
                    if app_state.settings.with(|s| s.theme == Theme::Custom) {
                        view! {
                            <input
                                type="color"
                                prop:value=move || custom_theme_color.get()
                                on:input=move |ev| custom_theme_color.set(event_target_value(&ev))
                            />
                        }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }
                }}
            </div>
            
            <div class="sidebar-section">
                <b>"Responsive:"</b>
                <select on:change=move |ev| {
                    let mode = match event_target_value(&ev).as_str() {
                        "Mobile" => ResponsiveMode::Mobile,
                        "Tablet" => ResponsiveMode::Tablet,
                        _ => ResponsiveMode::Desktop,
                    };
                    app_state.ui.responsive_mode.set(mode);
                }>
                    <option selected=move || app_state.ui.responsive_mode.get() == ResponsiveMode::Desktop>"Desktop"</option>
                    <option selected=move || app_state.ui.responsive_mode.get() == ResponsiveMode::Tablet>"Tablet"</option>
                    <option selected=move || app_state.ui.responsive_mode.get() == ResponsiveMode::Mobile>"Mobile"</option>
                </select>
            </div>
            
            <div class="sidebar-section">
                <label>"Export Preset:"</label>
                <select on:change=move |ev| {
                    let preset = match event_target_value(&ev).as_str() {
                        "ThawUi" => ExportPreset::ThawUi,
                        "LeptosMaterial" => ExportPreset::LeptosMaterial,
                        "LeptosUse" => ExportPreset::LeptosUse,
                        _ => ExportPreset::Plain,
                    };
                    app_state.settings.update(|s| s.export_preset = preset);
                }>
                    <option value="Plain">"Plain"</option>
                    <option value="ThawUi">"thaw-ui"</option>
                    <option value="LeptosMaterial">"leptos-material"</option>
                    <option value="LeptosUse">"leptos-use"</option>
                </select>
            </div>
            
            <button on:click=export_code>"Export Project"</button>
            
            {move || {
                app_state.ui.notification.get().as_ref().map(|notif| {
                    view! { <div style="color:green;margin-top:8px;">{notif.message.clone()}</div> }
                })
            }}
            
            <div style="margin-top:16px;border-top:1px solid #ccc;padding-top:12px;">
                <h3>"Component Library"</h3>
                <div style="display:grid;gap:8px;">
                    {move || {
                        app_state.ui.component_library.get()
                            .into_iter()
                            .map(|comp| {
                                let comp_kind = comp.kind.clone();
                                view! {
                                    <div 
                                        style="padding:8px;border:1px solid #ddd;border-radius:4px;cursor:move;background:#f9f9f9;"
                                        draggable="true"
                                        on:dragstart=move |ev| {
                                            let data_transfer = ev.data_transfer().unwrap();
                                            let _ = data_transfer.set_data("text/plain", &comp_kind);
                                            app_state.canvas.drag_state.set(DragState::Dragging {
                                                component_type: comp_kind.clone(),
                                                ghost_x: ev.client_x() as f64,
                                                ghost_y: ev.client_y() as f64,
                                            });
                                        }
                                        on:dragend=move |_| {
                                            app_state.canvas.drag_state.set(DragState::NotDragging);
                                        }
                                    >
                                        <div style="font-weight:bold;">{comp.name.clone()}</div>
                                        <div style="font-size:11px;color:#666;">
                                            {comp.description.clone().unwrap_or_default()}
                                        </div>
                                        <div style="font-size:10px;color:#999;margin-top:4px;">
                                            {comp.category.clone()}
                                        </div>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
            
            <div style="margin-top:16px;border-top:1px solid #ccc;padding-top:12px;">
                <h3>"Custom Components"</h3>
                
                {move || {
                    if !error_msg.get().is_empty() {
                        view! { <div style="color:red;">{error_msg.get()}</div> }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }
                }}
                
                <input
                    type="text"
                    placeholder="Filter..."
                    style="width:100%;margin-bottom:8px;"
                    prop:value=move || filter_query.get()
                    on:input=move |ev| filter_query.set(event_target_value(&ev))
                />
                
                <div>
                    {move || {
                        let query = filter_query.get().to_lowercase();
                        app_state.ui.custom_components.get()
                            .into_iter()
                            .enumerate()
                            .filter(|(_, c)| query.is_empty() || c.name.to_lowercase().contains(&query))
                            .map(|(idx, comp)| {
                                view! {
                                    <div style="display:flex;justify-content:space-between;margin-bottom:4px;">
                                        <span>{comp.name.clone()}</span>
                                        <div>
                                            <button on:click=move |_| start_edit_custom_component(idx)>"Edit"</button>
                                            <button on:click=move |_| delete_custom_component(idx)>"Del"</button>
                                        </div>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
                
                <button on:click=move |_| show_add_form.update(|v| *v = !*v)>
                    {move || if show_add_form.get() { "Cancel" } else { "+ Add" }}
                </button>
                
                {move || {
                    if show_add_form.get() {
                        view! {
                            <div style="margin-top:8px;padding:8px;border:1px solid #ddd;">
                                <input
                                    type="text"
                                    placeholder="Name"
                                    style="width:100%;"
                                    prop:value=move || new_name.get()
                                    on:input=move |ev| new_name.set(event_target_value(&ev))
                                />
                                <textarea
                                    placeholder="Template"
                                    style="width:100%;height:60px;"
                                    prop:value=move || new_template.get()
                                    on:input=move |ev| new_template.set(event_target_value(&ev))
                                />
                                <button on:click=add_custom_component>"Add"</button>
                            </div>
                        }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }
                }}
                
                {move || {
                    if editing_idx.get().is_some() {
                        view! {
                            <div style="margin-top:8px;padding:8px;border:1px solid #dd0;">
                                <h4>"Edit"</h4>
                                <input
                                    type="text"
                                    style="width:100%;"
                                    prop:value=move || edit_name.get()
                                    on:input=move |ev| edit_name.set(event_target_value(&ev))
                                />
                                <textarea
                                    style="width:100%;height:60px;"
                                    prop:value=move || edit_template.get()
                                    on:input=move |ev| edit_template.set(event_target_value(&ev))
                                />
                                <button on:click=save_edit_custom_component>"Save"</button>
                                <button on:click=cancel_edit_custom_component>"Cancel"</button>
                            </div>
                        }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }
                }}
            </div>
        </aside>
    }
}
