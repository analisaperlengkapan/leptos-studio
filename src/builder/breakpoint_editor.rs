//! Breakpoint Editor
//!
//! Provides custom responsive breakpoint configuration and
//! preview capabilities for different screen sizes.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Responsive breakpoint definition
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Breakpoint {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Minimum width (px)
    pub min_width: u32,
    /// Maximum width (px) - None means no max
    pub max_width: Option<u32>,
    /// Icon for display
    pub icon: String,
}

impl Breakpoint {
    pub fn new(id: &str, name: &str, min_width: u32, max_width: Option<u32>, icon: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            min_width,
            max_width,
            icon: icon.to_string(),
        }
    }

    /// Get display width (uses min_width or midpoint if range)
    pub fn display_width(&self) -> u32 {
        match self.max_width {
            Some(max) => (self.min_width + max) / 2,
            None => self.min_width,
        }
    }
}

/// Default breakpoints matching common device sizes
pub fn default_breakpoints() -> Vec<Breakpoint> {
    vec![
        Breakpoint::new("xs", "Extra Small", 320, Some(479), "📱"),
        Breakpoint::new("sm", "Small (Phone)", 480, Some(767), "📱"),
        Breakpoint::new("md", "Medium (Tablet)", 768, Some(1023), "📱"),
        Breakpoint::new("lg", "Large (Desktop)", 1024, Some(1279), "🖥️"),
        Breakpoint::new("xl", "Extra Large", 1280, Some(1535), "🖥️"),
        Breakpoint::new("2xl", "2X Large", 1536, None, "🖥️"),
    ]
}

/// Common device presets
pub fn device_presets() -> Vec<DevicePreset> {
    vec![
        DevicePreset::new("iphone-se", "iPhone SE", 375, 667, "📱"),
        DevicePreset::new("iphone-14", "iPhone 14", 390, 844, "📱"),
        DevicePreset::new("iphone-14-pro-max", "iPhone 14 Pro Max", 430, 932, "📱"),
        DevicePreset::new("pixel-7", "Pixel 7", 412, 915, "📱"),
        DevicePreset::new("samsung-s23", "Samsung Galaxy S23", 360, 780, "📱"),
        DevicePreset::new("ipad-mini", "iPad Mini", 768, 1024, "📱"),
        DevicePreset::new("ipad-pro-11", "iPad Pro 11\"", 834, 1194, "📱"),
        DevicePreset::new("ipad-pro-12", "iPad Pro 12.9\"", 1024, 1366, "📱"),
        DevicePreset::new("macbook-air", "MacBook Air", 1280, 800, "💻"),
        DevicePreset::new("macbook-pro-14", "MacBook Pro 14\"", 1512, 982, "💻"),
        DevicePreset::new("imac-24", "iMac 24\"", 2048, 1152, "🖥️"),
        DevicePreset::new("dell-xps-15", "Dell XPS 15", 1920, 1200, "💻"),
        DevicePreset::new("desktop-hd", "Desktop HD", 1920, 1080, "🖥️"),
        DevicePreset::new("desktop-4k", "Desktop 4K", 3840, 2160, "🖥️"),
    ]
}

/// Device preset for exact dimensions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePreset {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub icon: String,
}

impl DevicePreset {
    pub fn new(id: &str, name: &str, width: u32, height: u32, icon: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            width,
            height,
            icon: icon.to_string(),
        }
    }
}

/// Breakpoint editor state
#[derive(Clone, Copy)]
pub struct BreakpointState {
    /// Current breakpoints
    pub breakpoints: RwSignal<Vec<Breakpoint>>,
    /// Currently selected breakpoint
    pub selected: RwSignal<Option<String>>,
    /// Custom width for preview
    pub custom_width: RwSignal<u32>,
    /// Custom height for preview
    pub custom_height: RwSignal<u32>,
    /// Whether using custom dimensions
    pub use_custom: RwSignal<bool>,
}

impl BreakpointState {
    pub fn new() -> Self {
        Self {
            breakpoints: RwSignal::new(default_breakpoints()),
            selected: RwSignal::new(Some("lg".to_string())),
            custom_width: RwSignal::new(1024),
            custom_height: RwSignal::new(768),
            use_custom: RwSignal::new(false),
        }
    }

    /// Get current preview dimensions
    pub fn preview_dimensions(&self) -> (u32, u32) {
        if self.use_custom.get() {
            (self.custom_width.get(), self.custom_height.get())
        } else if let Some(id) = self.selected.get() {
            let breakpoints = self.breakpoints.get();
            if let Some(bp) = breakpoints.iter().find(|b| b.id == id) {
                // Use a standard 16:9 aspect ratio height
                let width = bp.display_width();
                let height = (width as f64 * 0.5625) as u32;
                (width, height)
            } else {
                (1024, 576)
            }
        } else {
            (1024, 576)
        }
    }

    /// Provide in context
    pub fn provide_context() {
        provide_context(Self::new());
    }

    /// Use from context
    pub fn use_context() -> Self {
        expect_context::<Self>()
    }
}

impl Default for BreakpointState {
    fn default() -> Self {
        Self::new()
    }
}

