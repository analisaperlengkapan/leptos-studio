use leptos::prelude::*;
use crate::state::app_state::{AppState, Notification};
use crate::services::project_manager::{ProjectManager, ProjectMetadata};

#[component]
pub fn ProjectDashboard() -> impl IntoView {
    let app_state = AppState::expect_context();
    let projects = RwSignal::new(Vec::<ProjectMetadata>::new());
    let loading = RwSignal::new(true);
    let editing_id = RwSignal::new(None::<String>);
    let edit_name = RwSignal::new(String::new());

    let refresh_projects = move || {
        loading.set(true);
        match ProjectManager::list_projects() {
            Ok(list) => projects.set(list),
            Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
        }
        loading.set(false);
    };

    // Load on mount (or when visibility changes to true)
    let show = app_state.ui.show_project_dashboard;
    Effect::new(move |_| {
        if show.get() {
            refresh_projects();
        }
    });

    let close_dashboard = move || {
        app_state.ui.show_project_dashboard.set(false);
        editing_id.set(None);
    };

    let on_close_click = move |_| {
        close_dashboard();
    };

    let on_open = move |id: String| {
        if let Err(e) = app_state.load_project(&id) {
             app_state.ui.notify(Notification::error(e.user_message()));
        } else {
             app_state.ui.notify(Notification::success("Project loaded".to_string()));
             close_dashboard();
        }
    };

    let on_delete = move |id: String| {
        if !window().confirm_with_message("Are you sure you want to delete this project?").unwrap_or(false) {
            return;
        }

        if let Err(e) = ProjectManager::delete_project(&id) {
             app_state.ui.notify(Notification::error(e.user_message()));
        } else {
             app_state.ui.notify(Notification::success("Project deleted".to_string()));
             refresh_projects();

             // If we deleted the current project, reset
             if let Some(curr) = app_state.current_project_id.get() {
                 if curr == id {
                     app_state.create_new_project();
                 }
             }
        }
    };

    let on_new = move |_| {
        app_state.create_new_project();
        close_dashboard();
        app_state.ui.notify(Notification::success("New project created".to_string()));
    };

    let start_rename = move |id: String, current_name: String| {
        editing_id.set(Some(id));
        edit_name.set(current_name);
    };

    let save_rename = move |id: String| {
        let new_name = edit_name.get();
        if new_name.trim().is_empty() {
            return;
        }

        if let Err(e) = ProjectManager::rename_project(&id, &new_name) {
            app_state.ui.notify(Notification::error(e.user_message()));
        } else {
            app_state.ui.notify(Notification::success("Project renamed".to_string()));
            editing_id.set(None);
            refresh_projects();

            // If renaming current project, update app state name
            if app_state.current_project_id.get().as_deref() == Some(&id) {
                app_state.project_name.set(new_name);
            }
        }
    };

    let cancel_rename = move |_| {
        editing_id.set(None);
    };

    view! {
        {move || if show.get() {
            view! {
                <div class="welcome-modal-overlay">
                    <div class="welcome-modal-content" style="max-width: 900px;">
                        <div class="welcome-header">
                            <div class="header-left">
                                <h2>"My Projects"</h2>
                            </div>
                            <button class="btn btn-ghost" on:click=on_close_click>"✕"</button>
                        </div>

                        <div class="welcome-body" style="background: var(--color-gray-50); min-height: 400px;">
                            <div style="margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center;">
                                <button class="btn btn-primary" on:click=on_new>
                                    <span class="icon">"+"</span> "New Project"
                                </button>
                                <span class="text-sm text-gray-500">
                                    {move || format!("{} Projects", projects.get().len())}
                                </span>
                            </div>

                            {move || if loading.get() {
                                view! { <div class="loading-state">"Loading projects..."</div> }.into_any()
                            } else if projects.get().is_empty() {
                                view! {
                                    <div class="empty-projects">
                                        <div style="font-size: 48px; margin-bottom: 16px;">"📂"</div>
                                        <h3>"No projects yet"</h3>
                                        <p>"Create your first project to get started building!"</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="project-grid">
                                        {projects.get().into_iter().map(|p| {
                                            let id = p.id.clone();
                                            let id_act = p.id.clone();
                                            let id_ren = p.id.clone();
                                            let id_del = p.id.clone();

                                            let id_save_key = p.id.clone();
                                            let id_save_btn = p.id.clone();

                                            let name_ren = p.name.clone();
                                            let name_display = p.name.clone();
                                            let name_title = p.name.clone();

                                            let is_current = app_state.current_project_id.get().as_deref() == Some(&id);
                                            let is_editing = editing_id.get().as_deref() == Some(&id);

                                            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(p.last_modified));
                                            let date_str = format!("{}/{}/{}",
                                                date.get_month() + 1,
                                                date.get_date(),
                                                date.get_full_year()
                                            );

                                            view! {
                                                <div class=format!("project-card {}", if is_current { "active" } else { "" })>
                                                    {if is_editing {
                                                        view! {
                                                            <div class="rename-container" style="margin-bottom: 8px;">
                                                                <input
                                                                    type="text"
                                                                    class="rename-input"
                                                                    prop:value=edit_name
                                                                    on:input=move |ev| edit_name.set(event_target_value(&ev))
                                                                    on:keydown=move |ev| {
                                                                        if ev.key() == "Enter" {
                                                                            save_rename(id_save_key.clone());
                                                                        } else if ev.key() == "Escape" {
                                                                            editing_id.set(None);
                                                                        }
                                                                    }
                                                                    autofocus
                                                                />
                                                                <div class="btn-group btn-group-sm">
                                                                    <button class="btn btn-sm btn-primary" on:click=move |_| save_rename(id_save_btn.clone())>"Save"</button>
                                                                    <button class="btn btn-sm btn-ghost" on:click=cancel_rename>"Cancel"</button>
                                                                </div>
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="project-card-header">
                                                                <h3 title=name_title>{name_display}</h3>
                                                                {if is_current {
                                                                    view! { <span class="badge badge-primary" style="font-size: 10px; padding: 2px 6px; background: var(--color-primary); color: white; border-radius: 4px;">"Active"</span> }.into_any()
                                                                } else {
                                                                    view! { <span></span> }.into_any()
                                                                }}
                                                            </div>
                                                        }.into_any()
                                                    }}

                                                    <div class="project-meta">
                                                        <div>{format!("{} components", p.component_count)}</div>
                                                        <div>{format!("Last modified: {}", date_str)}</div>
                                                    </div>

                                                    <div class="project-actions">
                                                        <button class="btn btn-sm btn-outline" on:click=move |_| on_open(id_act.clone())>
                                                            {if is_current { "Resume" } else { "Open" }}
                                                        </button>
                                                        <button class="btn btn-sm btn-ghost" on:click=move |_| start_rename(id_ren.clone(), name_ren.clone())>
                                                            "Rename"
                                                        </button>
                                                        <button class="btn btn-sm btn-ghost" style="color: var(--color-error-500);" on:click=move |_| on_delete(id_del.clone())>
                                                            "Delete"
                                                        </button>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }}
                        </div>
                    </div>
                </div>
            }.into_any()
        } else {
            view! { <div style="display:none"></div> }.into_any()
        }}
    }
}
