use crate::domain::{
    ButtonComponent,
    ButtonVariant,
    ButtonSize,
    InputComponent,
    InputType,
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
