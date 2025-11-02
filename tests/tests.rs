// Additional unit tests for Leptos Studio

use leptos_studio::builder::canvas::CanvasComponent;
use leptos_studio::builder::component_library::{LibraryComponent, ResponsiveMode, Theme};

#[test]
fn test_container_nested_serialization() {
    let container = CanvasComponent::Container {
        children: vec![
            CanvasComponent::Button {
                label: "Child Button".to_string(),
            },
            CanvasComponent::Text {
                content: "Child Text".to_string(),
            },
        ],
    };
    let json = serde_json::to_string(&container).unwrap();
    let de: CanvasComponent = serde_json::from_str(&json).unwrap();
    match de {
        CanvasComponent::Container { children } => {
            assert_eq!(children.len(), 2);
        }
        _ => panic!("Container deserialization failed"),
    }
}

#[test]
fn test_theme_enum() {
    let light = Theme::Light;
    let dark = Theme::Dark;
    let custom = Theme::Custom;

    assert_ne!(light, dark);
    assert_ne!(dark, custom);
    assert_ne!(light, custom);
}

#[test]
fn test_responsive_mode() {
    let desktop = ResponsiveMode::Desktop;
    let tablet = ResponsiveMode::Tablet;
    let mobile = ResponsiveMode::Mobile;

    assert_ne!(desktop, tablet);
    assert_ne!(tablet, mobile);
    assert_ne!(desktop, mobile);
}

#[test]
fn test_library_component_creation() {
    let comp = LibraryComponent {
        name: "TestComp".to_string(),
        kind: "Button".to_string(),
        template: None,
        category: "Test".to_string(),
        props_schema: None,
        description: Some("Test component".to_string()),
    };

    assert_eq!(comp.name, "TestComp");
    assert_eq!(comp.kind, "Button");
    assert!(comp.template.is_none());
    assert_eq!(comp.category, "Test");
    assert_eq!(comp.description.unwrap(), "Test component");
}
