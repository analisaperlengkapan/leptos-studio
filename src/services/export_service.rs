use crate::domain::{AppError, AppResult, CanvasComponent};
use crate::state::ExportPreset;

/// Code generator trait
pub trait CodeGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String>;
    fn file_extension(&self) -> &str;
}

/// Leptos code generator
pub struct LeptosCodeGenerator {
    preset: ExportPreset,
}

impl LeptosCodeGenerator {
    pub fn new(preset: ExportPreset) -> Self {
        Self { preset }
    }

    fn generate_imports(&self) -> String {
        match self.preset {
            ExportPreset::Plain => "use leptos::*;\n".to_string(),
            ExportPreset::ThawUi => "use leptos::*;\nuse thaw::*;\n".to_string(),
            ExportPreset::LeptosMaterial => "use leptos::*;\nuse leptos_material::*;\n".to_string(),
            ExportPreset::LeptosUse => "use leptos::*;\nuse leptos_use::*;\n".to_string(),
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn generate_component(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "    ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                let variant_class = match btn.variant {
                    crate::domain::ButtonVariant::Primary => "btn-primary",
                    crate::domain::ButtonVariant::Secondary => "btn-secondary",
                    crate::domain::ButtonVariant::Outline => "btn-outline",
                    crate::domain::ButtonVariant::Ghost => "btn-ghost",
                };

                let size_class = match btn.size {
                    crate::domain::ButtonSize::Small => "btn-sm",
                    crate::domain::ButtonSize::Medium => "btn-md",
                    crate::domain::ButtonSize::Large => "btn-lg",
                };

                output.push_str(&format!(
                    "{}        <button class=\"{} {}\" disabled={}>{}</button>\n",
                    indent, variant_class, size_class, btn.disabled, btn.label
                ));
            }
            CanvasComponent::Text(txt) => {
                let tag = match txt.tag {
                    crate::domain::TextTag::H1 => "h1",
                    crate::domain::TextTag::H2 => "h2",
                    crate::domain::TextTag::H3 => "h3",
                    crate::domain::TextTag::P => "p",
                    crate::domain::TextTag::Span => "span",
                };

                output.push_str(&format!(
                    "{}        <{} class=\"text-{}\">{}</{}>\n",
                    indent,
                    tag,
                    match txt.style {
                        crate::domain::TextStyle::Heading1 => "heading1",
                        crate::domain::TextStyle::Heading2 => "heading2",
                        crate::domain::TextStyle::Heading3 => "heading3",
                        crate::domain::TextStyle::Body => "body",
                        crate::domain::TextStyle::Caption => "caption",
                    },
                    txt.content,
                    tag
                ));
            }
            CanvasComponent::Input(inp) => {
                let input_type = match inp.input_type {
                    crate::domain::InputType::Text => "text",
                    crate::domain::InputType::Password => "password",
                    crate::domain::InputType::Email => "email",
                    crate::domain::InputType::Number => "number",
                    crate::domain::InputType::Tel => "tel",
                };

                output.push_str(&format!(
                    "{}        <input type=\"{}\" placeholder=\"{}\" required={} disabled={} />\n",
                    indent, input_type, inp.placeholder, inp.required, inp.disabled
                ));
            }
            CanvasComponent::Select(sel) => {
                output.push_str(&format!(
                    "{}        <select disabled={}>\n",
                    indent, sel.disabled
                ));

                if !sel.placeholder.is_empty() {
                    output.push_str(&format!(
                        "{}            <option value=\"\" disabled selected>\"{}\"</option>\n",
                        indent, sel.placeholder
                    ));
                }

                for option in sel.options.split(',') {
                    let opt = option.trim();
                    output.push_str(&format!(
                        "{}            <option value=\"{}\">\"{}\"</option>\n",
                        indent, opt, opt
                    ));
                }

                output.push_str(&format!("{}        </select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                let (layout_class, align_style) = match &container.layout {
                    crate::domain::LayoutType::Flex { direction, wrap, align_items, justify_content } => {
                        let dir = match direction {
                            crate::domain::FlexDirection::Row => "flex-row",
                            crate::domain::FlexDirection::Column => "flex-col",
                        };
                        let mut classes = dir.to_string();
                        if *wrap {
                            classes.push_str(" flex-wrap");
                        }

                        let align_css = match align_items {
                            crate::domain::FlexAlign::Start => "flex-start",
                            crate::domain::FlexAlign::Center => "center",
                            crate::domain::FlexAlign::End => "flex-end",
                            crate::domain::FlexAlign::Stretch => "stretch",
                            crate::domain::FlexAlign::Baseline => "baseline",
                        };

                        let justify_css = match justify_content {
                            crate::domain::FlexJustify::Start => "flex-start",
                            crate::domain::FlexJustify::Center => "center",
                            crate::domain::FlexJustify::End => "flex-end",
                            crate::domain::FlexJustify::Between => "space-between",
                            crate::domain::FlexJustify::Around => "space-around",
                            crate::domain::FlexJustify::Evenly => "space-evenly",
                        };

                        (classes, format!("align-items: {}; justify-content: {};", align_css, justify_css))
                    }
                    crate::domain::LayoutType::Grid { columns, rows } => {
                        (format!("grid grid-cols-{} grid-rows-{}", columns, rows), String::new())
                    }
                    crate::domain::LayoutType::Stack => ("stack".to_string(), String::new()),
                };

                output.push_str(&format!(
                    "{}        <div class=\"container {}\" style=\"gap: {}px; padding: {}px {}px {}px {}px; {}\">\n",
                    indent,
                    layout_class,
                    container.gap,
                    container.padding.top,
                    container.padding.right,
                    container.padding.bottom,
                    container.padding.left,
                    align_style
                ));

                // Recursively generate children
                for child in &container.children {
                    self.generate_component(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}        </div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                let width_attr = img.width.as_ref().map_or(String::new(), |w| format!(" width=\"{}\"", w));
                let height_attr = img.height.as_ref().map_or(String::new(), |h| format!(" height=\"{}\"", h));
                output.push_str(&format!(
                    "{}        <img src=\"{}\" alt=\"{}\"{}{} />\n",
                    indent, img.src, img.alt, width_attr, height_attr
                ));
            }
            CanvasComponent::Card(card) => {
                let shadow_style = if card.shadow { "box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);" } else { "" };
                let border_class = if card.border { "border border-gray-200" } else { "" };
                output.push_str(&format!(
                    "{}        <div class=\"card {}\" style=\"padding: {}px; border-radius: {}px; {}\">\n",
                    indent, border_class, card.padding, card.border_radius, shadow_style
                ));
                for child in &card.children {
                    self.generate_component(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}        </div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!(
                    "{}        // Custom component: {}\n",
                    indent, custom.name
                ));
                output.push_str(&format!("{}        {}\n", indent, custom.template));
            }
        }

        Ok(())
    }
}

impl CodeGenerator for LeptosCodeGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut output = String::new();