/// Breakpoint editor panel
#[component]
pub fn BreakpointEditor() -> impl IntoView {
    let state = BreakpointState::use_context();
    let show_presets = RwSignal::new(false);

    // Add new breakpoint
    let add_breakpoint = move |_| {
        let new_bp = Breakpoint::new(
            &format!("custom-{}", uuid::Uuid::new_v4()),
            "Custom",
            800,
            Some(1200),
            "📐",
        );
        state.breakpoints.update(|bps| bps.push(new_bp));
    };

    // Remove breakpoint
    let remove_breakpoint = move |id: String| {
        state.breakpoints.update(|bps| bps.retain(|bp| bp.id != id));
        if state.selected.get() == Some(id.clone()) {
            state.selected.set(state.breakpoints.get().first().map(|bp| bp.id.clone()));
        }
    };

    // Apply device preset
    let apply_preset = move |preset: DevicePreset| {
        state.custom_width.set(preset.width);
        state.custom_height.set(preset.height);
        state.use_custom.set(true);
        show_presets.set(false);
    };

    view! {
        <div class="breakpoint-editor">
            <div class="breakpoint-header">
                <h4>"Responsive Preview"</h4>
                <button
                    class="btn btn-sm"
                    on:click=move |_| show_presets.update(|v| *v = !*v)
                >
                    {move || if show_presets.get() { "Hide Presets" } else { "Device Presets" }}
                </button>
            </div>

            // Device presets dropdown
            {move || {
                if show_presets.get() {
                    view! {
                        <div class="device-presets">
                            <h5>"Device Presets"</h5>
                            <div class="preset-grid">
                                <For
                                    each=move || device_presets()
                                    key=|p| p.id.clone()
                                    children=move |preset| {
                                        let preset_for_click = preset.clone();
                                        view! {
                                            <button
                                                class="preset-item"
                                                on:click=move |_| apply_preset(preset_for_click.clone())
                                            >
                                                <span class="preset-icon">{preset.icon.clone()}</span>
                                                <span class="preset-name">{preset.name.clone()}</span>
                                                <span class="preset-dims">
                                                    {format!("{}×{}", preset.width, preset.height)}
                                                </span>
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}

            // Breakpoint selector
            <div class="breakpoint-list">
                <For
                    each=move || state.breakpoints.get()
                    key=|bp| bp.id.clone()
                    children=move |bp| {
                        let bp_id = bp.id.clone();
                        let bp_id_for_select = bp.id.clone();
                        let bp_id_for_remove = bp.id.clone();

                        view! {
                            <button
                                class=move || {
                                    let selected = state.selected.get() == Some(bp_id.clone());
                                    if selected && !state.use_custom.get() {
                                        "breakpoint-btn active"
                                    } else {
                                        "breakpoint-btn"
                                    }
                                }
                                on:click=move |_| {
                                    state.selected.set(Some(bp_id_for_select.clone()));
                                    state.use_custom.set(false);
                                }
                            >
                                <span class="bp-icon">{bp.icon.clone()}</span>
                                <span class="bp-name">{bp.name.clone()}</span>
                                <span class="bp-range">
                                    {move || {
                                        match bp.max_width {
                                            Some(max) => format!("{}-{}px", bp.min_width, max),
                                            None => format!("≥{}px", bp.min_width),
                                        }
                                    }}
                                </span>
                                {if bp.id.starts_with("custom-") {
                                    view! {
                                        <button
                                            class="bp-remove"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                remove_breakpoint(bp_id_for_remove.clone());
                                            }
                                        >
                                            "×"
                                        </button>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </button>
                        }
                    }
                />
            </div>

            // Custom dimensions
            <div class="custom-dimensions">
                <label class="custom-toggle">
                    <input
                        type="checkbox"
                        prop:checked=move || state.use_custom.get()
                        on:change=move |ev| {
                            state.use_custom.set(event_target_checked(&ev));
                        }
                    />
                    " Custom Size"
                </label>

                {move || {
                    if state.use_custom.get() {
                        view! {
                            <div class="dimension-inputs">
                                <div class="dim-input">
                                    <label>"Width"</label>
                                    <input
                                        type="number"
                                        min="200"
                                        max="4000"
                                        prop:value=move || state.custom_width.get()
                                        on:input=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                                state.custom_width.set(v.clamp(200, 4000));
                                            }
                                        }
                                    />
                                    <span>"px"</span>
                                </div>
                                <span class="dim-separator">"×"</span>
                                <div class="dim-input">
                                    <label>"Height"</label>
                                    <input
                                        type="number"
                                        min="200"
                                        max="4000"
                                        prop:value=move || state.custom_height.get()
                                        on:input=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                                state.custom_height.set(v.clamp(200, 4000));
                                            }
                                        }
                                    />
                                    <span>"px"</span>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        ().into_any()
                    }
                }}
            </div>

            // Add custom breakpoint button
            <button
                class="btn btn-secondary btn-sm add-breakpoint"
                on:click=add_breakpoint
            >
                "+ Add Breakpoint"
            </button>

            // Current dimensions display
            <div class="current-dimensions">
                {move || {
                    let (w, h) = state.preview_dimensions();
                    format!("Preview: {}px × {}px", w, h)
                }}
            </div>
        </div>
    }
}

/// Responsive wrapper that applies breakpoint dimensions
#[component]
pub fn ResponsiveWrapper(children: Children) -> impl IntoView {
    let state = BreakpointState::use_context();

    let style = move || {
        let (width, height) = state.preview_dimensions();
        format!(
            "width: {}px; height: {}px; max-width: 100%; overflow: auto; margin: 0 auto; border: 1px solid var(--color-border); border-radius: var(--border-radius-lg); background: white;",
            width, height
        )
    };

    view! {
        <div class="responsive-wrapper" style=style>
            {children()}
        </div>
    }
}
