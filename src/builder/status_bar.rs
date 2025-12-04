//! Status Bar Component
//!
//! A status bar at the bottom of the application showing current state,
//! component count, responsive mode, and other useful information.

use leptos::prelude::*;

use crate::state::{AppState, DerivedState};

/// Status bar component
#[component]
pub fn StatusBar() -> impl IntoView {
    let app_state = AppState::use_context();
    
    // Use derived state from context (provided by App)
    let derived = DerivedState::use_context();

    view! {
        <footer class="status-bar" role="status" aria-live="polite">
            <div class="status-bar-left">
                // Component count
                <span class="status-item" title="Total components">
                    <span class="status-icon">"📦"</span>
                    <span class="status-text">
                        {move || format!("{} components", derived.component_count.get())}
                    </span>
                </span>

                // Nesting depth indicator
                {move || {
                    let depth = derived.max_nesting_depth.get();
                    if depth > 0 {
                        view! {
                            <span class="status-item" title="Maximum nesting depth">
                                <span class="status-icon">"📊"</span>
                                <span class="status-text">{format!("Depth: {}", depth)}</span>
                            </span>
                        }.into_any()
                    } else {
                        ().into_any()
                    }
                }}

                // Selection status
                {move || {
                    if let Some(comp) = derived.selected_component.get() {
                        let type_name = format!("{:?}", comp.component_type());
                        view! {
                            <span class="status-item status-selected" title="Selected component">
                                <span class="status-icon">"✓"</span>
                                <span class="status-text">{type_name}</span>
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <span class="status-item status-no-selection" title="No selection">
                                <span class="status-text">"No selection"</span>
                            </span>
                        }.into_any()
                    }
                }}
            </div>

            <div class="status-bar-center">
                // Undo/Redo availability
                <span class="status-item">
                    <button
                        class="status-btn"
                        disabled=move || !derived.can_undo.get()
                        title="Undo (Ctrl+Z)"
                        on:click=move |_| {
                            if let Some(snapshot) = app_state.canvas.history.write().undo() {
                                app_state.canvas.apply_snapshot(&snapshot);
                            }
                        }
                    >
                        "↩️"
                    </button>
                    <button
                        class="status-btn"
                        disabled=move || !derived.can_redo.get()
                        title="Redo (Ctrl+Y)"
                        on:click=move |_| {
                            if let Some(snapshot) = app_state.canvas.history.write().redo() {
                                app_state.canvas.apply_snapshot(&snapshot);
                            }
                        }
                    >
                        "↪️"
                    </button>
                </span>
            </div>

            <div class="status-bar-right">
                // Responsive mode indicator
                <span class="status-item" title="Current responsive mode">
                    <span class="status-icon">
                        {move || {
                            match app_state.ui.responsive_mode.get() {
                                crate::state::ResponsiveMode::Desktop => "🖥️",
                                crate::state::ResponsiveMode::Tablet => "📱",
                                crate::state::ResponsiveMode::Mobile => "📲",
                            }
                        }}
                    </span>
                    <span class="status-text">
                        {move || format!("{:?}", app_state.ui.responsive_mode.get())}
                    </span>
                </span>

                // Theme indicator
                <span class="status-item" title="Current theme">
                    <span class="status-icon">
                        {move || {
                            match app_state.settings.with(|s| s.theme.clone()) {
                                crate::state::Theme::Light => "☀️",
                                crate::state::Theme::Dark => "🌙",
                                crate::state::Theme::Custom => "🎨",
                            }
                        }}
                    </span>
                </span>

                // Render metrics (debug)
                <span class="status-item status-metrics" title="Render performance">
                    <span class="status-text">
                        {move || format!("{:.1}ms", app_state.ui.render_time.get())}
                    </span>
                </span>
            </div>
        </footer>
    }
}

/// Mini status indicator for inline use
#[component]
pub fn StatusIndicator(
    /// Label text
    #[prop(into)]
    label: String,
    /// Value to display
    #[prop(into)]
    value: Signal<String>,
    /// Optional icon
    #[prop(optional)]
    icon: Option<&'static str>,
) -> impl IntoView {
    view! {
        <span class="status-indicator">
            {icon.map(|i| view! { <span class="status-indicator-icon">{i}</span> })}
            <span class="status-indicator-label">{label}</span>
            <span class="status-indicator-value">{value}</span>
        </span>
    }
}
