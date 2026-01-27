use leptos_studio::domain::{
    ButtonComponent, CanvasComponent, ContainerComponent, FlexDirection, LayoutType,
};
use leptos_studio::services::export_service::{CodeGenerator, LeptosCodeGenerator};
use leptos_studio::state::ExportPreset;

#[test]
fn test_leptos_generator_structure() {
    let generator = LeptosCodeGenerator::new(ExportPreset::Plain, Vec::new());

    // Create nested structure: Container -> Button
    let mut container = ContainerComponent::new();
    container.layout = LayoutType::Flex {
        direction: FlexDirection::Row,
        wrap: false,
        align_items: Default::default(),
        justify_content: Default::default(),
    };

    let button = CanvasComponent::Button(ButtonComponent::new("Nested Button".to_string()));
    container.children.push(button);

    let components = vec![CanvasComponent::Container(container)];

    let code = generator
        .generate(&components)
        .expect("Failed to generate code");

    println!("{}", code);

    // Check for essential parts
    assert!(code.contains("use leptos::*;"));
    assert!(code.contains("#[component]"));
    assert!(code.contains("pub fn App()"));

    // Check container structure
    assert!(code.contains("class=\"container flex-row\""));

    // Check nested button
    assert!(code.contains("Nested Button"));
    assert!(code.contains("<button"));
}
