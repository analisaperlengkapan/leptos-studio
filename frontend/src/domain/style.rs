use serde::{Deserialize, Serialize};

/// Style properties for components
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentStyle {
    pub padding: Option<String>, // e.g., "8px" or "8px 16px"
    pub margin: Option<String>,  // e.g., "8px" or "0 auto"
    pub width: Option<String>,   // e.g., "100px" or "50%"
    pub height: Option<String>,  // e.g., "40px" or "auto"
    pub color: Option<String>,   // text color
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<u32>,      // in pixels
    pub border_radius: Option<u32>,     // in pixels
    pub font_size: Option<u32>,         // in pixels
    pub font_weight: Option<String>,    // "normal", "bold", "600", etc.
    pub text_align: Option<String>,     // "left", "center", "right"
    pub display: Option<String>,        // "flex", "grid", "block"
    pub flex_direction: Option<String>, // "row", "column"
    pub gap: Option<String>,            // e.g., "8px"
    pub custom_css: Option<String>,     // Custom CSS class or inline styles
}

impl ComponentStyle {
    /// Generate inline CSS string from style properties
    pub fn to_css_string(&self) -> String {
        let mut css = String::new();

        if let Some(ref val) = self.padding {
            css.push_str(&format!("padding: {};", val));
        }
        if let Some(ref val) = self.margin {
            css.push_str(&format!("margin: {};", val));
        }
        if let Some(ref val) = self.width {
            css.push_str(&format!("width: {};", val));
        }
        if let Some(ref val) = self.height {
            css.push_str(&format!("height: {};", val));
        }
        if let Some(ref val) = self.color {
            css.push_str(&format!("color: {};", val));
        }
        if let Some(ref val) = self.background_color {
            css.push_str(&format!("background-color: {};", val));
        }
        if let Some(ref val) = self.border_color {
            if let Some(width) = self.border_width {
                css.push_str(&format!("border: {}px solid {};", width, val));
            }
        }
        if let Some(radius) = self.border_radius {
            css.push_str(&format!("border-radius: {}px;", radius));
        }
        if let Some(size) = self.font_size {
            css.push_str(&format!("font-size: {}px;", size));
        }
        if let Some(ref weight) = self.font_weight {
            css.push_str(&format!("font-weight: {};", weight));
        }
        if let Some(ref align) = self.text_align {
            css.push_str(&format!("text-align: {};", align));
        }
        if let Some(ref display) = self.display {
            css.push_str(&format!("display: {};", display));
        }
        if let Some(ref direction) = self.flex_direction {
            css.push_str(&format!("flex-direction: {};", direction));
        }
        if let Some(ref gap) = self.gap {
            css.push_str(&format!("gap: {};", gap));
        }

        css
    }

    /// Get CSS class selector string
    pub fn to_class_string(&self) -> Option<String> {
        self.custom_css.clone()
    }
}

/// Theme presets
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThemePreset {
    Light,
    Dark,
    HighContrast,
    Colorful,
    Minimal,
}

impl ThemePreset {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemePreset::Light => "Light",
            ThemePreset::Dark => "Dark",
            ThemePreset::HighContrast => "High Contrast",
            ThemePreset::Colorful => "Colorful",
            ThemePreset::Minimal => "Minimal",
        }
    }

    /// Get theme-specific styles
    pub fn get_button_style(&self) -> ComponentStyle {
        match self {
            ThemePreset::Light => ComponentStyle {
                background_color: Some("#3b82f6".to_string()),
                color: Some("#ffffff".to_string()),
                border_color: Some("#2563eb".to_string()),
                border_width: Some(1),
                border_radius: Some(6),
                padding: Some("8px 16px".to_string()),
                font_weight: Some("600".to_string()),
                ..Default::default()
            },
            ThemePreset::Dark => ComponentStyle {
                background_color: Some("#1e293b".to_string()),
                color: Some("#f8fafc".to_string()),
                border_color: Some("#475569".to_string()),
                border_width: Some(1),
                border_radius: Some(6),
                padding: Some("8px 16px".to_string()),
                font_weight: Some("600".to_string()),
                ..Default::default()
            },
            ThemePreset::HighContrast => ComponentStyle {
                background_color: Some("#000000".to_string()),
                color: Some("#ffffff".to_string()),
                border_color: Some("#ffffff".to_string()),
                border_width: Some(2),
                border_radius: Some(0),
                padding: Some("10px 20px".to_string()),
                font_weight: Some("bold".to_string()),
                ..Default::default()
            },
            ThemePreset::Colorful => ComponentStyle {
                background_color: Some("#ef4444".to_string()),
                color: Some("#ffffff".to_string()),
                border_color: Some("#dc2626".to_string()),
                border_width: Some(1),
                border_radius: Some(12),
                padding: Some("8px 16px".to_string()),
                font_weight: Some("600".to_string()),
                ..Default::default()
            },
            ThemePreset::Minimal => ComponentStyle {
                background_color: None,
                color: Some("#111827".to_string()),
                border_color: Some("#d1d5db".to_string()),
                border_width: Some(1),
                border_radius: Some(2),
                padding: Some("6px 12px".to_string()),
                font_weight: Some("400".to_string()),
                ..Default::default()
            },
        }
    }

    pub fn get_container_style(&self) -> ComponentStyle {
        match self {
            ThemePreset::Light => ComponentStyle {
                background_color: Some("#ffffff".to_string()),
                border_color: Some("#e5e7eb".to_string()),
                border_width: Some(1),
                border_radius: Some(8),
                padding: Some("16px".to_string()),
                ..Default::default()
            },
            ThemePreset::Dark => ComponentStyle {
                background_color: Some("#1f2937".to_string()),
                border_color: Some("#374151".to_string()),
                border_width: Some(1),
                border_radius: Some(8),
                padding: Some("16px".to_string()),
                ..Default::default()
            },
            ThemePreset::HighContrast => ComponentStyle {
                background_color: Some("#ffffff".to_string()),
                border_color: Some("#000000".to_string()),
                border_width: Some(2),
                border_radius: Some(0),
                padding: Some("16px".to_string()),
                ..Default::default()
            },
            ThemePreset::Colorful => ComponentStyle {
                background_color: Some("#fef3c7".to_string()),
                border_color: Some("#fcd34d".to_string()),
                border_width: Some(2),
                border_radius: Some(12),
                padding: Some("16px".to_string()),
                ..Default::default()
            },
            ThemePreset::Minimal => ComponentStyle {
                background_color: None,
                border_color: None,
                padding: Some("12px".to_string()),
                ..Default::default()
            },
        }
    }
}
