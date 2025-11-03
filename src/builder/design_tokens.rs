use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ColorToken {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SpacingToken {
    pub name: String,
    pub value: String,
    pub rem_value: f32,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TypographyToken {
    pub name: String,
    pub font_size: String,
    pub line_height: String,
    pub font_weight: String,
    pub letter_spacing: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BorderRadiusToken {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ShadowToken {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesignTokens {
    pub colors: Vec<ColorToken>,
    pub spacing: Vec<SpacingToken>,
    pub typography: Vec<TypographyToken>,
    pub border_radius: Vec<BorderRadiusToken>,
    pub shadows: Vec<ShadowToken>,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self {
            colors: default_color_tokens(),
            spacing: default_spacing_tokens(),
            typography: default_typography_tokens(),
            border_radius: default_border_radius_tokens(),
            shadows: default_shadow_tokens(),
        }
    }
}

fn default_color_tokens() -> Vec<ColorToken> {
    vec![
        ColorToken {
            name: "primary-50".to_string(),
            value: "#eff6ff".to_string(),
            description: Some("Lightest primary blue".to_string()),
        },
        ColorToken {
            name: "primary-100".to_string(),
            value: "#dbeafe".to_string(),
            description: Some("Very light primary blue".to_string()),
        },
        ColorToken {
            name: "primary-500".to_string(),
            value: "#3b82f6".to_string(),
            description: Some("Primary blue".to_string()),
        },
        ColorToken {
            name: "primary-600".to_string(),
            value: "#2563eb".to_string(),
            description: Some("Dark primary blue".to_string()),
        },
        ColorToken {
            name: "primary-900".to_string(),
            value: "#1e3a8a".to_string(),
            description: Some("Darkest primary blue".to_string()),
        },
        ColorToken {
            name: "gray-50".to_string(),
            value: "#f9fafb".to_string(),
            description: Some("Lightest gray".to_string()),
        },
        ColorToken {
            name: "gray-100".to_string(),
            value: "#f3f4f6".to_string(),
            description: Some("Very light gray".to_string()),
        },
        ColorToken {
            name: "gray-200".to_string(),
            value: "#e5e7eb".to_string(),
            description: Some("Light gray".to_string()),
        },
        ColorToken {
            name: "gray-300".to_string(),
            value: "#d1d5db".to_string(),
            description: Some("Medium light gray".to_string()),
        },
        ColorToken {
            name: "gray-400".to_string(),
            value: "#9ca3af".to_string(),
            description: Some("Medium gray".to_string()),
        },
        ColorToken {
            name: "gray-500".to_string(),
            value: "#6b7280".to_string(),
            description: Some("Medium dark gray".to_string()),
        },
        ColorToken {
            name: "gray-600".to_string(),
            value: "#4b5563".to_string(),
            description: Some("Dark gray".to_string()),
        },
        ColorToken {
            name: "gray-700".to_string(),
            value: "#374151".to_string(),
            description: Some("Very dark gray".to_string()),
        },
        ColorToken {
            name: "gray-800".to_string(),
            value: "#1f2937".to_string(),
            description: Some("Darkest gray".to_string()),
        },
        ColorToken {
            name: "gray-900".to_string(),
            value: "#111827".to_string(),
            description: Some("Black gray".to_string()),
        },
        ColorToken {
            name: "success-500".to_string(),
            value: "#10b981".to_string(),
            description: Some("Success green".to_string()),
        },
        ColorToken {
            name: "warning-500".to_string(),
            value: "#f59e0b".to_string(),
            description: Some("Warning orange".to_string()),
        },
        ColorToken {
            name: "error-500".to_string(),
            value: "#ef4444".to_string(),
            description: Some("Error red".to_string()),
        },
        ColorToken {
            name: "white".to_string(),
            value: "#ffffff".to_string(),
            description: Some("Pure white".to_string()),
        },
        ColorToken {
            name: "black".to_string(),
            value: "#000000".to_string(),
            description: Some("Pure black".to_string()),
        },
    ]
}

fn default_spacing_tokens() -> Vec<SpacingToken> {
    vec![
        SpacingToken {
            name: "1".to_string(),
            value: "4px".to_string(),
            rem_value: 0.25,
            description: Some("Extra small spacing".to_string()),
        },
        SpacingToken {
            name: "2".to_string(),
            value: "8px".to_string(),
            rem_value: 0.5,
            description: Some("Small spacing".to_string()),
        },
        SpacingToken {
            name: "3".to_string(),
            value: "12px".to_string(),
            rem_value: 0.75,
            description: Some("Medium small spacing".to_string()),
        },
        SpacingToken {
            name: "4".to_string(),
            value: "16px".to_string(),
            rem_value: 1.0,
            description: Some("Medium spacing".to_string()),
        },
        SpacingToken {
            name: "5".to_string(),
            value: "20px".to_string(),
            rem_value: 1.25,
            description: Some("Medium large spacing".to_string()),
        },
        SpacingToken {
            name: "6".to_string(),
            value: "24px".to_string(),
            rem_value: 1.5,
            description: Some("Large spacing".to_string()),
        },
        SpacingToken {
            name: "8".to_string(),
            value: "32px".to_string(),
            rem_value: 2.0,
            description: Some("Extra large spacing".to_string()),
        },
        SpacingToken {
            name: "10".to_string(),
            value: "40px".to_string(),
            rem_value: 2.5,
            description: Some("2X large spacing".to_string()),
        },
        SpacingToken {
            name: "12".to_string(),
            value: "48px".to_string(),
            rem_value: 3.0,
            description: Some("3X large spacing".to_string()),
        },
        SpacingToken {
            name: "16".to_string(),
            value: "64px".to_string(),
            rem_value: 4.0,
            description: Some("4X large spacing".to_string()),
        },
    ]
}

fn default_typography_tokens() -> Vec<TypographyToken> {
    vec![
        TypographyToken {
            name: "text-xs".to_string(),
            font_size: "12px".to_string(),
            line_height: "16px".to_string(),
            font_weight: "400".to_string(),
            letter_spacing: None,
            description: Some("Extra small text".to_string()),
        },
        TypographyToken {
            name: "text-sm".to_string(),
            font_size: "14px".to_string(),
            line_height: "20px".to_string(),
            font_weight: "400".to_string(),
            letter_spacing: None,
            description: Some("Small text".to_string()),
        },
        TypographyToken {
            name: "text-base".to_string(),
            font_size: "16px".to_string(),
            line_height: "24px".to_string(),
            font_weight: "400".to_string(),
            letter_spacing: None,
            description: Some("Base text".to_string()),
        },
        TypographyToken {
            name: "text-lg".to_string(),
            font_size: "18px".to_string(),
            line_height: "28px".to_string(),
            font_weight: "400".to_string(),
            letter_spacing: None,
            description: Some("Large text".to_string()),
        },
        TypographyToken {
            name: "text-xl".to_string(),
            font_size: "20px".to_string(),
            line_height: "28px".to_string(),
            font_weight: "400".to_string(),
            letter_spacing: None,
            description: Some("Extra large text".to_string()),
        },
        TypographyToken {
            name: "text-2xl".to_string(),
            font_size: "24px".to_string(),
            line_height: "32px".to_string(),
            font_weight: "700".to_string(),
            letter_spacing: Some("-0.025em".to_string()),
            description: Some("2X large text".to_string()),
        },
        TypographyToken {
            name: "text-3xl".to_string(),
            font_size: "30px".to_string(),
            line_height: "36px".to_string(),
            font_weight: "700".to_string(),
            letter_spacing: Some("-0.025em".to_string()),
            description: Some("3X large text".to_string()),
        },
    ]
}

fn default_border_radius_tokens() -> Vec<BorderRadiusToken> {
    vec![
        BorderRadiusToken {
            name: "none".to_string(),
            value: "0px".to_string(),
            description: Some("No border radius".to_string()),
        },
        BorderRadiusToken {
            name: "sm".to_string(),
            value: "2px".to_string(),
            description: Some("Small border radius".to_string()),
        },
        BorderRadiusToken {
            name: "md".to_string(),
            value: "4px".to_string(),
            description: Some("Medium border radius".to_string()),
        },
        BorderRadiusToken {
            name: "lg".to_string(),
            value: "6px".to_string(),
            description: Some("Large border radius".to_string()),
        },
        BorderRadiusToken {
            name: "xl".to_string(),
            value: "12px".to_string(),
            description: Some("Extra large border radius".to_string()),
        },
        BorderRadiusToken {
            name: "full".to_string(),
            value: "9999px".to_string(),
            description: Some("Full border radius".to_string()),
        },
    ]
}

fn default_shadow_tokens() -> Vec<ShadowToken> {
    vec![
        ShadowToken {
            name: "none".to_string(),
            value: "none".to_string(),
            description: Some("No shadow".to_string()),
        },
        ShadowToken {
            name: "sm".to_string(),
            value: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            description: Some("Small shadow".to_string()),
        },
        ShadowToken {
            name: "md".to_string(),
            value: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
            description: Some("Medium shadow".to_string()),
        },
        ShadowToken {
            name: "lg".to_string(),
            value: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
            description: Some("Large shadow".to_string()),
        },
        ShadowToken {
            name: "xl".to_string(),
            value: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)"
                .to_string(),
            description: Some("Extra large shadow".to_string()),
        },
    ]
}

impl DesignTokens {
    pub fn to_css_variables(&self) -> String {
        let mut css = ":root {\n".to_string();

        // Colors
        for color in &self.colors {
            css.push_str(&format!("  --color-{}: {};\n", color.name, color.value));
        }

        // Spacing
        for spacing in &self.spacing {
            css.push_str(&format!(
                "  --spacing-{}: {};\n",
                spacing.name, spacing.value
            ));
        }

        // Typography
        for typo in &self.typography {
            css.push_str(&format!(
                "  --font-size-{}: {};\n",
                typo.name.replace("text-", ""),
                typo.font_size
            ));
            css.push_str(&format!(
                "  --line-height-{}: {};\n",
                typo.name.replace("text-", ""),
                typo.line_height
            ));
            css.push_str(&format!(
                "  --font-weight-{}: {};\n",
                typo.name.replace("text-", ""),
                typo.font_weight
            ));
            if let Some(spacing) = &typo.letter_spacing {
                css.push_str(&format!(
                    "  --letter-spacing-{}: {};\n",
                    typo.name.replace("text-", ""),
                    spacing
                ));
            }
        }

        // Border radius
        for radius in &self.border_radius {
            css.push_str(&format!(
                "  --border-radius-{}: {};\n",
                radius.name, radius.value
            ));
        }

        // Shadows
        for shadow in &self.shadows {
            css.push_str(&format!("  --shadow-{}: {};\n", shadow.name, shadow.value));
        }

        css.push_str("}\n");
        css
    }

    pub fn apply_to_document(&self) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(head) = document.head() {
                    // Remove existing design token styles
                    if let Ok(Some(existing_element)) =
                        document.query_selector("#design-tokens-style")
                    {
                        existing_element.remove();
                    }

                    // Create new style element
                    if let Ok(style_element) = document.create_element("style") {
                        style_element.set_id("design-tokens-style");
                        style_element.set_inner_html(&self.to_css_variables());
                        _ = head.append_child(&style_element);
                    }
                }
            }
        }
    }

    pub fn get_color(&self, name: &str) -> Option<&ColorToken> {
        self.colors.iter().find(|c| c.name == name)
    }

    pub fn get_spacing(&self, name: &str) -> Option<&SpacingToken> {
        self.spacing.iter().find(|s| s.name == name)
    }

    pub fn get_typography(&self, name: &str) -> Option<&TypographyToken> {
        self.typography.iter().find(|t| t.name == name)
    }

    pub fn css_var_color(&self, name: &str) -> String {
        format!("var(--color-{})", name)
    }

    pub fn css_var_spacing(&self, name: &str) -> String {
        format!("var(--spacing-{})", name)
    }

    pub fn css_var_shadow(&self, name: &str) -> String {
        format!("var(--shadow-{})", name)
    }

    pub fn css_var_border_radius(&self, name: &str) -> String {
        format!("var(--border-radius-{})", name)
    }
}

