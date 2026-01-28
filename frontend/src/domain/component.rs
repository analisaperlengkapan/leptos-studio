use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::error::ValidationError;
use super::validation::Validator;

/// Component ID for unique identification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(Uuid);

/// Animation types
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnimationType {
    #[default]
    None,
    FadeIn,
    SlideInUp,
    SlideInDown,
    SlideInLeft,
    SlideInRight,
    Bounce,
    ZoomIn,
    Pulse,
}

/// Animation configuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    pub animation_type: AnimationType,
    pub duration: f32, // in seconds
    pub delay: f32,    // in seconds
    pub infinite: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            animation_type: AnimationType::None,
            duration: 0.3,
            delay: 0.0,
            infinite: false,
        }
    }
}

impl Animation {
    pub fn to_css_string(&self) -> String {
        if self.animation_type == AnimationType::None {
            return String::new();
        }

        let anim_name = match self.animation_type {
            AnimationType::None => "",
            AnimationType::FadeIn => "fadeIn",
            AnimationType::SlideInUp => "slideInUp",
            AnimationType::SlideInDown => "slideInDown",
            AnimationType::SlideInLeft => "slideInLeft",
            AnimationType::SlideInRight => "slideInRight",
            AnimationType::Bounce => "bounce",
            AnimationType::ZoomIn => "zoomIn",
            AnimationType::Pulse => "pulse",
        };

        let iteration = if self.infinite { "infinite" } else { "1" };

        format!(
            "animation: {} {}s ease-in-out {}s {} both;",
            anim_name, self.duration, self.delay, iteration
        )
    }
}

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
    Select,
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
    #[serde(default)]
    pub animation: Option<Animation>,
    #[serde(default)]
    pub bindings: HashMap<String, String>,
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
            animation: None,
            bindings: HashMap::new(),
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
    #[serde(default)]
    pub animation: Option<Animation>,
    #[serde(default)]
    pub bindings: HashMap<String, String>,
}

impl TextComponent {
    pub fn new(content: String) -> Self {
        Self {
            id: ComponentId::new(),
            content,
            style: TextStyle::Body,
            tag: TextTag::P,
            animation: None,
            bindings: HashMap::new(),
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
    pub on_change: Option<String>,
    pub on_input: Option<String>,
    #[serde(default)]
    pub animation: Option<Animation>,
    #[serde(default)]
    pub bindings: HashMap<String, String>,
}

impl InputComponent {
    pub fn new() -> Self {
        Self {
            id: ComponentId::new(),
            placeholder: String::new(),
            input_type: InputType::Text,
            required: false,
            disabled: false,
            on_change: None,
            on_input: None,
            animation: None,
            bindings: HashMap::new(),
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

/// Select component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectComponent {
    pub id: ComponentId,
    pub options: String, // Comma separated values
    pub placeholder: String,
    pub disabled: bool,
    pub on_change: Option<String>,
    #[serde(default)]
    pub animation: Option<Animation>,
}

impl SelectComponent {
    pub fn new() -> Self {
        Self {
            id: ComponentId::new(),
            options: "Option 1, Option 2, Option 3".to_string(),
            placeholder: "Select an option".to_string(),
            disabled: false,
            on_change: None,
            animation: None,
        }
    }
}

impl Default for SelectComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for SelectComponent {
    fn component_type(&self) -> ComponentType {
        ComponentType::Select
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
    pub on_click: Option<String>,
    #[serde(default)]
    pub animation: Option<Animation>,
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
            on_click: None,
            animation: None,
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
    pub on_click: Option<String>,
    #[serde(default)]
    pub animation: Option<Animation>,
}

impl ImageComponent {
    pub fn new(src: String, alt: String) -> Self {
        Self {
            id: ComponentId::new(),
            src,
            alt,
            width: None,
            height: None,
            on_click: None,
            animation: None,
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
    pub on_click: Option<String>,
    #[serde(default)]
    pub animation: Option<Animation>,
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
            on_click: None,
            animation: None,
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
    Select(SelectComponent),
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
            CanvasComponent::Select(c) => c.id(),
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
            CanvasComponent::Select(c) => c.component_type(),
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
            CanvasComponent::Select(c) => c.validate(),
            CanvasComponent::Custom(c) => c.validate(),
        }
    }

    pub fn duplicate_with_new_id(&self) -> Self {
        match self {
            CanvasComponent::Button(c) => {
                let mut new_c = c.clone();
                new_c.id = ComponentId::new();
                CanvasComponent::Button(new_c)
            }
            CanvasComponent::Text(c) => {
                let mut new_c = c.clone();
                new_c.id = ComponentId::new();
                CanvasComponent::Text(new_c)
            }
            CanvasComponent::Input(c) => {
                let mut new_c = c.clone();
                new_c.id = ComponentId::new();
                CanvasComponent::Input(new_c)
            }
            CanvasComponent::Select(c) => {
                let mut new_c = c.clone();
                new_c.id = ComponentId::new();
                CanvasComponent::Select(new_c)
            }
            CanvasComponent::Image(c) => {
                let mut new_c = c.clone();
                new_c.id = ComponentId::new();
                CanvasComponent::Image(new_c)
            }
            CanvasComponent::Container(c) => {
                let mut new_c = c.clone();
                new_c.id = ComponentId::new();
                new_c.children = c
                    .children
                    .iter()
                    .map(|child| child.duplicate_with_new_id())
                    .collect();
                CanvasComponent::Container(new_c)
            }
            CanvasComponent::Card(c) => {
                let mut new_c = c.clone();
                new_c.id = ComponentId::new();
                new_c.children = c
                    .children
                    .iter()
                    .map(|child| child.duplicate_with_new_id())
                    .collect();
                CanvasComponent::Card(new_c)
            }
            CanvasComponent::Custom(c) => {
                let mut new_c = c.clone();
                new_c.id = ComponentId::new();
                CanvasComponent::Custom(new_c)
            }
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

    #[test]
    fn test_duplicate_with_new_id() {
        let mut container = ContainerComponent::new();
        let button = ButtonComponent::new("Button".to_string());
        container.children.push(CanvasComponent::Button(button));

        let original = CanvasComponent::Container(container);
        let duplicated = original.duplicate_with_new_id();

        assert_ne!(original.id(), duplicated.id());

        if let CanvasComponent::Container(orig_c) = &original {
            if let CanvasComponent::Container(dup_c) = &duplicated {
                assert_eq!(orig_c.children.len(), dup_c.children.len());
                assert_ne!(orig_c.children[0].id(), dup_c.children[0].id());
            }
        }
    }
}
