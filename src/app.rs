use leptos::prelude::*;

use crate::builder::accessibility::{AccessibilityProvider, SkipLink, announce};
use crate::builder::breadcrumb::BreadcrumbNavigation;
use crate::builder::canvas::Canvas;
use crate::builder::command_palette::CommandPalette;
use crate::builder::design_tokens::{DesignTokenProvider, DesignTokens};
use crate::builder::drag_drop::DragPreview;
use crate::builder::export_modal::ExportModal;
use crate::builder::hooks::use_keyboard_actions::use_keyboard_actions;
use crate::builder::hooks::use_export_actions::use_export_actions;
use crate::builder::keyboard::{KeyboardHandler, get_default_shortcuts};
use crate::builder::preview::Preview;
use crate::builder::property_editor::PropertyEditor;
use crate::builder::responsive_preview::{CanvasViewport, ResponsivePreviewControls};
use crate::builder::sidebar::Sidebar;
use crate::builder::snackbar::Snackbar;
use crate::builder::status_bar::StatusBar;
use crate::builder::template_gallery::TemplateGallery;
use crate::services::analytics_service::AnalyticsService;
use crate::services::event_bus::EventBus;
use crate::services::template_service::TemplateService;
use crate::state::app_state::{AppState, Notification};
use crate::state::derived::DerivedState;

#[component]
pub fn App() -> impl IntoView {
    // Initialize global AppState context
    AppState::provide_context();
    let app_state = AppState::use_context();

    // Initialize services
    let _event_bus = StoredValue::new(EventBus::new());
    let _template_service = StoredValue::new(TemplateService::new());
    let _analytics_service = StoredValue::new(AnalyticsService::new());

    // Create and provide derived state for memoized computations
    DerivedState::provide_context(app_state);

    // Design tokens
    let design_tokens = RwSignal::new(DesignTokens::default());

    // Export modal (local UI state)
    let show_export = RwSignal::new(false);
    let export_code = RwSignal::new(String::new());
    let export_template = RwSignal::new("leptos".to_string());

    // Template gallery visibility
    let show_template_gallery = RwSignal::new(false);

    // Keyboard action handler
    let keyboard_action_handler = use_keyboard_actions(show_export.write_only(), export_code.write_only(), show_template_gallery.write_only());

    // Export handler
    let do_export = use_export_actions(show_export.write_only(), export_code.write_only(), export_template.read_only());

    // Save/Load handlers
    let save_layout = move |_| {
        if let Err(e) = app_state.save() {
            app_state
                .ui
                .notification
                .set(Some(Notification::error(format!(
                    "❌ {}",
                    e.user_message()
                ))));
        } else {
            app_state
                .ui
                .notification
                .set(Some(Notification::success("💾 Layout saved!".to_string())));
        }
    };

    let load_layout = move |_| {
        if let Err(e) = app_state.load() {
            app_state
                .ui
                .notification
                .set(Some(Notification::error(format!(
                    "❌ {}",
                    e.user_message()
                ))));
        } else {
            app_state
                .ui
                .notification
                .set(Some(Notification::success("📂 Layout loaded!".to_string())));
        }
    };

    // Undo/Redo handlers
    let do_undo = move |_| {
        if let Some(snapshot) = app_state.canvas.history.write().undo() {
            app_state.canvas.components.set(snapshot.components);
            app_state.canvas.selected.set(snapshot.selected);
            app_state
                .ui
                .notification
                .set(Some(Notification::info("↩️ Undo".to_string())));
        }
    };

    let do_redo = move |_| {
        if let Some(snapshot) = app_state.canvas.history.write().redo() {
            app_state.canvas.components.set(snapshot.components);
            app_state.canvas.selected.set(snapshot.selected);
            app_state
                .ui
                .notification
                .set(Some(Notification::info("↪️ Redo".to_string())));
        }
    };

    view! {
        <DesignTokenProvider tokens=design_tokens>
            <AccessibilityProvider>
                <SkipLink target="#main-canvas" label="Skip to canvas" />
                <div class="leptos-studio" tabindex="0" role="application" aria-label="Leptos Studio Visual Builder">
                    <KeyboardHandler
                        shortcuts=get_default_shortcuts()
                        on_action=keyboard_action_handler.clone()
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
                        <button
                            class="btn btn-outline btn-sm"
                            on:click=move |_| show_template_gallery.set(true)
                            aria-label="Open template gallery"
                        >
                            {"📑 Templates"}
                        </button>
                    </header>

                    <BreadcrumbNavigation />

                    <div class="app-layout">
                        <aside class="sidebar-panel" role="navigation" aria-label="Component library">
                            <Sidebar />
                        </aside>
                        <main role="main">
                            <nav class="main-nav" aria-label="Main actions">
                                <div class="nav-actions">
                                    <button on:click=save_layout class="btn btn-primary" aria-label="Save layout">{"Save"}</button>
                                    <button on:click=load_layout class="btn btn-secondary" aria-label="Load layout">{"Load"}</button>
                                    <button on:click=do_export class="btn btn-success" aria-label="Export code">{"Export"}</button>
                                    <button
                                        on:click=do_undo
                                        class="btn btn-outline"
                                        aria-label="Undo last action"
                                    >{"Undo"}</button>
                                    <button
                                        on:click=do_redo
                                        class="btn btn-outline"
                                        aria-label="Redo last action"
                                    >{"Redo"}</button>
                                </div>
                                <ResponsivePreviewControls />
                            </nav>
                            <div class="main-content">
                                <section id="main-canvas" class="canvas-area" role="region" aria-label="Design canvas">
                                    <CanvasViewport>
                                        <Canvas />
                                    </CanvasViewport>
                                </section>
                                <aside class="property-panel" role="complementary" aria-label="Property editor">
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

                    <StatusBar />

                    {move || if show_template_gallery.get() {
                        view! {
                            <TemplateGallery
                                on_close=move || show_template_gallery.set(false)
                                on_apply=move |template: crate::services::Template| {
                                    app_state.canvas.record_snapshot();
                                    let comp_count = template.components.len();
                                    let template_name = template.name.clone();
                                    for comp in template.components {
                                        app_state.canvas.add_component(comp);
                                    }
                                    show_template_gallery.set(false);
                                    app_state.ui.notification.set(Some(Notification::success(
                                        format!("✨ Template '{}' applied!", template_name)
                                    )));
                                    announce(&format!("Template {} applied with {} components", template_name, comp_count));
                                }
                            />
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}

                {move || if show_export.get() {
                    view! {
                        <ExportModal
                            show=show_export
                            code=export_code
                            format=export_template
                            on_close=Callback::new(move |_| show_export.set(false))
                            notification_signal=app_state.ui.notification
                        />
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                    <Snackbar notification=app_state.ui.notification />
                </div>
            </AccessibilityProvider>
        </DesignTokenProvider>
    }
}
