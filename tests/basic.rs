// Basic unit tests for CanvasComponent and undo/redo logic

use leptos_studio::builder::canvas::CanvasComponent;

#[test]
fn test_canvas_component_serialization() {
    let btn = CanvasComponent::Button {
        label: "Test".to_string(),
    };
    let json = serde_json::to_string(&btn).unwrap();
    let de: CanvasComponent = serde_json::from_str(&json).unwrap();
    match de {
        CanvasComponent::Button { label } => assert_eq!(label, "Test"),
        _ => panic!("Deserialization failed"),
    }
}

#[test]
fn test_undo_redo_stack() {
    let mut undo_stack = Vec::new();
    let mut redo_stack = Vec::new();
    let mut state = vec![CanvasComponent::Button {
        label: "A".to_string(),
    }];
    // Simulate change
    undo_stack.push(state.clone());
    state.push(CanvasComponent::Text {
        content: "B".to_string(),
    });
    // Undo
    if let Some(prev) = undo_stack.pop() {
        redo_stack.push(state.clone());
        state = prev;
    }
    assert_eq!(state.len(), 1);
    // Redo
    if let Some(next) = redo_stack.pop() {
        undo_stack.push(state.clone());
        state = next;
    }
    assert_eq!(state.len(), 2);
}
