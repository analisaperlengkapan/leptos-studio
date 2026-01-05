use leptos::prelude::*;

use crate::builder::accessibility::{AccessibilityProvider, SkipLink, announce};
use crate::builder::breadcrumb::BreadcrumbNavigation;
use crate::builder::canvas::Canvas;
use crate::builder::code_panel::CodePanel;
use crate::builder::command_palette::CommandPalette;
use crate::builder::component_palette::ComponentPalette;
use crate::builder::design_tokens::{DesignTokenProvider, DesignTokens};
use crate::builder::drag_drop::DragPreview;
use crate::builder::export_modal::ExportModal;
use crate::builder::git_panel::GitPanel;
use crate::builder::hooks::use_export_actions::use_export_actions;
use crate::builder::hooks::use_keyboard_actions::use_keyboard_actions;
use crate::builder::keyboard::{KeyboardHandler, get_default_shortcuts};
use crate::builder::preview::Preview;
use crate::builder::property_editor::PropertyEditor;
use crate::builder::responsive_preview::{CanvasViewport, ResponsivePreviewControls};
use crate::builder::snackbar::Snackbar;
use crate::builder::status_bar::StatusBar;
use crate::builder::template_gallery::TemplateGallery;
use crate::builder::toolbar::Toolbar;
use crate::services::analytics_service::AnalyticsService;
use crate::services::event_bus::EventBus;
use crate::services::template_service::TemplateService;
use crate::state::app_state::{AppState, Notification};
use crate::state::derived::DerivedState;

#[component]
pub fn App() -> impl IntoView {
    // Initialize global AppState context
    AppState::provide_context();
    let app_state = AppState::expect_context();

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
    let keyboard_action_handler = use_keyboard_actions(
        show_export.write_only(),
        export_code.write_only(),
        show_template_gallery.write_only(),
    );

    // Right panel tabs state
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum RightPanelTab {
        Properties,
        Git,
        Code,
    }

    let active_right_tab = RwSignal::new(RightPanelTab::Properties);

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

                    <Toolbar
                        show_template_gallery=show_template_gallery.write_only()
                        show_export=show_export.write_only()
                        export_code=export_code.write_only()
                        export_template=export_template.read_only()
                    />

                    <BreadcrumbNavigation />

                    <div class="app-layout">
                        <aside class="sidebar-panel" role="navigation" aria-label="Component library">
                            <ComponentPalette />
                        </aside>

                        <main role="main">
                            <nav class="main-nav" aria-label="Main actions">
                                <ResponsivePreviewControls />
                            </nav>
                            <div class="main-content">
                                <section id="main-canvas" class="canvas-area" role="region" aria-label="Design canvas">
                                    <CanvasViewport>
                                        <Canvas />
                                    </CanvasViewport>
                                </section>

                                <aside class="property-panel" role="complementary" aria-label="Right Panel">
                                    <div class="panel-tabs">
                                        <button
                                            class=move || if active_right_tab.get() == RightPanelTab::Properties { "tab active" } else { "tab" }
                                            on:click=move |_| active_right_tab.set(RightPanelTab::Properties)
                                        >
                                            "Properties"
                                        </button>
                                        <button
                                            class=move || if active_right_tab.get() == RightPanelTab::Code { "tab active" } else { "tab" }
                                            on:click=move |_| active_right_tab.set(RightPanelTab::Code)
                                        >
                                            "Code"
                                        </button>
                                        <button
                                            class=move || if active_right_tab.get() == RightPanelTab::Git { "tab active" } else { "tab" }
                                            on:click=move |_| active_right_tab.set(RightPanelTab::Git)
                                        >
                                            "Git"
                                        </button>
                                    </div>

                                    <div class="panel-content">
                                        {move || match active_right_tab.get() {
                                            RightPanelTab::Git => view! { <GitPanel /> }.into_any(),
                                            RightPanelTab::Code => view! { <CodePanel /> }.into_any(),
                                            RightPanelTab::Properties => view! {
                                                <div class="property-editor-container">
                                                    <PropertyEditor />
                                                    <div class="preview-section-min">
                                                        <Preview />
                                                    </div>
                                                </div>
                                            }.into_any()
                                        }}
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
                                    app_state.canvas.record_snapshot(&format!("Apply Template: {}", template.name));
                                    let comp_count = template.components.len();
                                    let template_name = template.name.clone();
                                    for comp in template.components {
                                        app_state.canvas.add_component_without_snapshot(comp);
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
