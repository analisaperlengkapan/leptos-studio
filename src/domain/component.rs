use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::error::ValidationError;
use super::validation::Validator;

/// Component ID for unique identification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(Uuid);

impl ComponentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for ComponentId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Component type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    Button,
    Text,
    Input,
    Container,
    Image,
    Card,
    Custom,
}

/// Core component trait for all UI components
pub trait Component: Clone {
    fn component_type(&self) -> ComponentType;
    fn id(&self) -> &ComponentId;
    fn validate(&self) -> Result<(), ValidationError>;
}

/// Button variants
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
}

/// Button sizes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// Button component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ButtonComponent {
    pub id: ComponentId,
    pub label: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub on_click: Option<String>,
}

impl ButtonComponent {
    pub fn new(label: String) -> Self {
        Self {
            id: ComponentId::new(),
            label,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
            on_click: None,
        }
    }
}

impl Component for ButtonComponent {
    fn component_type(&self) -> ComponentType {
        ComponentType::Button
    }

    fn id(&self) -> &ComponentId {
        &self.id
    }

    fn validate(&self) -> Result<(), ValidationError> {
        if self.label.trim().is_empty() {
            return Err(ValidationError::InvalidPropertyValue(
                "label".to_string(),
                "Button label cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// Text styles
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextStyle {
    Heading1,
    Heading2,
    Heading3,
    Body,
    Caption,
}

/// Text HTML tags
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextTag {
    H1,
    H2,
    H3,
    P,
    Span,
}

/// Text component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextComponent {
    pub id: ComponentId,
    pub content: String,
    pub style: TextStyle,
    pub tag: TextTag,
}

impl TextComponent {
    pub fn new(content: String) -> Self {
        Self {
            id: ComponentId::new(),
            content,
            style: TextStyle::Body,
            tag: TextTag::P,
        }
    }
}

impl Component for TextComponent {
    fn component_type(&self) -> ComponentType {
        ComponentType::Text
    }

    fn id(&self) -> &ComponentId {
        &self.id
    }

    fn validate(&self) -> Result<(), ValidationError> {
        // Text content can be empty (for placeholder text)
        Ok(())
    }
}

/// Input types
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Tel,
}

/// Input component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputComponent {
    pub id: ComponentId,
    pub placeholder: String,
    pub input_type: InputType,
    pub required: bool,
    pub disabled: bool,
}

impl InputComponent {
    pub fn new() -> Self {
        Self {
            id: ComponentId::new(),
            placeholder: String::new(),
            input_type: InputType::Text,
            required: false,
            disabled: false,
        }
    }
}

impl Default for InputComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for InputComponent {
    fn component_type(&self) -> ComponentType {
        ComponentType::Input
    }

    fn id(&self) -> &ComponentId {
        &self.id
    }

    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

/// Layout types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutType {
    Flex {
        direction: FlexDirection,
        wrap: bool,
        #[serde(default)]
        align_items: FlexAlign,
        #[serde(default)]
        justify_content: FlexJustify,
    },
    Grid {
        columns: u32,
        rows: u32,
    },
    Stack,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FlexAlign {
    #[default]
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FlexJustify {
    #[default]
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

/// Spacing
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Spacing {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

/// Container component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerComponent {
    pub id: ComponentId,
    pub children: Vec<CanvasComponent>,
    pub layout: LayoutType,
    pub gap: u32,
    pub padding: Spacing,
}

impl ContainerComponent {
    pub fn new() -> Self {
        Self {
            id: ComponentId::new(),
            children: Vec::new(),
            layout: LayoutType::Flex {
                direction: FlexDirection::Column,
                wrap: false,
                align_items: FlexAlign::default(),
                justify_content: FlexJustify::default(),
            },
            gap: 8,
            padding: Spacing::default(),
        }
    }
}

impl Default for ContainerComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for ContainerComponent {
    fn component_type(&self) -> ComponentType {
        ComponentType::Container
    }

    fn id(&self) -> &ComponentId {
        &self.id
    }

    fn validate(&self) -> Result<(), ValidationError> {
        // Validate all children
        for child in &self.children {
            child.validate()?;
        }
        Ok(())
    }
}

/// Image component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageComponent {
    pub id: ComponentId,
    pub src: String,
    pub alt: String,
    pub width: Option<String>,
    pub height: Option<String>,
}

impl ImageComponent {
    pub fn new(src: String, alt: String) -> Self {
        Self {
            id: ComponentId::new(),
            src,
            alt,
            width: None,
            height: None,
        }
    }
}

impl Component for ImageComponent {
    fn component_type(&self) -> ComponentType {
        ComponentType::Image
    }

    fn id(&self) -> &ComponentId {
        &self.id
    }

    fn validate(&self) -> Result<(), ValidationError> {
        if self.src.trim().is_empty() {
            return Err(ValidationError::InvalidPropertyValue(
                "src".to_string(),
                "Image source URL cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// Card component - A pre-styled container
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardComponent {
    pub id: ComponentId,
    pub children: Vec<CanvasComponent>,
    pub padding: u32,
    pub shadow: bool,
    pub border: bool,
    pub border_radius: u32,
}

impl CardComponent {
    pub fn new() -> Self {
        Self {
            id: ComponentId::new(),
            children: Vec::new(),
            padding: 16,
            shadow: true,
            border: true,
            border_radius: 8,
        }
    }
}

impl Default for CardComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for CardComponent {
    fn component_type(&self) -> ComponentType {
        ComponentType::Card
    }

    fn id(&self) -> &ComponentId {
        &self.id
    }

    fn validate(&self) -> Result<(), ValidationError> {
        for child in &self.children {
            child.validate()?;
        }
        Ok(())
    }
}

/// Property value types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PropValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

/// Custom component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomComponent {
    pub id: ComponentId,
    pub name: String,
    pub template: String,
    pub props: HashMap<String, PropValue>,
}

impl CustomComponent {
    pub fn new(name: String, template: String) -> Self {
        Self {
            id: ComponentId::new(),
            name,
            template,
            props: HashMap::new(),
        }
    }
}

impl Component for CustomComponent {
    fn component_type(&self) -> ComponentType {
        ComponentType::Custom
    }

    fn id(&self) -> &ComponentId {
        &self.id
    }

    fn validate(&self) -> Result<(), ValidationError> {
        use super::validation::{ComponentNameValidator, HtmlTemplateValidator};

        // Validate name
        let name_validator = ComponentNameValidator;
        name_validator.validate(&self.name)?;

        // Validate template
        let template_validator = HtmlTemplateValidator;
        template_validator.validate(&self.template)?;

        Ok(())
    }
}

/// Main component enum with all variants
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CanvasComponent {
    Button(ButtonComponent),
    Text(TextComponent),
    Input(InputComponent),
    Container(ContainerComponent),
    Image(ImageComponent),
    Card(CardComponent),
    Custom(CustomComponent),
}

impl CanvasComponent {
    pub fn id(&self) -> &ComponentId {
        match self {
            CanvasComponent::Button(c) => c.id(),
            CanvasComponent::Text(c) => c.id(),
            CanvasComponent::Input(c) => c.id(),
            CanvasComponent::Container(c) => c.id(),
            CanvasComponent::Image(c) => c.id(),
            CanvasComponent::Card(c) => c.id(),
            CanvasComponent::Custom(c) => c.id(),
        }
    }

    pub fn component_type(&self) -> ComponentType {
        match self {
            CanvasComponent::Button(c) => c.component_type(),
            CanvasComponent::Text(c) => c.component_type(),
            CanvasComponent::Input(c) => c.component_type(),
            CanvasComponent::Container(c) => c.component_type(),
            CanvasComponent::Image(c) => c.component_type(),
            CanvasComponent::Card(c) => c.component_type(),
            CanvasComponent::Custom(c) => c.component_type(),
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            CanvasComponent::Button(c) => c.validate(),
            CanvasComponent::Text(c) => c.validate(),
            CanvasComponent::Input(c) => c.validate(),
            CanvasComponent::Container(c) => c.validate(),
            CanvasComponent::Image(c) => c.validate(),
            CanvasComponent::Card(c) => c.validate(),
            CanvasComponent::Custom(c) => c.validate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_id_creation() {
        let id1 = ComponentId::new();
        let id2 = ComponentId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_button_component_validation() {
        let button = ButtonComponent::new("Click me".to_string());
        assert!(button.validate().is_ok());

        let empty_button = ButtonComponent::new("".to_string());
        assert!(empty_button.validate().is_err());
    }

    #[test]
    fn test_custom_component_validation() {
        let valid_custom =
            CustomComponent::new("MyComponent".to_string(), "<div>Hello</div>".to_string());
        assert!(valid_custom.validate().is_ok());

        let invalid_name =
            CustomComponent::new("123Invalid".to_string(), "<div>Hello</div>".to_string());
        assert!(invalid_name.validate().is_err());

        let invalid_template =
            CustomComponent::new("ValidName".to_string(), "No tags here".to_string());
        assert!(invalid_template.validate().is_err());
    }

    #[test]
    fn test_container_component_validation() {
        let mut container = ContainerComponent::new();
        assert!(container.validate().is_ok());

        // Add valid child
        container
            .children
            .push(CanvasComponent::Button(ButtonComponent::new(
                "Button".to_string(),
            )));
        assert!(container.validate().is_ok());

        // Add invalid child
        container
            .children
            .push(CanvasComponent::Button(ButtonComponent::new(
                "".to_string(),
            )));
        assert!(container.validate().is_err());
    }
}
