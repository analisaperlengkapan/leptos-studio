//! Advanced Export Formats
//!
//! Additional export generators for JSON Schema, TypeScript types,
//! React components, and other formats.

use crate::domain::{AppError, AppResult, CanvasComponent};

use super::CodeGenerator;

/// JSON Schema generator for component validation
#[derive(Default)]
pub struct JsonSchemaGenerator;

impl CodeGenerator for JsonSchemaGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let schema = serde_json::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "Leptos Studio Layout",
            "description": "JSON Schema for Leptos Studio component layouts",
            "type": "array",
            "items": {
                "$ref": "#/definitions/CanvasComponent"
            },
            "definitions": {
                "CanvasComponent": {
                    "oneOf": [
                        { "$ref": "#/definitions/ButtonComponent" },
                        { "$ref": "#/definitions/TextComponent" },
                        { "$ref": "#/definitions/InputComponent" },
                        { "$ref": "#/definitions/ContainerComponent" },
                        { "$ref": "#/definitions/CustomComponent" }
                    ]
                },
                "ButtonComponent": {
                    "type": "object",
                    "required": ["Button"],
                    "properties": {
                        "Button": {
                            "type": "object",
                            "required": ["id", "label", "variant", "size", "disabled"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "label": { "type": "string" },
                                "variant": {
                                    "type": "string",
                                    "enum": ["Primary", "Secondary", "Outline", "Ghost"]
                                },
                                "size": {
                                    "type": "string",
                                    "enum": ["Small", "Medium", "Large"]
                                },
                                "disabled": { "type": "boolean" },
                                "on_click": { "type": ["string", "null"] }
                            }
                        }
                    }
                },
                "TextComponent": {
                    "type": "object",
                    "required": ["Text"],
                    "properties": {
                        "Text": {
                            "type": "object",
                            "required": ["id", "content", "style", "tag"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "content": { "type": "string" },
                                "style": {
                                    "type": "string",
                                    "enum": ["Heading1", "Heading2", "Heading3", "Body", "Caption"]
                                },
                                "tag": {
                                    "type": "string",
                                    "enum": ["H1", "H2", "H3", "P", "Span"]
                                }
                            }
                        }
                    }
                },
                "InputComponent": {
                    "type": "object",
                    "required": ["Input"],
                    "properties": {
                        "Input": {
                            "type": "object",
                            "required": ["id", "placeholder", "input_type", "required", "disabled"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "placeholder": { "type": "string" },
                                "input_type": {
                                    "type": "string",
                                    "enum": ["Text", "Password", "Email", "Number", "Tel"]
                                },
                                "required": { "type": "boolean" },
                                "disabled": { "type": "boolean" }
                            }
                        }
                    }
                },
                "ContainerComponent": {
                    "type": "object",
                    "required": ["Container"],
                    "properties": {
                        "Container": {
                            "type": "object",
                            "required": ["id", "children", "layout", "gap", "padding"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "children": {
                                    "type": "array",
                                    "items": { "$ref": "#/definitions/CanvasComponent" }
                                },
                                "layout": { "$ref": "#/definitions/LayoutType" },
                                "gap": { "type": "integer", "minimum": 0 },
                                "padding": { "$ref": "#/definitions/Spacing" }
                            }
                        }
                    }
                },
                "CustomComponent": {
                    "type": "object",
                    "required": ["Custom"],
                    "properties": {
                        "Custom": {
                            "type": "object",
                            "required": ["id", "name", "template", "props"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "name": { "type": "string" },
                                "template": { "type": "string" },
                                "props": { "type": "object" }
                            }
                        }
                    }
                },
                "LayoutType": {
                    "oneOf": [
                        {
                            "type": "object",
                            "properties": {
                                "Flex": {
                                    "type": "object",
                                    "properties": {
                                        "direction": { "enum": ["Row", "Column"] },
                                        "wrap": { "type": "boolean" }
                                    }
                                }
                            }
                        },
                        {
                            "type": "object",
                            "properties": {
                                "Grid": {
                                    "type": "object",
                                    "properties": {
                                        "columns": { "type": "integer" },
                                        "rows": { "type": "integer" }
                                    }
                                }
                            }
                        },
                        { "const": "Stack" }
                    ]
                },
                "Spacing": {
                    "type": "object",
                    "required": ["top", "right", "bottom", "left"],
                    "properties": {
                        "top": { "type": "integer", "minimum": 0 },
                        "right": { "type": "integer", "minimum": 0 },
                        "bottom": { "type": "integer", "minimum": 0 },
                        "left": { "type": "integer", "minimum": 0 }
                    }
                }
            },
            "examples": [serde_json::to_value(components).unwrap_or_default()]
        });

        serde_json::to_string_pretty(&schema)
            .map_err(|e| AppError::Export(format!("Failed to generate JSON Schema: {}", e)))
    }

    fn file_extension(&self) -> &str {
        "schema.json"
    }
}

