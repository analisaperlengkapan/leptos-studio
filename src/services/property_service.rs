use crate::domain::{
    ButtonComponent,
    ButtonVariant,
    ButtonSize,
    ContainerComponent,
    FlexDirection,
    InputComponent,
    InputType,
    LayoutType,
    PropValue,
    TextComponent,
    TextStyle,
    TextTag,
};

/// Update a ButtonComponent based on a property name and value.
///
/// This is a small, focused helper that allows PropertyEditor (or other
/// callers) to update button properties in a centralized place, while the UI
/// remains declarative.
pub fn update_button_prop(mut button: ButtonComponent, name: &str, value: PropValue) -> ButtonComponent {
    match (name, value) {
        ("label", PropValue::String(s)) => {
            button.label = s;
        }
        ("variant", PropValue::String(s)) => {
            button.variant = match s.as_str() {
                "Primary" => ButtonVariant::Primary,
                "Secondary" => ButtonVariant::Secondary,
                "Outline" => ButtonVariant::Outline,
                "Ghost" => ButtonVariant::Ghost,
                _ => button.variant,
            };
        }
        ("size", PropValue::String(s)) => {
            button.size = match s.as_str() {
                "Small" => ButtonSize::Small,
                "Medium" => ButtonSize::Medium,
                "Large" => ButtonSize::Large,
                _ => button.size,
            };
        }
        ("disabled", PropValue::Boolean(b)) => {
            button.disabled = b;
        }
        // Ignore mismatched types or unknown property names for now.
        _ => {}
    }

    button
}

/// Update a TextComponent based on a property name and value.
pub fn update_text_prop(mut text: TextComponent, name: &str, value: PropValue) -> TextComponent {
    match (name, value) {
        ("content", PropValue::String(s)) => {
            text.content = s;
        }
        ("style", PropValue::String(s)) => {
            text.style = match s.as_str() {
                "Heading1" => TextStyle::Heading1,
                "Heading2" => TextStyle::Heading2,
                "Heading3" => TextStyle::Heading3,
                "Body" => TextStyle::Body,
                "Caption" => TextStyle::Caption,
                _ => text.style,
            };
        }
        ("tag", PropValue::String(s)) => {
            text.tag = match s.as_str() {
                "H1" => TextTag::H1,
                "H2" => TextTag::H2,
                "H3" => TextTag::H3,
                "P" => TextTag::P,
                "Span" => TextTag::Span,
                _ => text.tag,
            };
        }
        _ => {}
    }

    text
}

/// Update an InputComponent based on a property name and value.
pub fn update_input_prop(mut input: InputComponent, name: &str, value: PropValue) -> InputComponent {
    match (name, value) {
        ("placeholder", PropValue::String(s)) => {
            input.placeholder = s;
        }
        ("input_type", PropValue::String(s)) => {
            input.input_type = match s.as_str() {
                "Text" => InputType::Text,
                "Password" => InputType::Password,
                "Email" => InputType::Email,
                "Number" => InputType::Number,
                "Tel" => InputType::Tel,
                _ => input.input_type,
            };
        }
        ("required", PropValue::Boolean(b)) => {
            input.required = b;
        }
        ("disabled", PropValue::Boolean(b)) => {
            input.disabled = b;
        }
        _ => {}
    }

    input
}

/// Update a ContainerComponent based on a property name and value.
pub fn update_container_prop(mut container: ContainerComponent, name: &str, value: PropValue) -> ContainerComponent {
    match (name, value) {
        ("layout", PropValue::String(s)) => {
            let current_layout = container.layout.clone();
            container.layout = match s.as_str() {
                "FlexRow" => {
                    match current_layout {
                        LayoutType::Flex { wrap, .. } => LayoutType::Flex { direction: FlexDirection::Row, wrap },
                        _ => LayoutType::Flex { direction: FlexDirection::Row, wrap: false },
                    }
                }
                "FlexColumn" => {
                    match current_layout {
                        LayoutType::Flex { wrap, .. } => LayoutType::Flex { direction: FlexDirection::Column, wrap },
                        _ => LayoutType::Flex { direction: FlexDirection::Column, wrap: false },
                    }
                }
                "Grid" => {
                    match current_layout {
                        LayoutType::Grid { columns, rows } => LayoutType::Grid { columns, rows },
                        _ => LayoutType::Grid { columns: 2, rows: 2 },
                    }
                }
                "Stack" => LayoutType::Stack,
                _ => current_layout,
            };
        }
        ("gap", PropValue::Number(n)) => {
            let value = if n.is_finite() && n >= 0.0 { n.round() as u32 } else { container.gap };
            container.gap = value;
        }
        ("padding_top", PropValue::Number(n)) => {
            if n.is_finite() && n >= 0.0 {
                container.padding.top = n.round() as u32;
            }
        }
        ("padding_right", PropValue::Number(n)) => {
            if n.is_finite() && n >= 0.0 {
                container.padding.right = n.round() as u32;
            }
        }
        ("padding_bottom", PropValue::Number(n)) => {
            if n.is_finite() && n >= 0.0 {
                container.padding.bottom = n.round() as u32;
            }
        }
        ("padding_left", PropValue::Number(n)) => {
            if n.is_finite() && n >= 0.0 {
                container.padding.left = n.round() as u32;
            }
        }
        _ => {}
    }

    container
}
