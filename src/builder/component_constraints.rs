use serde::{Deserialize, Serialize};

/// Component size constraints
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SizeConstraints {
    pub min_width: Option<u32>, // in pixels
    pub max_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_height: Option<u32>,
    pub aspect_ratio: Option<f32>, // width/height ratio
}

impl SizeConstraints {
    /// Validate dimensions against constraints
    pub fn validate(&self, width: u32, height: u32) -> Result<(), String> {
        if let Some(min_w) = self.min_width
            && width < min_w
        {
            return Err(format!("Width must be at least {}px", min_w));
        }

        if let Some(max_w) = self.max_width
            && width > max_w
        {
            return Err(format!("Width cannot exceed {}px", max_w));
        }

        if let Some(min_h) = self.min_height
            && height < min_h
        {
            return Err(format!("Height must be at least {}px", min_h));
        }

        if let Some(max_h) = self.max_height
            && height > max_h
        {
            return Err(format!("Height cannot exceed {}px", max_h));
        }

        if let Some(ratio) = self.aspect_ratio {
            let current_ratio = width as f32 / height as f32;
            if (current_ratio - ratio).abs() > 0.01 {
                return Err(format!("Aspect ratio must be approximately {}", ratio));
            }
        }

        Ok(())
    }

    /// Get reasonable defaults for common components
    pub fn for_button() -> Self {
        Self {
            min_width: Some(80),
            min_height: Some(32),
            max_width: Some(400),
            ..Default::default()
        }
    }

    pub fn for_input() -> Self {
        Self {
            min_width: Some(120),
            min_height: Some(32),
            max_width: Some(500),
            ..Default::default()
        }
    }

    pub fn for_container() -> Self {
        Self {
            min_width: Some(100),
            min_height: Some(100),
            ..Default::default()
        }
    }

    pub fn for_text() -> Self {
        Self {
            min_width: Some(50),
            min_height: Some(20),
            ..Default::default()
        }
    }
}

/// Layout alignment options
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlignmentOption {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl AlignmentOption {
    pub fn as_css(&self) -> &'static str {
        match self {
            AlignmentOption::FlexStart => "flex-start",
            AlignmentOption::Center => "center",
            AlignmentOption::FlexEnd => "flex-end",
            AlignmentOption::SpaceBetween => "space-between",
            AlignmentOption::SpaceAround => "space-around",
            AlignmentOption::SpaceEvenly => "space-evenly",
        }
    }

    pub fn as_label(&self) -> &'static str {
        match self {
            AlignmentOption::FlexStart => "Start",
            AlignmentOption::Center => "Center",
            AlignmentOption::FlexEnd => "End",
            AlignmentOption::SpaceBetween => "Space Between",
            AlignmentOption::SpaceAround => "Space Around",
            AlignmentOption::SpaceEvenly => "Space Evenly",
        }
    }
}

/// Component guidelines for design consistency
#[derive(Clone, Debug)]
pub struct DesignGuideline {
    pub name: &'static str,
    pub description: &'static str,
    pub recommended_size: Option<(u32, u32)>, // width, height
    pub spacing: Option<u32>,                 // recommended spacing in pixels
}

impl DesignGuideline {
    pub fn button_guidelines() -> Vec<Self> {
        vec![
            DesignGuideline {
                name: "Primary Button",
                description: "Main call-to-action button",
                recommended_size: Some((120, 40)),
                spacing: Some(8),
            },
            DesignGuideline {
                name: "Secondary Button",
                description: "Secondary action button",
                recommended_size: Some((100, 36)),
                spacing: Some(8),
            },
            DesignGuideline {
                name: "Icon Button",
                description: "Button with icon only",
                recommended_size: Some((40, 40)),
                spacing: Some(4),
            },
        ]
    }

    pub fn input_guidelines() -> Vec<Self> {
        vec![
            DesignGuideline {
                name: "Text Input",
                description: "Standard text input field",
                recommended_size: Some((200, 36)),
                spacing: Some(8),
            },
            DesignGuideline {
                name: "Large Input",
                description: "Large text area for multiple lines",
                recommended_size: Some((300, 120)),
                spacing: Some(12),
            },
        ]
    }

    pub fn spacing_guidelines() -> Vec<Self> {
        vec![
            DesignGuideline {
                name: "Compact",
                description: "Minimal spacing between components",
                recommended_size: None,
                spacing: Some(4),
            },
            DesignGuideline {
                name: "Default",
                description: "Standard spacing",
                recommended_size: None,
                spacing: Some(8),
            },
            DesignGuideline {
                name: "Comfortable",
                description: "Comfortable spacing",
                recommended_size: None,
                spacing: Some(16),
            },
            DesignGuideline {
                name: "Loose",
                description: "Extra loose spacing",
                recommended_size: None,
                spacing: Some(24),
            },
        ]
    }
}

/// Breakpoints for responsive design
#[derive(Clone, Debug)]
pub struct ResponsiveBreakpoint {
    pub name: &'static str,
    pub width: u32,
}

impl ResponsiveBreakpoint {
    pub fn all() -> Vec<Self> {
        vec![
            Self {
                name: "Mobile",
                width: 320,
            },
            Self {
                name: "Small",
                width: 480,
            },
            Self {
                name: "Tablet",
                width: 768,
            },
            Self {
                name: "Desktop",
                width: 1024,
            },
            Self {
                name: "Large",
                width: 1280,
            },
            Self {
                name: "Extra Large",
                width: 1920,
            },
        ]
    }
}

/// Grid system definition
#[derive(Clone, Debug)]
pub struct GridSystem {
    pub columns: u32,
    pub gap: u32,
    pub width: u32,
}

impl GridSystem {
    pub fn bootstrap_12() -> Self {
        Self {
            columns: 12,
            gap: 16,
            width: 1200,
        }
    }

    pub fn material_8() -> Self {
        Self {
            columns: 8,
            gap: 8,
            width: 1200,
        }
    }

    pub fn column_width(&self) -> f32 {
        (self.width as f32 - ((self.columns - 1) as f32 * self.gap as f32)) / self.columns as f32
    }
}
