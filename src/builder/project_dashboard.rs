use leptos::prelude::*;
use crate::state::app_state::{AppState, Notification};
use crate::services::project_manager::{ProjectManager, ProjectMetadata};

#[component]
pub fn ProjectDashboard() -> impl IntoView {
    let app_state = AppState::expect_context();
    let projects = RwSignal::new(Vec::<ProjectMetadata>::new());
    let loading = RwSignal::new(true);

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

    view! {
        {move || if show.get() {
            view! {
                <div class="welcome-modal-overlay" style="z-index: 2000;">
                    <div class="welcome-modal-content" style="max-width: 800px; width: 90%;">
                        <div class="welcome-header" style="display: flex; justify-content: space-between; align-items: center;">
                            <h2>"My Projects"</h2>
                            <button class="btn btn-ghost" on:click=on_close_click>"✕"</button>
                        </div>

                        <div class="welcome-body" style="max-height: 60vh; overflow-y: auto;">
                            <div class="project-actions" style="margin-bottom: 20px;">
                                <button class="btn btn-primary" on:click=on_new>
                                    "+ New Project"
                                </button>
                            </div>

                            {move || if loading.get() {
                                view! { <p>"Loading..."</p> }.into_any()
                            } else if projects.get().is_empty() {
                                view! {
                                    <div class="empty-state" style="text-align: center; padding: 40px; color: #888;">
                                        <p>"No projects found. Create one to get started!"</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="project-grid" style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 16px;">
                                        {projects.get().into_iter().map(|p| {
                                            let id = p.id.clone();
                                            let id_del = p.id.clone();
                                            let is_current = app_state.current_project_id.get().as_deref() == Some(&id);
                                            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(p.last_modified));

                                            // Format date simply
                                            let date_str = format!("{}/{}/{}",
                                                date.get_month() + 1,
                                                date.get_date(),
                                                date.get_full_year()
                                            );

                                            view! {
                                                <div class="project-card" style=format!("border: 1px solid var(--border-color, #ccc); padding: 16px; border-radius: 8px; background: var(--bg-secondary, #fff); position: relative; {}", if is_current { "border-color: var(--primary-color, blue); box-shadow: 0 0 0 2px var(--primary-color, blue);" } else { "" })>
                                                    <h3 style="margin-top: 0;">{p.name}</h3>
                                                    <p style="font-size: 0.8em; color: #666;">
                                                        "Components: " {p.component_count} <br/>
                                                        "Modified: " {date_str}
                                                    </p>

                                                    <div class="card-actions" style="display: flex; gap: 8px; margin-top: 12px;">
                                                        <button class="btn btn-sm btn-outline" on:click=move |_| on_open(id.clone())>
                                                            {if is_current { "Current" } else { "Open" }}
                                                        </button>
                                                        <button class="btn btn-sm btn-ghost" style="color: var(--error-color, red);" on:click=move |_| on_delete(id_del.clone())>
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