/// TypeScript types generator
#[derive(Default)]
pub struct TypeScriptGenerator;

impl CodeGenerator for TypeScriptGenerator {
    fn generate(&self, _components: &[CanvasComponent]) -> AppResult<String> {
        let types = r#"/**
 * Leptos Studio - TypeScript Type Definitions
 * Auto-generated from component layout
 */

// Component ID type
export type ComponentId = string;

// Button variants
export type ButtonVariant = 'Primary' | 'Secondary' | 'Outline' | 'Ghost';

// Button sizes
export type ButtonSize = 'Small' | 'Medium' | 'Large';

// Text styles
export type TextStyle = 'Heading1' | 'Heading2' | 'Heading3' | 'Body' | 'Caption';

// HTML tags for text
export type TextTag = 'H1' | 'H2' | 'H3' | 'P' | 'Span';

// Input types
export type InputType = 'Text' | 'Password' | 'Email' | 'Number' | 'Tel';

// Flex direction
export type FlexDirection = 'Row' | 'Column';

// Layout types
export type LayoutType = 
  | { Flex: { direction: FlexDirection; wrap: boolean } }
  | { Grid: { columns: number; rows: number } }
  | 'Stack';

// Spacing
export interface Spacing {
  top: number;
  right: number;
  bottom: number;
  left: number;
}

// Button component
export interface ButtonComponent {
  id: ComponentId;
  label: string;
  variant: ButtonVariant;
  size: ButtonSize;
  disabled: boolean;
  on_click?: string | null;
}

// Text component
export interface TextComponent {
  id: ComponentId;
  content: string;
  style: TextStyle;
  tag: TextTag;
}

// Input component
export interface InputComponent {
  id: ComponentId;
  placeholder: string;
  input_type: InputType;
  required: boolean;
  disabled: boolean;
}

// Container component
export interface ContainerComponent {
  id: ComponentId;
  children: CanvasComponent[];
  layout: LayoutType;
  gap: number;
  padding: Spacing;
}

// Custom component
export interface CustomComponent {
  id: ComponentId;
  name: string;
  template: string;
  props: Record<string, PropValue>;
}

// Property value types
export type PropValue = 
  | { String: string }
  | { Number: number }
  | { Boolean: boolean }
  | 'Null';

// Canvas component union type
export type CanvasComponent = 
  | { Button: ButtonComponent }
  | { Text: TextComponent }
  | { Input: InputComponent }
  | { Container: ContainerComponent }
  | { Custom: CustomComponent };

// Layout type (array of components)
export type Layout = CanvasComponent[];

// Project export type
export interface Project {
  name: string;
  description?: string;
  layout: Layout;
  theme: string;
  created_at: string;
  updated_at: string;
}

// Helper type guards
export function isButton(c: CanvasComponent): c is { Button: ButtonComponent } {
  return 'Button' in c;
}

export function isText(c: CanvasComponent): c is { Text: TextComponent } {
  return 'Text' in c;
}

export function isInput(c: CanvasComponent): c is { Input: InputComponent } {
  return 'Input' in c;
}

export function isContainer(c: CanvasComponent): c is { Container: ContainerComponent } {
  return 'Container' in c;
}

export function isCustom(c: CanvasComponent): c is { Custom: CustomComponent } {
  return 'Custom' in c;
}
"#;

        Ok(types.to_string())
    }

