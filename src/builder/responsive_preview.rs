use crate::state::{AppState, ResponsiveMode};
use leptos::prelude::*;

/// Viewport dimensions for different device types
#[derive(Clone, Debug)]
pub struct ViewportSize {
    pub width: u32,
    pub height: u32,
    pub label: &'static str,
}

impl ViewportSize {
    pub fn mobile() -> Self {
        Self {
            width: 375,
            height: 667,
            label: "Mobile (375px)",
        }
    }

    pub fn tablet() -> Self {
        Self {
            width: 768,
            height: 1024,
            label: "Tablet (768px)",
        }
    }

    pub fn desktop() -> Self {
        Self {
            width: 1920,
            height: 1080,
            label: "Desktop (1920px)",
        }
    }

    pub fn from_mode(mode: ResponsiveMode) -> Self {
        match mode {
            ResponsiveMode::Mobile => Self::mobile(),
            ResponsiveMode::Tablet => Self::tablet(),
            ResponsiveMode::Desktop => Self::desktop(),
        }
    }
}

/// Responsive Preview Controls
/// Displays preset device options and allows switching between them
#[component]
pub fn ResponsivePreviewControls() -> impl IntoView {
    let app_state = AppState::use_context();
    let responsive_mode = app_state.ui.responsive_mode;

    view! {
        <div class="responsive-controls">
            <span class="responsive-label">Viewport:</span>
            <button
                class={move || {
                    let is_active = responsive_mode.get() == ResponsiveMode::Mobile;
                    format!("responsive-btn {}", if is_active { "active" } else { "" })
                }}
                on:click=move |_| {
                    responsive_mode.set(ResponsiveMode::Mobile);
                    app_state.ui.notification.set(Some(crate::state::app_state::Notification::info(
                        "📱 Mobile preview".to_string()
                    )));
                }
            >
                "📱 Mobile"
            </button>
            <button
                class={move || {
                    let is_active = responsive_mode.get() == ResponsiveMode::Tablet;
                    format!("responsive-btn {}", if is_active { "active" } else { "" })
                }}
                on:click=move |_| {
                    responsive_mode.set(ResponsiveMode::Tablet);
                    app_state.ui.notification.set(Some(crate::state::app_state::Notification::info(
                        "📱 Tablet preview".to_string()
                    )));
                }
            >
                "📱 Tablet"
            </button>
            <button
                class={move || {
                    let is_active = responsive_mode.get() == ResponsiveMode::Desktop;
                    format!("responsive-btn {}", if is_active { "active" } else { "" })
                }}
                on:click=move |_| {
                    responsive_mode.set(ResponsiveMode::Desktop);
                    app_state.ui.notification.set(Some(crate::state::app_state::Notification::info(
                        "🖥️ Desktop preview".to_string()
                    )));
                }
            >
                "🖥️ Desktop"
            </button>
        </div>
    }
}

/// Canvas Viewport Wrapper
/// Applies viewport constraints and styling based on the selected responsive mode
#[component]
pub fn CanvasViewport(children: Children) -> impl IntoView {
    let app_state = AppState::use_context();
    let responsive_mode = app_state.ui.responsive_mode;

    view! {
        <div
            class="canvas-viewport-wrapper"
            style={move || {
                let viewport = ViewportSize::from_mode(responsive_mode.get());
                format!(
                    "width: {}px; height: {}px;",
                    viewport.width, viewport.height
                )
            }}
        >
            <div class="canvas-viewport">
                {children()}
            </div>
        </div>
    }
}

/// Responsive Preview Indicator
/// Shows the current viewport size and dimensions
#[component]
pub fn ResponsiveIndicator() -> impl IntoView {
    let app_state = AppState::use_context();
    let responsive_mode = app_state.ui.responsive_mode;

    view! {
        <div class="responsive-indicator">
            {move || {
                let viewport = ViewportSize::from_mode(responsive_mode.get());
                format!("{} ({}x{})", viewport.label, viewport.width, viewport.height)
            }}
        </div>
    }
}