        // Add imports
        output.push_str(&self.generate_imports());
        output.push('\n');

        // Generate component function
        output.push_str("#[component]\n");
        output.push_str("pub fn App() -> impl IntoView {\n");
        output.push_str("    view! {\n");

        // Generate components
        for component in components {
            self.generate_component(component, &mut output, 0)?;
        }

        output.push_str("    }\n");
        output.push_str("}\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "rs"
    }
}

/// HTML code generator
pub struct HtmlCodeGenerator;

impl CodeGenerator for HtmlCodeGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut output = String::from(
            "<!DOCTYPE html>\n<html>\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Generated Layout</title>\n</head>\n<body>\n",
        );

        for component in components {
            self.generate_html(component, &mut output, 1)?;
        }

        output.push_str("</body>\n</html>");
        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "html"
    }
}

impl HtmlCodeGenerator {
    #[allow(clippy::only_used_in_recursion)]
    fn generate_html(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "    ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                output.push_str(&format!(
                    "{}<button{}>{}</button>\n",
                    indent,
                    if btn.disabled { " disabled" } else { "" },
                    btn.label
                ));
            }
            CanvasComponent::Text(txt) => {
                let tag = match txt.tag {
                    crate::domain::TextTag::H1 => "h1",
                    crate::domain::TextTag::H2 => "h2",
                    crate::domain::TextTag::H3 => "h3",
                    crate::domain::TextTag::P => "p",
                    crate::domain::TextTag::Span => "span",
                };
                output.push_str(&format!("{}<{}>{}</{}>\n", indent, tag, txt.content, tag));
            }
            CanvasComponent::Input(inp) => {
                let input_type = match inp.input_type {
                    crate::domain::InputType::Text => "text",
                    crate::domain::InputType::Password => "password",
                    crate::domain::InputType::Email => "email",
                    crate::domain::InputType::Number => "number",
                    crate::domain::InputType::Tel => "tel",
                };
                output.push_str(&format!(
                    "{}<input type=\"{}\" placeholder=\"{}\"{}{}>\n",
                    indent,
                    input_type,
                    inp.placeholder,
                    if inp.required { " required" } else { "" },
                    if inp.disabled { " disabled" } else { "" }
                ));
            }
            CanvasComponent::Select(sel) => {
                output.push_str(&format!("{}<select{}>\n", indent, if sel.disabled { " disabled" } else { "" }));
                if !sel.placeholder.is_empty() {
                    output.push_str(&format!("{}    <option value=\"\" disabled selected>{}</option>\n", indent, sel.placeholder));
                }
                for option in sel.options.split(',') {
                    let opt = option.trim();
                    output.push_str(&format!("{}    <option value=\"{}\">{}</option>\n", indent, opt, opt));
                }
                output.push_str(&format!("{}</select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                output.push_str(&format!("{}<div>\n", indent));
                for child in &container.children {
                    self.generate_html(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                let width_attr = img.width.as_ref().map_or(String::new(), |w| format!(" width=\"{}\"", w));
                let height_attr = img.height.as_ref().map_or(String::new(), |h| format!(" height=\"{}\"", h));
                output.push_str(&format!(
                    "{}<img src=\"{}\" alt=\"{}\"{}{} />\n",
                    indent, img.src, img.alt, width_attr, height_attr
                ));
            }
            CanvasComponent::Card(card) => {
                let shadow_style = if card.shadow { "box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);" } else { "" };
                let border_style = if card.border { "border: 1px solid #e5e7eb;" } else { "" };
                output.push_str(&format!(
                    "{}<div style=\"padding: {}px; border-radius: {}px; {} {}\">\n",
                    indent, card.padding, card.border_radius, shadow_style, border_style
                ));
                for child in &card.children {
                    self.generate_html(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!("{}<!-- {} -->\n", indent, custom.name));
                output.push_str(&format!("{}{}\n", indent, custom.template));
            }
        }

        Ok(())
    }
}

/// JSON code generator
pub struct JsonCodeGenerator;

impl CodeGenerator for JsonCodeGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        serde_json::to_string_pretty(components)
            .map_err(|e| AppError::Export(format!("Failed to serialize to JSON: {}", e)))
    }

    fn file_extension(&self) -> &str {
        "json"
    }
}