#[component]
pub fn DesignTokenProvider(tokens: RwSignal<DesignTokens>, children: Children) -> impl IntoView {
    // Apply tokens to document whenever they change
    Effect::new(move |_| {
        let tokens = tokens.get();
        tokens.apply_to_document();
    });

    view! {
        <div class="design-token-provider">
            {children()}
        </div>
    }
}

#[component]
pub fn TokenPreview(tokens: RwSignal<DesignTokens>) -> impl IntoView {
    view! {
        <div style="
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: var(--spacing-6);
            padding: var(--spacing-6);
        ">
            // Colors
            <div style="
                background: var(--color-white);
                border: 1px solid var(--color-gray-200);
                border-radius: var(--border-radius-lg);
                padding: var(--spacing-6);
                box-shadow: var(--shadow-sm);
            ">
                <h3 style="
                    font-size: var(--font-size-lg);
                    font-weight: var(--font-weight-lg);
                    margin-bottom: var(--spacing-4);
                    color: var(--color-gray-900);
                ">
                    "Colors"
                </h3>
                <div style="display: grid; gap: var(--spacing-2);">
                    <For
                        each=move || tokens.get().colors
                        key=|color| color.name.clone()
                        children=|color| {
                            view! {
                                <div style="
                                    display: flex;
                                    align-items: center;
                                    gap: var(--spacing-3);
                                ">
                                    <div style=format!("
                                        width: 24px;
                                        height: 24px;
                                        border-radius: var(--border-radius-md);
                                        background: {};
                                        border: 1px solid var(--color-gray-200);
                                    ", color.value)></div>
                                    <div>
                                        <div style="
                                            font-size: var(--font-size-sm);
                                            font-weight: 500;
                                            color: var(--color-gray-900);
                                        ">
                                            {color.name}
                                        </div>
                                        <div style="
                                            font-size: var(--font-size-xs);
                                            color: var(--color-gray-500);
                                            font-family: monospace;
                                        ">
                                            {color.value}
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </div>

            // Typography
            <div style="
                background: var(--color-white);
                border: 1px solid var(--color-gray-200);
                border-radius: var(--border-radius-lg);
                padding: var(--spacing-6);
                box-shadow: var(--shadow-sm);
            ">
                <h3 style="
                    font-size: var(--font-size-lg);
                    font-weight: var(--font-weight-lg);
                    margin-bottom: var(--spacing-4);
                    color: var(--color-gray-900);
                ">
                    "Typography"
                </h3>
                <div style="display: grid; gap: var(--spacing-3);">
                    <For
                        each=move || tokens.get().typography
                        key=|typo| typo.name.clone()
                        children=|typo| {
                            view! {
                                <div>
                                    <div style=format!("
                                        font-size: {};
                                        line-height: {};
                                        font-weight: {};
                                        color: var(--color-gray-900);
                                        margin-bottom: var(--spacing-1);
                                    ", typo.font_size, typo.line_height, typo.font_weight)>
                                        "The quick brown fox jumps"
                                    </div>
                                    <div style="
                                        font-size: var(--font-size-xs);
                                        color: var(--color-gray-500);
                                    ">
                                        {format!("{} • {} / {} • {}", typo.name, typo.font_size, typo.line_height, typo.font_weight)}
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
