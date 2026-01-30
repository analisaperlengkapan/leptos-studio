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

    pub fn mobile_landscape() -> Self {
        Self {
            width: 667,
            height: 375,
            label: "Mobile Landscape (667px)",
        }
    }

    pub fn tablet() -> Self {
        Self {
            width: 768,
            height: 1024,
            label: "Tablet (768px)",
        }
    }

    pub fn tablet_landscape() -> Self {
        Self {
            width: 1024,
            height: 768,
            label: "Tablet Landscape (1024px)",
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
            ResponsiveMode::MobileLandscape => Self::mobile_landscape(),
            ResponsiveMode::Tablet => Self::tablet(),
            ResponsiveMode::TabletLandscape => Self::tablet_landscape(),
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
            <div class="btn-group">
                <button
                    class={move || {
                        let is_active = matches!(responsive_mode.get(), ResponsiveMode::Mobile | ResponsiveMode::MobileLandscape);
                        format!("responsive-btn {}", if is_active { "active" } else { "" })
                    }}
                    on:click=move |_| {
                        responsive_mode.set(ResponsiveMode::Mobile);
                        app_state.ui.notification.set(Some(crate::state::app_state::Notification::info(
                            "📱 Mobile preview".to_string()
                        )));
                    }
                    title="Mobile View"
                >
                    "📱"
                </button>
                <button
                    class={move || {
                        let is_active = matches!(responsive_mode.get(), ResponsiveMode::Tablet | ResponsiveMode::TabletLandscape);
                        format!("responsive-btn {}", if is_active { "active" } else { "" })
                    }}
                    on:click=move |_| {
                        responsive_mode.set(ResponsiveMode::Tablet);
                        app_state.ui.notification.set(Some(crate::state::app_state::Notification::info(
                            "📱 Tablet preview".to_string()
                        )));
                    }
                    title="Tablet View"
                >
                    "📟"
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
                    title="Desktop View"
                >
                    "🖥️"
                </button>
            </div>

            <span class="responsive-sep">"|"</span>

            <button
                class="responsive-btn"
                disabled=move || responsive_mode.get() == ResponsiveMode::Desktop
                on:click=move |_| {
                    match responsive_mode.get() {
                        ResponsiveMode::Mobile => responsive_mode.set(ResponsiveMode::MobileLandscape),
                        ResponsiveMode::MobileLandscape => responsive_mode.set(ResponsiveMode::Mobile),
                        ResponsiveMode::Tablet => responsive_mode.set(ResponsiveMode::TabletLandscape),
                        ResponsiveMode::TabletLandscape => responsive_mode.set(ResponsiveMode::Tablet),
                        _ => {}
                    }
                }
                title="Rotate Viewport"
            >
                "🔄 Rotate"
            </button>

            <span class="responsive-sep">"|"</span>

            <ZoomControls />
        </div>
    }
}

#[component]
fn ZoomControls() -> impl IntoView {
    let app_state = AppState::expect_context();
    let zoom = app_state.ui.canvas_zoom;

    let zoom_in = move |_| zoom.update(|z| *z = (*z + 0.1).min(2.0));
    let zoom_out = move |_| zoom.update(|z| *z = (*z - 0.1).max(0.2));
    let zoom_reset = move |_| zoom.set(1.0);

    view! {
        <div class="btn-group">
            <button
                class="responsive-btn"
                on:click=zoom_out
                title="Zoom Out"
            >
                "➖"
            </button>
            <span class="zoom-level" style="font-size: 0.8rem; padding: 0 4px; min-width: 3.5em; text-align: center; display: inline-block;">
                {move || format!("{:.0}%", zoom.get() * 100.0)}
            </span>
            <button
                class="responsive-btn"
                on:click=zoom_in
                title="Zoom In"
            >
                "➕"
            </button>
            <button
                class="responsive-btn"
                on:click=zoom_reset
                title="Reset Zoom"
            >
                "1:1"
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
            <div
                class="canvas-viewport"
                style="width: 100%; height: 100%; transform-origin: center top;"
                style:transform=move || format!("scale({})", app_state.ui.canvas_zoom.get())
            >
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
