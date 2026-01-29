use crate::services::project_manager::{ProjectManager, ProjectMetadata};
use crate::state::app_state::{AppState, Notification};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let app_state = AppState::expect_context();
    let projects = RwSignal::new(Vec::<ProjectMetadata>::new());
    let loading = RwSignal::new(true);
    let editing_id = RwSignal::new(None::<String>);
    let edit_name = RwSignal::new(String::new());

    let refresh_projects = move || {
        loading.set(true);
        leptos::task::spawn_local(async move {
            match ProjectManager::list_projects().await {
                Ok(list) => projects.set(list),
                Err(e) => app_state.ui.notify(Notification::error(e.user_message())),
            }
            loading.set(false);
        });
    };

    // Load on mount
    Effect::new(move |_| {
        refresh_projects();
    });

    let on_delete = move |id: String| {
        if !window()
            .confirm_with_message("Are you sure you want to delete this project?")
            .unwrap_or(false)
        {
            return;
        }

        leptos::task::spawn_local(async move {
            if let Err(e) = ProjectManager::delete_project(&id).await {
                app_state.ui.notify(Notification::error(e.user_message()));
            } else {
                app_state
                    .ui
                    .notify(Notification::success("Project deleted".to_string()));
                refresh_projects();
            }
        });
    };

    let on_new = move |_| {
        app_state.create_new_project();
        // We need to save it to get an ID, then navigate
        // Or we can just navigate to a new ID and let the editor handle creation on save?
        // AppState::create_new_project clears the state.
        // But to navigate to /editor/:id we need an ID.
        // Let's generate one.
        let new_id = ProjectManager::generate_id();
        app_state.current_project_id.set(Some(new_id.clone()));

        // We should probably save the empty project so it exists?
        // Or we just navigate and let the user save later.
        // If we just navigate, `load_project` might fail (404).
        // So we should probably initialize it.

        let navigate = leptos_router::hooks::use_navigate();
        navigate(
            &format!("/editor/{}", new_id),
            Default::default(),
        );

        app_state
            .ui
            .notify(Notification::success("New project created".to_string()));
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

        leptos::task::spawn_local(async move {
            if let Err(e) = ProjectManager::rename_project(&id, &new_name).await {
                app_state.ui.notify(Notification::error(e.user_message()));
            } else {
                app_state
                    .ui
                    .notify(Notification::success("Project renamed".to_string()));
                editing_id.set(None);
                refresh_projects();
            }
        });
    };

    let cancel_rename = move |_| {
        editing_id.set(None);
    };

    view! {
        <div class="dashboard-page">
            <header class="dashboard-header">
                <div class="header-content">
                    <h1>"Leptos Studio"</h1>
                    <div class="header-actions">
                        <button class="btn btn-primary" on:click=on_new>
                            <span class="icon">"+"</span> "New Project"
                        </button>
                    </div>
                </div>
            </header>

            <main class="dashboard-content">
                <div class="projects-section">
                    <div class="section-header">
                        <h2>"My Projects"</h2>
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
                                <button class="btn btn-primary mt-4" on:click=on_new>"Create Project"</button>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="project-grid">
                                {projects.get().into_iter().map(|p| {
                                    let id = p.id.clone();
                                    let id_ren = p.id.clone();
                                    let id_del = p.id.clone();

                                    let id_save_key = p.id.clone();
                                    let id_save_btn = p.id.clone();

                                    let name_ren = p.name.clone();
                                    let name_display = p.name.clone();
                                    let name_title = p.name.clone();

                                    let is_editing = editing_id.get().as_deref() == Some(&id);

                                    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(p.last_modified));
                                    let date_str = format!("{}/{}/{}",
                                        date.get_month() + 1,
                                        date.get_date(),
                                        date.get_full_year()
                                    );

                                    let editor_url = format!("/editor/{}", id);

                                    view! {
                                        <div class="project-card">
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
                                                            on:click=move |ev| ev.stop_propagation()
                                                        />
                                                        <div class="btn-group btn-group-sm">
                                                            <button class="btn btn-sm btn-primary" on:click=move |ev| { ev.stop_propagation(); save_rename(id_save_btn.clone()); }>"Save"</button>
                                                            <button class="btn btn-sm btn-ghost" on:click=move |ev| { ev.stop_propagation(); cancel_rename(ev); }>"Cancel"</button>
                                                        </div>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="project-card-header">
                                                        <h3 title=name_title>{name_display}</h3>
                                                    </div>
                                                }.into_any()
                                            }}

                                            <A href=editor_url attr:class="project-card-link">
                                                <div class="project-preview">
                                                    // Placeholder for preview image
                                                    <div class="preview-placeholder">"UI"</div>
                                                </div>
                                            </A>

                                            <div class="project-meta">
                                                <div>{format!("{} components", p.component_count)}</div>
                                                <div>{format!("Last modified: {}", date_str)}</div>
                                            </div>

                                            <div class="project-actions">
                                                <A href=format!("/editor/{}", id) attr:class="btn btn-sm btn-outline">"Open"</A>
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
            </main>
        </div>
    }
}
