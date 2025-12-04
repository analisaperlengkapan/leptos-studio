//! Template Gallery Panel
//!
//! A visual gallery for browsing and applying pre-built templates
//! to the canvas. Supports search, filtering by category, and preview.

use leptos::prelude::*;

use crate::services::{Template, TemplateCategory, TemplateService};
use crate::state::app_state::{AppState, Notification};

/// Template gallery panel component with callbacks
#[component]
pub fn TemplateGallery(
    /// Callback when gallery is closed
    #[prop(into)]
    on_close: Callback<()>,
    /// Callback when a template is applied
    #[prop(into)]
    on_apply: Callback<Template>,
) -> impl IntoView {
    let _app_state = AppState::use_context();
    
    // Local state
    let search_query = RwSignal::new(String::new());
    let selected_category = RwSignal::new(None::<TemplateCategory>);
    let preview_template = RwSignal::new(None::<Template>);

    // Filtered templates
    let filtered_templates = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        let category = selected_category.get();
        
        let mut templates = if query.is_empty() {
            if let Some(cat) = category {
                TemplateService::templates_by_category(cat)
            } else {
                TemplateService::builtin_templates()
            }
        } else {
            TemplateService::search_templates(&query)
        };

        // Filter by category if both search and category are set
        if let Some(cat) = category {
            if !query.is_empty() {
                templates.retain(|t| t.category == cat);
            }
        }

        templates
    });

    // Category button helper
    let category_button = move |cat: Option<TemplateCategory>, label: &'static str| {
        let is_active = Memo::new(move |_| selected_category.get() == cat);
        
        view! {
            <button
                class=move || if is_active.get() { "category-btn active" } else { "category-btn" }
                on:click=move |_| selected_category.set(cat)
            >
                {label}
            </button>
        }
    };

    view! {
        <div class="template-gallery-overlay" role="dialog" aria-modal="true" aria-labelledby="template-gallery-title">
            <div class="template-gallery-panel">
                <div class="template-gallery-header">
                    <h3 id="template-gallery-title">"Template Gallery"</h3>
                    <button
                        class="close-btn"
                        on:click=move |_| on_close.run(())
                        aria-label="Close gallery"
                    >
                        "×"
                    </button>
                </div>

                <div class="template-search">
                    <input
                        type="text"
                        placeholder="🔍 Search templates..."
                        class="template-search-input"
                        prop:value=move || search_query.get()
                        on:input=move |ev| search_query.set(event_target_value(&ev))
                    />
                </div>

                <div class="template-categories">
                    {category_button(None, "All")}
                    {category_button(Some(TemplateCategory::Form), "Forms")}
                    {category_button(Some(TemplateCategory::Hero), "Hero")}
                    {category_button(Some(TemplateCategory::Navigation), "Navigation")}
                    {category_button(Some(TemplateCategory::Card), "Cards")}
                    {category_button(Some(TemplateCategory::Dashboard), "Dashboard")}
                    {category_button(Some(TemplateCategory::Footer), "Footer")}
                </div>

                <div class="template-grid">
                    <For
                        each=move || filtered_templates.get()
                        key=|template| template.id.clone()
                        children=move |template| {
                            let template_for_preview = template.clone();
                            let template_for_apply = template.clone();
                            let template_name = template.name.clone();
                            let template_desc = template.description.clone();
                            let template_tags = template.tags.clone();

                            view! {
                                <div
                                    class="template-card"
                                    on:mouseenter=move |_| preview_template.set(Some(template_for_preview.clone()))
                                    on:mouseleave=move |_| preview_template.set(None)
                                >
                                    <div class="template-card-preview">
                                        <div class="template-icon">
                                            {category_icon(&template.category)}
                                        </div>
                                    </div>
                                    <div class="template-card-content">
                                        <h4 class="template-name">{template_name}</h4>
                                        <p class="template-description">{template_desc}</p>
                                        <div class="template-tags">
                                            {template_tags.into_iter().take(3).map(|tag| {
                                                view! { <span class="template-tag">{tag}</span> }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                    <button
                                        class="btn btn-primary template-apply-btn"
                                        on:click={
                                            let template_clone = template_for_apply.clone();
                                            move |_| on_apply.run(template_clone.clone())
                                        }
                                    >
                                        "Apply"
                                    </button>
                                </div>
                            }
                        }
                    />
                </div>

                {move || {
                    if filtered_templates.get().is_empty() {
                        view! {
                            <div class="template-empty-state">
                                <p>"No templates found"</p>
                                <p class="template-empty-hint">"Try a different search or category"</p>
                            </div>
                        }.into_any()
                    } else {
                        ().into_any()
                    }
                }}
            </div>
        </div>
    }
}

/// Standalone template gallery toggle button with built-in panel
#[component]
pub fn TemplateGalleryToggle() -> impl IntoView {
    let app_state = AppState::use_context();
    let show_gallery = RwSignal::new(false);

    view! {
        <div class="template-gallery-wrapper">
            <button
                class="btn btn-secondary template-gallery-toggle"
                on:click=move |_| show_gallery.update(|v| *v = !*v)
            >
                {move || if show_gallery.get() { "📁 Close Templates" } else { "📁 Templates" }}
            </button>

            {move || {
                if show_gallery.get() {
                    view! {
                        <TemplateGallery
                            on_close=move || show_gallery.set(false)
                            on_apply=move |template: Template| {
                                app_state.canvas.record_snapshot();
                                for component in template.components {
                                    app_state.canvas.add_component(component);
                                }
                                app_state.ui.notification.set(Some(Notification::success(
                                    format!("✅ Template '{}' applied!", template.name),
                                )));
                                show_gallery.set(false);
                            }
                        />
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}

/// Get icon for template category
fn category_icon(category: &TemplateCategory) -> &'static str {
    match category {
        TemplateCategory::Form => "📝",
        TemplateCategory::Hero => "🦸",
        TemplateCategory::Navigation => "🧭",
        TemplateCategory::Card => "🃏",
        TemplateCategory::Dashboard => "📊",
        TemplateCategory::Footer => "🦶",
        TemplateCategory::LandingPage => "🏠",
        TemplateCategory::Custom => "⚙️",
    }
}