    fn file_extension(&self) -> &str {
        "d.ts"
    }
}

/// React component generator
#[derive(Default)]
pub struct ReactGenerator;

impl CodeGenerator for ReactGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut output = String::new();

        // Imports
        output.push_str("import React from 'react';\n\n");

        // Generate component
        output.push_str("export function GeneratedLayout() {\n");
        output.push_str("  return (\n");
        output.push_str("    <>\n");

        for component in components {
            Self::generate_react(component, &mut output, 3)?;
        }

        output.push_str("    </>\n");
        output.push_str("  );\n");
        output.push_str("}\n\n");

        output.push_str("export default GeneratedLayout;\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "tsx"
    }
}

impl ReactGenerator {
    fn generate_react(
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(indent_level);

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
                    "{}<button className=\"{} {}\" disabled={{{}}}>{}</button>\n",
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
                    "{}<input type=\"{}\" placeholder=\"{}\" required={{{}}} disabled={{{}}} />\n",
                    indent, input_type, inp.placeholder, inp.required, inp.disabled
                ));
            }
            CanvasComponent::Container(container) => {
                let layout_style = match &container.layout {
                    crate::domain::LayoutType::Flex {
                        direction,
                        align_items,
                        justify_content,
                        ..
                    } => {
                        let dir = match direction {
                            crate::domain::FlexDirection::Row => "row",
                            crate::domain::FlexDirection::Column => "column",
                        };
                        let align = match align_items {
                            crate::domain::FlexAlign::Start => "flex-start",
                            crate::domain::FlexAlign::Center => "center",
                            crate::domain::FlexAlign::End => "flex-end",
                            crate::domain::FlexAlign::Stretch => "stretch",
                            crate::domain::FlexAlign::Baseline => "baseline",
                        };
                        let justify = match justify_content {
                            crate::domain::FlexJustify::Start => "flex-start",
                            crate::domain::FlexJustify::Center => "center",
                            crate::domain::FlexJustify::End => "flex-end",
                            crate::domain::FlexJustify::Between => "space-between",
                            crate::domain::FlexJustify::Around => "space-around",
                            crate::domain::FlexJustify::Evenly => "space-evenly",
                        };
                        format!(
                            "display: 'flex', flexDirection: '{}', alignItems: '{}', justifyContent: '{}'",
                            dir, align, justify
                        )
                    }
                    crate::domain::LayoutType::Grid { columns, rows } => {
                        format!(
                            "display: 'grid', gridTemplateColumns: 'repeat({}, 1fr)', gridTemplateRows: 'repeat({}, auto)'",
                            columns, rows
                        )
                    }
                    crate::domain::LayoutType::Stack => "display: 'flex', flexDirection: 'column'".to_string(),
                };

                let style = format!(
                    "{{ {}, gap: '{}px', padding: '{}px {}px {}px {}px' }}",
                    layout_style,
                    container.gap,
                    container.padding.top,
                    container.padding.right,
                    container.padding.bottom,
                    container.padding.left
                );

                output.push_str(&format!("{}<div style={{{}}}>\n", indent, style));

                for child in &container.children {
                    Self::generate_react(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!("{}<!-- Custom: {} -->\n", indent, custom.name));
                output.push_str(&format!(
                    "{}<div dangerouslySetInnerHTML={{{{ __html: `{}` }}}} />\n",
                    indent, custom.template
                ));
            }
        }

        Ok(())
    }
}

/// Vue component generator
#[derive(Default)]
pub struct VueGenerator;

impl CodeGenerator for VueGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut template = String::from("<template>\n  <div class=\"generated-layout\">\n");

        for component in components {
            Self::generate_vue(component, &mut template, 2)?;
        }

        template.push_str("  </div>\n</template>\n\n");

        // Script section
        template.push_str("<script setup lang=\"ts\">\n");
        template.push_str("// Generated by Leptos Studio\n");
        template.push_str("</script>\n\n");

        // Style section
        template.push_str("<style scoped>\n");
        template.push_str(".generated-layout {\n");
        template.push_str("  /* Add your styles here */\n");
        template.push_str("}\n");
        template.push_str("</style>\n");

        Ok(template)
    }

    fn file_extension(&self) -> &str {
        "vue"
    }
}

