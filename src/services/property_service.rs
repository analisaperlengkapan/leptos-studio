use crate::domain::{
    ButtonComponent, ButtonSize, ButtonVariant, ContainerComponent, FlexDirection, InputComponent,
    InputType, LayoutType, PropValue, TextComponent, TextStyle, TextTag,
};

/// Update a ButtonComponent based on a property name and value.
///
/// This is a small, focused helper that allows PropertyEditor (or other
/// callers) to update button properties in a centralized place, while the UI
/// remains declarative.
pub fn update_button_prop(
    mut button: ButtonComponent,
    name: &str,
    value: PropValue,
) -> ButtonComponent {
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
        _ => {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::warn_1(
                &format!("Unknown property or type mismatch: {} = {:?}", name, value).into(),
            );
        }
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
        _ => {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::warn_1(
                &format!("Unknown property or type mismatch: {} = {:?}", name, value).into(),
            );
        }
    }

    text
}

/// Update an InputComponent based on a property name and value.
pub fn update_input_prop(
    mut input: InputComponent,
    name: &str,
    value: PropValue,
) -> InputComponent {
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
        _ => {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::warn_1(
                &format!("Unknown property or type mismatch: {} = {:?}", name, value).into(),
            );
        }
    }

    input
}

/// Update a ContainerComponent based on a property name and value.
pub fn update_container_prop(
    mut container: ContainerComponent,
    name: &str,
    value: PropValue,
) -> ContainerComponent {
    match (name, value) {
        ("layout", PropValue::String(s)) => {
            let current_layout = container.layout.clone();
            container.layout = match s.as_str() {
                "FlexRow" => match current_layout {
                    LayoutType::Flex { wrap, .. } => LayoutType::Flex {
                        direction: FlexDirection::Row,
                        wrap,
                    },
                    _ => LayoutType::Flex {
                        direction: FlexDirection::Row,
                        wrap: false,
                    },
                },
                "FlexColumn" => match current_layout {
                    LayoutType::Flex { wrap, .. } => LayoutType::Flex {
                        direction: FlexDirection::Column,
                        wrap,
                    },
                    _ => LayoutType::Flex {
                        direction: FlexDirection::Column,
                        wrap: false,
                    },
                },
                "Grid" => match current_layout {
                    LayoutType::Grid { columns, rows } => LayoutType::Grid { columns, rows },
                    _ => LayoutType::Grid {
                        columns: 2,
                        rows: 2,
                    },
                },
                "Stack" => LayoutType::Stack,
                _ => current_layout,
            };
        }
        ("gap", PropValue::Number(n)) => {
            let value = if n.is_finite() && n >= 0.0 {
                n.round() as u32
            } else {
                container.gap
            };
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
        _ => {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::warn_1(
                &format!("Unknown property or type mismatch: {} = {:?}", name, value).into(),
            );
        }
    }

    container
}

#[cfg(test)]
mod tests {
    use super::*;

    // Button tests
    #[test]
    fn test_update_button_label() {
        let button = ButtonComponent::new("Original".to_string());
        let updated = update_button_prop(button, "label", PropValue::String("Updated".to_string()));
        assert_eq!(updated.label, "Updated");
    }

    #[test]
    fn test_update_button_variant() {
        let button = ButtonComponent::new("Test".to_string());

        let updated = update_button_prop(
            button.clone(),
            "variant",
            PropValue::String("Secondary".to_string()),
        );
        assert_eq!(updated.variant, ButtonVariant::Secondary);

        let updated = update_button_prop(
            button.clone(),
            "variant",
            PropValue::String("Outline".to_string()),
        );
        assert_eq!(updated.variant, ButtonVariant::Outline);

        let updated = update_button_prop(
            button.clone(),
            "variant",
            PropValue::String("Ghost".to_string()),
        );
        assert_eq!(updated.variant, ButtonVariant::Ghost);

        // Unknown variant should keep original
        let updated =
            update_button_prop(button, "variant", PropValue::String("Unknown".to_string()));
        assert_eq!(updated.variant, ButtonVariant::Primary);
    }

    #[test]
    fn test_update_button_size() {
        let button = ButtonComponent::new("Test".to_string());

        let updated = update_button_prop(
            button.clone(),
            "size",
            PropValue::String("Small".to_string()),
        );
        assert_eq!(updated.size, ButtonSize::Small);

        let updated = update_button_prop(
            button.clone(),
            "size",
            PropValue::String("Large".to_string()),
        );
        assert_eq!(updated.size, ButtonSize::Large);
    }

    #[test]
    fn test_update_button_disabled() {
        let button = ButtonComponent::new("Test".to_string());
        assert!(!button.disabled);

        let updated = update_button_prop(button, "disabled", PropValue::Boolean(true));
        assert!(updated.disabled);
    }

    #[test]
    fn test_update_button_wrong_type() {
        let button = ButtonComponent::new("Test".to_string());
        // Passing a boolean to a string field should not change anything
        let updated = update_button_prop(button.clone(), "label", PropValue::Boolean(true));
        assert_eq!(updated.label, "Test");
    }

    // Text tests
    #[test]
    fn test_update_text_content() {
        let text = TextComponent::new("Original".to_string());
        let updated = update_text_prop(
            text,
            "content",
            PropValue::String("New Content".to_string()),
        );
        assert_eq!(updated.content, "New Content");
    }

    #[test]
    fn test_update_text_style() {
        let text = TextComponent::new("Test".to_string());

        let updated = update_text_prop(
            text.clone(),
            "style",
            PropValue::String("Heading1".to_string()),
        );
        assert_eq!(updated.style, TextStyle::Heading1);

        let updated = update_text_prop(
            text.clone(),
            "style",
            PropValue::String("Heading2".to_string()),
        );
        assert_eq!(updated.style, TextStyle::Heading2);

        let updated = update_text_prop(
            text.clone(),
            "style",
            PropValue::String("Caption".to_string()),
        );
        assert_eq!(updated.style, TextStyle::Caption);
    }

    #[test]
    fn test_update_text_tag() {
        let text = TextComponent::new("Test".to_string());

        let updated = update_text_prop(text.clone(), "tag", PropValue::String("H1".to_string()));
        assert_eq!(updated.tag, TextTag::H1);

        let updated = update_text_prop(text.clone(), "tag", PropValue::String("H2".to_string()));
        assert_eq!(updated.tag, TextTag::H2);

        let updated = update_text_prop(text.clone(), "tag", PropValue::String("Span".to_string()));
        assert_eq!(updated.tag, TextTag::Span);
    }

    // Input tests
    #[test]
    fn test_update_input_placeholder() {
        let input = InputComponent::new();
        let updated = update_input_prop(
            input,
            "placeholder",
            PropValue::String("Enter text...".to_string()),
        );
        assert_eq!(updated.placeholder, "Enter text...");
    }

    #[test]
    fn test_update_input_type() {
        let input = InputComponent::new();

        let updated = update_input_prop(
            input.clone(),
            "input_type",
            PropValue::String("Password".to_string()),
        );
        assert_eq!(updated.input_type, InputType::Password);

        let updated = update_input_prop(
            input.clone(),
            "input_type",
            PropValue::String("Email".to_string()),
        );
        assert_eq!(updated.input_type, InputType::Email);

        let updated = update_input_prop(
            input.clone(),
            "input_type",
            PropValue::String("Number".to_string()),
        );
        assert_eq!(updated.input_type, InputType::Number);

        let updated = update_input_prop(input, "input_type", PropValue::String("Tel".to_string()));
        assert_eq!(updated.input_type, InputType::Tel);
    }

    #[test]
    fn test_update_input_required() {
        let input = InputComponent::new();
        assert!(!input.required);

        let updated = update_input_prop(input, "required", PropValue::Boolean(true));
        assert!(updated.required);
    }

    #[test]
    fn test_update_input_disabled() {
        let input = InputComponent::new();
        assert!(!input.disabled);

        let updated = update_input_prop(input, "disabled", PropValue::Boolean(true));
        assert!(updated.disabled);
    }

    // Container tests
    #[test]
    fn test_update_container_layout_flex_row() {
        let container = ContainerComponent::new();
        let updated = update_container_prop(
            container,
            "layout",
            PropValue::String("FlexRow".to_string()),
        );

        match updated.layout {
            LayoutType::Flex { direction, .. } => {
                assert_eq!(direction, FlexDirection::Row);
            }
            _ => panic!("Expected Flex layout"),
        }
    }

    #[test]
    fn test_update_container_layout_grid() {
        let container = ContainerComponent::new();
        let updated =
            update_container_prop(container, "layout", PropValue::String("Grid".to_string()));

        match updated.layout {
            LayoutType::Grid { columns, rows } => {
                assert_eq!(columns, 2);
                assert_eq!(rows, 2);
            }
            _ => panic!("Expected Grid layout"),
        }
    }

    #[test]
    fn test_update_container_layout_stack() {
        let container = ContainerComponent::new();
        let updated =
            update_container_prop(container, "layout", PropValue::String("Stack".to_string()));
        assert_eq!(updated.layout, LayoutType::Stack);
    }

    #[test]
    fn test_update_container_gap() {
        let container = ContainerComponent::new();

        let updated = update_container_prop(container.clone(), "gap", PropValue::Number(16.0));
        assert_eq!(updated.gap, 16);

        // Test with decimal - should round
        let updated = update_container_prop(container.clone(), "gap", PropValue::Number(10.7));
        assert_eq!(updated.gap, 11);

        // Negative values should be ignored
        let updated = update_container_prop(container.clone(), "gap", PropValue::Number(-5.0));
        assert_eq!(updated.gap, container.gap);

        // NaN should be ignored
        let updated = update_container_prop(container.clone(), "gap", PropValue::Number(f64::NAN));
        assert_eq!(updated.gap, container.gap);
    }

    #[test]
    fn test_update_container_padding() {
        let container = ContainerComponent::new();

        let updated =
            update_container_prop(container.clone(), "padding_top", PropValue::Number(10.0));
        assert_eq!(updated.padding.top, 10);

        let updated =
            update_container_prop(container.clone(), "padding_right", PropValue::Number(20.0));
        assert_eq!(updated.padding.right, 20);

        let updated =
            update_container_prop(container.clone(), "padding_bottom", PropValue::Number(30.0));
        assert_eq!(updated.padding.bottom, 30);

        let updated =
            update_container_prop(container.clone(), "padding_left", PropValue::Number(40.0));
        assert_eq!(updated.padding.left, 40);
    }

    #[test]
    fn test_update_unknown_property() {
        let button = ButtonComponent::new("Test".to_string());
        let original_label = button.label.clone();

        // Unknown property should not change anything
        let updated = update_button_prop(
            button,
            "unknown_prop",
            PropValue::String("value".to_string()),
        );
        assert_eq!(updated.label, original_label);
    }
}
