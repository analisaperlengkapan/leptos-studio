//! Application-wide constants
//!
//! This module centralizes all magic numbers and configuration values
//! used throughout the application for easier maintenance and configuration.

// ============================================================================
// History & Undo/Redo
// ============================================================================

/// Maximum number of snapshots to keep in the undo/redo history
pub const MAX_HISTORY_SIZE: usize = 50;

/// Minimum time (ms) between history snapshots to prevent excessive recording
pub const HISTORY_DEBOUNCE_MS: u32 = 100;

// ============================================================================
// UI Timing
// ============================================================================

/// Default notification display duration (ms)
pub const NOTIFICATION_DURATION_DEFAULT_MS: u32 = 3000;

/// Success notification duration (ms)
pub const NOTIFICATION_DURATION_SUCCESS_MS: u32 = 3000;

/// Error notification duration (ms)
pub const NOTIFICATION_DURATION_ERROR_MS: u32 = 5000;

/// Warning notification duration (ms)
pub const NOTIFICATION_DURATION_WARNING_MS: u32 = 4000;

/// Info notification duration (ms)
pub const NOTIFICATION_DURATION_INFO_MS: u32 = 3000;

/// Drag preview update throttle (ms)
pub const DRAG_PREVIEW_THROTTLE_MS: u32 = 16; // ~60fps

/// Auto-save debounce delay (ms)
pub const AUTO_SAVE_DEBOUNCE_MS: u32 = 1000;

// ============================================================================
// Canvas & Layout
// ============================================================================

/// Default container gap in pixels
pub const DEFAULT_CONTAINER_GAP: u32 = 8;

/// Default grid columns
pub const DEFAULT_GRID_COLUMNS: u32 = 2;

/// Default grid rows
pub const DEFAULT_GRID_ROWS: u32 = 2;

/// Minimum canvas width in pixels
pub const MIN_CANVAS_WIDTH: u32 = 400;

/// Minimum canvas height in pixels
pub const MIN_CANVAS_HEIGHT: u32 = 300;

// ============================================================================
// Responsive Breakpoints
// ============================================================================

/// Mobile viewport width
pub const VIEWPORT_MOBILE_WIDTH: u32 = 375;

/// Mobile viewport height
pub const VIEWPORT_MOBILE_HEIGHT: u32 = 667;

/// Tablet viewport width
pub const VIEWPORT_TABLET_WIDTH: u32 = 768;

/// Tablet viewport height
pub const VIEWPORT_TABLET_HEIGHT: u32 = 1024;

/// Desktop viewport width
pub const VIEWPORT_DESKTOP_WIDTH: u32 = 1920;

/// Desktop viewport height
pub const VIEWPORT_DESKTOP_HEIGHT: u32 = 1080;

// ============================================================================
// Drag & Drop
// ============================================================================

/// Distance threshold for auto-scroll trigger (pixels from edge)
pub const DRAG_SCROLL_THRESHOLD: f64 = 50.0;

/// Auto-scroll speed in pixels per frame
pub const DRAG_SCROLL_SPEED: f64 = 10.0;

// ============================================================================
// Validation & Limits
// ============================================================================

/// Minimum template length for custom components
pub const MIN_TEMPLATE_LENGTH: usize = 3;

/// Maximum template length for custom components (50KB)
pub const MAX_TEMPLATE_LENGTH: usize = 50_000;

/// Maximum project name length
pub const MAX_PROJECT_NAME_LENGTH: usize = 100;

/// Maximum component name length
pub const MAX_COMPONENT_NAME_LENGTH: usize = 50;

/// Maximum number of components on canvas
pub const MAX_CANVAS_COMPONENTS: usize = 1000;

/// Maximum nesting depth for containers
pub const MAX_CONTAINER_NESTING_DEPTH: usize = 10;

// ============================================================================
// Storage Keys
// ============================================================================

/// LocalStorage key for canvas data
pub const STORAGE_KEY_CANVAS: &str = "leptos_studio_canvas";

/// LocalStorage key for settings
pub const STORAGE_KEY_SETTINGS: &str = "leptos_studio_settings";

/// LocalStorage key for custom components
pub const STORAGE_KEY_CUSTOM_COMPONENTS: &str = "leptos_studio_custom_components";

/// LocalStorage key for project
pub const STORAGE_KEY_PROJECT: &str = "leptos_studio_project";

// ============================================================================
// Application Metadata
// ============================================================================

/// Application name
pub const APP_NAME: &str = "Leptos Studio";

/// Application version (from Cargo.toml)
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application description
pub const APP_DESCRIPTION: &str = "Visual UI builder for Leptos framework";

// ============================================================================
// Export Formats
// ============================================================================

/// Default Leptos view indentation
pub const EXPORT_INDENT_SPACES: usize = 4;

/// Maximum export file size warning threshold (1MB)
pub const EXPORT_SIZE_WARNING_THRESHOLD: usize = 1_000_000;

// ============================================================================
// Performance Monitoring
// ============================================================================

/// Render time warning threshold (ms)
pub const RENDER_TIME_WARNING_MS: f64 = 16.0; // Target 60fps

/// Render time critical threshold (ms)
pub const RENDER_TIME_CRITICAL_MS: f64 = 100.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_are_valid() {
        // Ensure all constants have sensible values
        assert!(MAX_HISTORY_SIZE > 0);
        assert!(NOTIFICATION_DURATION_DEFAULT_MS > 0);
        assert!(MAX_TEMPLATE_LENGTH > MIN_TEMPLATE_LENGTH);
        assert!(MAX_CANVAS_COMPONENTS > 0);
        assert!(MAX_CONTAINER_NESTING_DEPTH > 0);

        // Viewport dimensions should be positive
        assert!(VIEWPORT_MOBILE_WIDTH > 0);
        assert!(VIEWPORT_TABLET_WIDTH > VIEWPORT_MOBILE_WIDTH);
        assert!(VIEWPORT_DESKTOP_WIDTH > VIEWPORT_TABLET_WIDTH);
    }

    #[test]
    fn test_storage_keys_are_unique() {
        let keys = [
            STORAGE_KEY_CANVAS,
            STORAGE_KEY_SETTINGS,
            STORAGE_KEY_CUSTOM_COMPONENTS,
            STORAGE_KEY_PROJECT,
        ];

        for (i, key1) in keys.iter().enumerate() {
            for key2 in keys.iter().skip(i + 1) {
                assert_ne!(key1, key2, "Storage keys must be unique");
            }
        }
    }
}
