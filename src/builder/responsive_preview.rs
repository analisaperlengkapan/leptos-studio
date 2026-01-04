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
    let app_state = AppState::expect_context();
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
    let app_state = AppState::expect_context();
    let responsive_mode = app_state.ui.responsive_mode;

    view! {
        <div
            class="canvas-viewport-wrapper"
            style={move || {
                let viewport = ViewportSize::from_mode(responsive_mode.get());
                // Use max-width/max-height to keep it responsive within the container if it's too big
                match responsive_mode.get() {
                    ResponsiveMode::Desktop => "width: 100%; height: 100%;".to_string(), // Desktop fills the area
                    _ => format!(
                        "width: {}px; height: {}px; max-width: 100%; max-height: 100%; margin: auto; border: 1px solid #ccc; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);",
                        viewport.width, viewport.height
                    ),
                }
            }}
        >
            <div class="canvas-viewport" style="width: 100%; height: 100%;">
                {children()}
            </div>
        </div>
    }
}

/// Responsive Preview Indicator
/// Shows the current viewport size and dimensions
#[component]
pub fn ResponsiveIndicator() -> impl IntoView {
    let app_state = AppState::expect_context();
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