/// Markdown code generator (for documentation)
pub struct MarkdownCodeGenerator;

impl CodeGenerator for MarkdownCodeGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut output = String::from("# Generated Layout Documentation\n\n");

        for (i, component) in components.iter().enumerate() {
            output.push_str(&format!("## Component {}\n\n", i + 1));
            self.generate_markdown(component, &mut output, 0)?;
            output.push('\n');
        }

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "md"
    }
}

impl MarkdownCodeGenerator {
    #[allow(clippy::only_used_in_recursion)]
    fn generate_markdown(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        depth: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(depth);

        match component {
            CanvasComponent::Button(btn) => {
                output.push_str(&format!("{}- **Button**: {}\n", indent, btn.label));
                output.push_str(&format!("{}  - Variant: {:?}\n", indent, btn.variant));
                output.push_str(&format!("{}  - Size: {:?}\n", indent, btn.size));
                output.push_str(&format!("{}  - Disabled: {}\n", indent, btn.disabled));
            }
            CanvasComponent::Text(txt) => {
                output.push_str(&format!("{}- **Text**: {}\n", indent, txt.content));
                output.push_str(&format!("{}  - Style: {:?}\n", indent, txt.style));
                output.push_str(&format!("{}  - Tag: {:?}\n", indent, txt.tag));
            }
            CanvasComponent::Input(inp) => {
                output.push_str(&format!("{}- **Input**\n", indent));
                output.push_str(&format!("{}  - Type: {:?}\n", indent, inp.input_type));
                output.push_str(&format!("{}  - Placeholder: {}\n", indent, inp.placeholder));
                output.push_str(&format!("{}  - Required: {}\n", indent, inp.required));
            }
            CanvasComponent::Select(sel) => {
                output.push_str(&format!("{}- **Select**\n", indent));
                output.push_str(&format!("{}  - Placeholder: {}\n", indent, sel.placeholder));
                output.push_str(&format!("{}  - Options: {}\n", indent, sel.options));
                output.push_str(&format!("{}  - Disabled: {}\n", indent, sel.disabled));
            }
            CanvasComponent::Container(container) => {
                output.push_str(&format!("{}- **Container**\n", indent));
                output.push_str(&format!("{}  - Layout: {:?}\n", indent, container.layout));
                output.push_str(&format!(
                    "{}  - Children: {}\n",
                    indent,
                    container.children.len()
                ));
                for child in &container.children {
                    self.generate_markdown(child, output, depth + 1)?;
                }
            }
            CanvasComponent::Image(img) => {
                output.push_str(&format!("{}- **Image**\n", indent));
                output.push_str(&format!("{}  - Src: {}\n", indent, img.src));
                output.push_str(&format!("{}  - Alt: {}\n", indent, img.alt));
            }
            CanvasComponent::Card(card) => {
                output.push_str(&format!("{}- **Card**\n", indent));
                output.push_str(&format!("{}  - Padding: {}\n", indent, card.padding));
                for child in &card.children {
                    self.generate_markdown(child, output, depth + 1)?;
                }
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!(
                    "{}- **Custom Component**: {}\n",
                    indent, custom.name
                ));
                output.push_str(&format!("{}  - Template: {}\n", indent, custom.template));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ButtonComponent, CanvasComponent, TextComponent};

    #[test]
    fn test_leptos_generator() {
        let generator = LeptosCodeGenerator::new(ExportPreset::Plain);
        let button = CanvasComponent::Button(ButtonComponent::new("Click me".to_string()));
        let code = generator.generate(&[button]).unwrap();

        assert!(code.contains("use leptos::*;"));
        assert!(code.contains("#[component]"));
        assert!(code.contains("pub fn App()"));
        assert!(code.contains("Click me"));
    }

    #[test]
    fn test_html_generator() {
        let generator = HtmlCodeGenerator;
        let text = CanvasComponent::Text(TextComponent::new("Hello World".to_string()));
        let code = generator.generate(&[text]).unwrap();

        assert!(code.contains("<!DOCTYPE html>"));
        assert!(code.contains("<body>"));
        assert!(code.contains("Hello World"));
    }

    #[test]
    fn test_json_generator() {
        let generator = JsonCodeGenerator;
        let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
        let code = generator.generate(&[button]).unwrap();

        assert!(code.contains("Button"));
        assert!(code.contains("Test"));
    }

    #[test]
    fn test_markdown_generator() {
        let generator = MarkdownCodeGenerator;
        let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
        let code = generator.generate(&[button]).unwrap();

        assert!(code.contains("# Generated Layout Documentation"));
        assert!(code.contains("**Button**"));
        assert!(code.contains("Test"));
    }

    #[test]
    fn test_leptos_select_generator() {
        use crate::domain::SelectComponent;
        let generator = LeptosCodeGenerator::new(ExportPreset::Plain);
        let select = CanvasComponent::Select(SelectComponent {
            id: Default::default(),
            options: "A, B, C".to_string(),
            placeholder: "Choose".to_string(),
            disabled: false,
        });
        let code = generator.generate(&[select]).unwrap();

        assert!(code.contains("<select disabled=false>"));
        assert!(code.contains("<option value=\"\" disabled selected>\"Choose\"</option>"));
        assert!(code.contains("<option value=\"A\">\"A\"</option>"));
        assert!(code.contains("<option value=\"B\">\"B\"</option>"));
    }

    #[test]
    fn test_html_select_generator() {
        use crate::domain::SelectComponent;
        let generator = HtmlCodeGenerator;
        let select = CanvasComponent::Select(SelectComponent {
            id: Default::default(),
            options: "X, Y".to_string(),
            placeholder: "Pick".to_string(),
            disabled: true,
        });
        let code = generator.generate(&[select]).unwrap();

        assert!(code.contains("<select disabled>"));
        assert!(code.contains("<option value=\"\" disabled selected>Pick</option>"));
        assert!(code.contains("<option value=\"X\">X</option>"));
    }

    #[test]
    fn test_markdown_select_generator() {
        use crate::domain::SelectComponent;
        let generator = MarkdownCodeGenerator;
        let select = CanvasComponent::Select(SelectComponent {
            id: Default::default(),
            options: "One, Two".to_string(),
            placeholder: "Select One".to_string(),
            disabled: false,
        });
        let code = generator.generate(&[select]).unwrap();

        assert!(code.contains("- **Select**"));
        assert!(code.contains("- Placeholder: Select One"));
        assert!(code.contains("- Options: One, Two"));
    }
}