impl VueGenerator {
    fn generate_vue(
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                output.push_str(&format!(
                    "{}<button :disabled=\"{}\">{}</button>\n",
                    indent, btn.disabled, btn.label
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
                    "{}<input type=\"{}\" placeholder=\"{}\" :required=\"{}\" :disabled=\"{}\" />\n",
                    indent, input_type, inp.placeholder, inp.required, inp.disabled
                ));
            }
            CanvasComponent::Container(container) => {
                output.push_str(&format!("{}<div>\n", indent));
                for child in &container.children {
                    Self::generate_vue(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!("{}<!-- {} -->\n", indent, custom.name));
                output.push_str(&format!(
                    "{}<div v-html=\"`{}`\"></div>\n",
                    indent, custom.template
                ));
            }
        }

        Ok(())
    }
}

/// CSS generator (extracts styles)
#[derive(Default)]
pub struct CssGenerator;

impl CodeGenerator for CssGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut css = String::from("/* Generated by Leptos Studio */\n\n");

        // Basic button styles
        css.push_str(".btn-primary {\n");
        css.push_str("  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);\n");
        css.push_str("  color: white;\n");
        css.push_str("  border: none;\n");
        css.push_str("  padding: 8px 16px;\n");
        css.push_str("  border-radius: 6px;\n");
        css.push_str("  cursor: pointer;\n");
        css.push_str("}\n\n");

        css.push_str(".btn-secondary {\n");
        css.push_str("  background: white;\n");
        css.push_str("  color: #475569;\n");
        css.push_str("  border: 1px solid #cbd5e1;\n");
        css.push_str("  padding: 8px 16px;\n");
        css.push_str("  border-radius: 6px;\n");
        css.push_str("  cursor: pointer;\n");
        css.push_str("}\n\n");

        css.push_str(".btn-outline {\n");
        css.push_str("  background: transparent;\n");
        css.push_str("  color: #3b82f6;\n");
        css.push_str("  border: 1px solid #3b82f6;\n");
        css.push_str("  padding: 8px 16px;\n");
        css.push_str("  border-radius: 6px;\n");
        css.push_str("  cursor: pointer;\n");
        css.push_str("}\n\n");

        css.push_str(".btn-ghost {\n");
        css.push_str("  background: transparent;\n");
        css.push_str("  color: #6b7280;\n");
        css.push_str("  border: none;\n");
        css.push_str("  padding: 8px 16px;\n");
        css.push_str("  cursor: pointer;\n");
        css.push_str("}\n\n");

        // Size modifiers
        css.push_str(".btn-sm { padding: 4px 12px; font-size: 12px; }\n");
        css.push_str(".btn-md { padding: 8px 16px; font-size: 14px; }\n");
        css.push_str(".btn-lg { padding: 12px 24px; font-size: 16px; }\n\n");

        // Container styles
        Self::extract_container_styles(components, &mut css);

        Ok(css)
    }

    fn file_extension(&self) -> &str {
        "css"
    }
}

impl CssGenerator {
    fn extract_container_styles(components: &[CanvasComponent], css: &mut String) {
        for component in components {
            if let CanvasComponent::Container(container) = component {
                let id = container.id.as_string();
                let class_name = format!("container-{}", &id[..8]);

                css.push_str(&format!(".{} {{\n", class_name));

                match &container.layout {
                    crate::domain::LayoutType::Flex {
                        direction,
                        wrap,
                        align_items,
                        justify_content,
                    } => {
                        css.push_str("  display: flex;\n");
                        css.push_str(&format!(
                            "  flex-direction: {};\n",
                            match direction {
                                crate::domain::FlexDirection::Row => "row",
                                crate::domain::FlexDirection::Column => "column",
                            }
                        ));
                        if *wrap {
                            css.push_str("  flex-wrap: wrap;\n");
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

                        css.push_str(&format!("  align-items: {};\n", align_css));
                        css.push_str(&format!("  justify-content: {};\n", justify_css));
                    }
                    crate::domain::LayoutType::Grid { columns, rows } => {
                        css.push_str("  display: grid;\n");
                        css.push_str(&format!(
                            "  grid-template-columns: repeat({}, 1fr);\n",
                            columns
                        ));
                        css.push_str(&format!("  grid-template-rows: repeat({}, auto);\n", rows));
                    }
                    crate::domain::LayoutType::Stack => {
                        css.push_str("  display: flex;\n");
                        css.push_str("  flex-direction: column;\n");
                    }
                }

                css.push_str(&format!("  gap: {}px;\n", container.gap));
                css.push_str(&format!(
                    "  padding: {}px {}px {}px {}px;\n",
                    container.padding.top,
                    container.padding.right,
                    container.padding.bottom,
                    container.padding.left
                ));

                css.push_str("}\n\n");

                // Recurse into children
                Self::extract_container_styles(&container.children, css);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ButtonComponent, TextComponent};

    #[test]
    fn test_json_schema_generator() {
        let generator = JsonSchemaGenerator;
        let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
        let schema = generator.generate(&[button]).unwrap();

        assert!(schema.contains("$schema"));
        assert!(schema.contains("definitions"));
        assert!(schema.contains("ButtonComponent"));
    }

    #[test]
    fn test_typescript_generator() {
        let generator = TypeScriptGenerator;
        let types = generator.generate(&[]).unwrap();

        assert!(types.contains("ButtonVariant"));
        assert!(types.contains("CanvasComponent"));
        assert!(types.contains("export interface"));
    }

    #[test]
    fn test_react_generator() {
        let generator = ReactGenerator;
        let text = CanvasComponent::Text(TextComponent::new("Hello".to_string()));
        let code = generator.generate(&[text]).unwrap();

        assert!(code.contains("import React"));
        assert!(code.contains("Hello"));
        assert!(code.contains("export function"));
    }

    #[test]
    fn test_vue_generator() {
        let generator = VueGenerator;
        let text = CanvasComponent::Text(TextComponent::new("Hello".to_string()));
        let code = generator.generate(&[text]).unwrap();

        assert!(code.contains("<template>"));
        assert!(code.contains("<script setup"));
        assert!(code.contains("Hello"));
    }

    #[test]
    fn test_css_generator() {
        let generator = CssGenerator;
        let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
        let css = generator.generate(&[button]).unwrap();

        assert!(css.contains(".btn-primary"));
        assert!(css.contains(".btn-secondary"));
    }

    #[test]
    fn test_tailwind_generator() {
        let generator = TailwindHtmlGenerator;
        let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
        let html = generator.generate(&[button]).unwrap();

        assert!(html.contains("class="));
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_svelte_generator() {
        let generator = SvelteGenerator;
        let text = CanvasComponent::Text(TextComponent::new("Hello".to_string()));
        let code = generator.generate(&[text]).unwrap();

        assert!(code.contains("<script"));
        assert!(code.contains("Hello"));
    }
}

// ============================================================================
// Additional Generators
// ============================================================================

/// HTML with Tailwind CSS classes generator
#[derive(Default)]
pub struct TailwindHtmlGenerator;

impl CodeGenerator for TailwindHtmlGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut output = String::new();

        output.push_str("<!-- Generated by Leptos Studio with Tailwind CSS -->\n");
        output.push_str("<!DOCTYPE html>\n");
        output.push_str("<html lang=\"en\">\n");
        output.push_str("<head>\n");
        output.push_str("  <meta charset=\"UTF-8\">\n");
        output.push_str(
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        output.push_str("  <script src=\"https://cdn.tailwindcss.com\"></script>\n");
        output.push_str("  <title>Generated Layout</title>\n");
        output.push_str("</head>\n");
        output.push_str("<body class=\"min-h-screen bg-gray-50\">\n");
        output.push_str("  <main class=\"container mx-auto p-4\">\n");

        for component in components {
            Self::generate_tailwind(component, &mut output, 2)?;
        }

        output.push_str("  </main>\n");
        output.push_str("</body>\n");
        output.push_str("</html>\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "html"
    }
}

impl TailwindHtmlGenerator {
    fn generate_tailwind(
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                let variant_classes = match btn.variant {
                    crate::domain::ButtonVariant::Primary => {
                        "bg-blue-600 hover:bg-blue-700 text-white"
                    }
                    crate::domain::ButtonVariant::Secondary => {
                        "bg-gray-200 hover:bg-gray-300 text-gray-800"
                    }
                    crate::domain::ButtonVariant::Outline => {
                        "bg-transparent border-2 border-blue-600 text-blue-600 hover:bg-blue-50"
                    }
                    crate::domain::ButtonVariant::Ghost => {
                        "bg-transparent text-gray-600 hover:bg-gray-100"
                    }
                };

                let size_classes = match btn.size {
                    crate::domain::ButtonSize::Small => "px-3 py-1 text-sm",
                    crate::domain::ButtonSize::Medium => "px-4 py-2 text-base",
                    crate::domain::ButtonSize::Large => "px-6 py-3 text-lg",
                };

                let disabled_classes = if btn.disabled {
                    "opacity-50 cursor-not-allowed"
                } else {
                    "cursor-pointer"
                };

                output.push_str(&format!(
                    "{}<button class=\"rounded-md font-medium transition-colors {} {} {}\"{}>{}</button>\n",
                    indent,
                    variant_classes,
                    size_classes,
                    disabled_classes,
                    if btn.disabled { " disabled" } else { "" },
                    btn.label
                ));
            }
            CanvasComponent::Text(txt) => {
                let (tag, classes) = match txt.tag {
                    crate::domain::TextTag::H1 => ("h1", "text-4xl font-bold text-gray-900"),
                    crate::domain::TextTag::H2 => ("h2", "text-3xl font-semibold text-gray-800"),
                    crate::domain::TextTag::H3 => ("h3", "text-2xl font-medium text-gray-700"),
                    crate::domain::TextTag::P => ("p", "text-base text-gray-600"),
                    crate::domain::TextTag::Span => ("span", "text-base text-gray-600"),
                };

                output.push_str(&format!(
                    "{}<{} class=\"{}\">{}</{}>\n",
                    indent, tag, classes, txt.content, tag
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
                    "{}<input type=\"{}\" placeholder=\"{}\" class=\"w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent\"{}{}>\n",
                    indent,
                    input_type,
                    inp.placeholder,
                    if inp.required { " required" } else { "" },
                    if inp.disabled { " disabled" } else { "" }
                ));
            }
            CanvasComponent::Container(container) => {
                let layout_classes = match &container.layout {
                    crate::domain::LayoutType::Flex {
                        direction,
                        wrap,
                        align_items,
                        justify_content,
                    } => {
                        let dir = match direction {
                            crate::domain::FlexDirection::Row => "flex-row",
                            crate::domain::FlexDirection::Column => "flex-col",
                        };
                        let wrap_cls = if *wrap { "flex-wrap" } else { "" };

                        let align_cls = match align_items {
                            crate::domain::FlexAlign::Start => "items-start",
                            crate::domain::FlexAlign::Center => "items-center",
                            crate::domain::FlexAlign::End => "items-end",
                            crate::domain::FlexAlign::Stretch => "items-stretch",
                            crate::domain::FlexAlign::Baseline => "items-baseline",
                        };

                        let justify_cls = match justify_content {
                            crate::domain::FlexJustify::Start => "justify-start",
                            crate::domain::FlexJustify::Center => "justify-center",
                            crate::domain::FlexJustify::End => "justify-end",
                            crate::domain::FlexJustify::Between => "justify-between",
                            crate::domain::FlexJustify::Around => "justify-around",
                            crate::domain::FlexJustify::Evenly => "justify-evenly",
                        };

                        format!("flex {} {} {} {}", dir, wrap_cls, align_cls, justify_cls)
                    }
                    crate::domain::LayoutType::Grid { columns, .. } => {
                        format!("grid grid-cols-{}", columns.min(&12))
                    }
                    crate::domain::LayoutType::Stack => "flex flex-col".to_string(),
                };

                let gap_class = format!("gap-{}", (container.gap / 4).clamp(1, 16));
                let padding_class = format!(
                    "pt-{} pr-{} pb-{} pl-{}",
                    (container.padding.top / 4).min(16),
                    (container.padding.right / 4).min(16),
                    (container.padding.bottom / 4).min(16),
                    (container.padding.left / 4).min(16)
                );

                output.push_str(&format!(
                    "{}<div class=\"{} {} {}\">\n",
                    indent, layout_classes, gap_class, padding_class
                ));

                for child in &container.children {
                    Self::generate_tailwind(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!("{}<!-- Custom: {} -->\n", indent, custom.name));
                output.push_str(&format!("{}<div class=\"custom-component\">\n", indent));
                output.push_str(&format!("{}  {}\n", indent, custom.template));
                output.push_str(&format!("{}</div>\n", indent));
            }
        }

        Ok(())
    }
}

/// Svelte component generator
#[derive(Default)]
pub struct SvelteGenerator;

impl CodeGenerator for SvelteGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut output = String::new();

        // Script section
        output.push_str("<script lang=\"ts\">\n");
        output.push_str("  // Generated by Leptos Studio\n");
        output.push_str("  // Props can be added here\n");
        output.push_str("</script>\n\n");

        // Template section
        output.push_str("<div class=\"generated-layout\">\n");

        for component in components {
            Self::generate_svelte(component, &mut output, 1)?;
        }

        output.push_str("</div>\n\n");

        // Style section
        output.push_str("<style>\n");
        output.push_str("  .generated-layout {\n");
        output.push_str("    /* Add your styles here */\n");
        output.push_str("  }\n");
        output.push_str("  \n");
        output.push_str("  .btn {\n");
        output.push_str("    padding: 8px 16px;\n");
        output.push_str("    border-radius: 6px;\n");
        output.push_str("    cursor: pointer;\n");
        output.push_str("    transition: all 0.2s;\n");
        output.push_str("  }\n");
        output.push_str("  \n");
        output.push_str("  .btn-primary {\n");
        output.push_str("    background: #3b82f6;\n");
        output.push_str("    color: white;\n");
        output.push_str("    border: none;\n");
        output.push_str("  }\n");
        output.push_str("  \n");
        output.push_str("  .btn-secondary {\n");
        output.push_str("    background: #e5e7eb;\n");
        output.push_str("    color: #374151;\n");
        output.push_str("    border: none;\n");
        output.push_str("  }\n");
        output.push_str("  \n");
        output.push_str("  input {\n");
        output.push_str("    width: 100%;\n");
        output.push_str("    padding: 8px 12px;\n");
        output.push_str("    border: 1px solid #d1d5db;\n");
        output.push_str("    border-radius: 6px;\n");
        output.push_str("  }\n");
        output.push_str("</style>\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "svelte"
    }
}

impl SvelteGenerator {
    fn generate_svelte(
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                let variant_class = match btn.variant {
                    crate::domain::ButtonVariant::Primary => "btn btn-primary",
                    crate::domain::ButtonVariant::Secondary => "btn btn-secondary",
                    crate::domain::ButtonVariant::Outline => "btn btn-outline",
                    crate::domain::ButtonVariant::Ghost => "btn btn-ghost",
                };

                output.push_str(&format!(
                    "{}<button class=\"{}\" disabled={{{}}}>{}</button>\n",
                    indent, variant_class, btn.disabled, btn.label
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
            CanvasComponent::Container(container) => {
                let style = format!(
                    "display: flex; flex-direction: {}; gap: {}px; padding: {}px {}px {}px {}px;",
                    match &container.layout {
                        crate::domain::LayoutType::Flex { direction, .. } => {
                            match direction {
                                crate::domain::FlexDirection::Row => "row",
                                crate::domain::FlexDirection::Column => "column",
                            }
                        }
                        _ => "column",
                    },
                    container.gap,
                    container.padding.top,
                    container.padding.right,
                    container.padding.bottom,
                    container.padding.left
                );

                output.push_str(&format!("{}<div style=\"{}\">\n", indent, style));

                for child in &container.children {
                    Self::generate_svelte(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!("{}<!-- Custom: {} -->\n", indent, custom.name));
                output.push_str(&format!("{}{{@html `{}`}}\n", indent, custom.template));
            }
        }

        Ok(())
    }
}
