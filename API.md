# API Documentation

Internal API reference for Leptos Studio core types and functions.

## Core Types

### CanvasComponent

Represents a component that can be placed on the canvas.

```rust
pub enum CanvasComponent {
    Button { label: String },
    Text { content: String },
    Input { placeholder: String },
    Container { children: Vec<CanvasComponent> },
    Custom { name: String },
}
```

**Variants:**
- `Button` - A clickable button with a label
- `Text` - Static text display
- `Input` - Text input field with placeholder
- `Container` - A container that can hold child components
- `Custom` - User-defined custom component

### LibraryComponent

Represents a component definition in the component library.

```rust
pub struct LibraryComponent {
    pub name: String,
    pub kind: String,
    pub template: Option<String>,
    pub category: String,
    pub props_schema: Option<Vec<PropSchema>>,
    pub description: Option<String>,
}
```

**Fields:**
- `name` - Component name (must be valid Rust identifier for custom components)
- `kind` - Component type ("Button", "Text", "Input", "Container", "Custom")
- `template` - HTML template for custom components
- `category` - Component category for organization
- `props_schema` - Optional schema defining component properties
- `description` - Optional description of the component

### PropSchema

Defines a property schema for components.

```rust
pub struct PropSchema {
    pub name: String,
    pub prop_type: String,
    pub required: bool,
    pub description: Option<String>,
}
```

**Fields:**
- `name` - Property name
- `prop_type` - Type of property ("string", "number", "bool")
- `required` - Whether the property is required
- `description` - Optional description

### SelectedComponent

Tracks the currently selected component on the canvas.

```rust
pub struct SelectedComponent {
    pub idx: Option<usize>,
}
```

### ResponsiveMode

Preview modes for responsive design.

```rust
pub enum ResponsiveMode {
    Desktop,
    Tablet,
    Mobile,
}
```

### Theme

Available theme options.

```rust
pub enum Theme {
    Light,
    Dark,
    Custom,
}
```

### ExportPreset

Code generation presets for different Leptos UI libraries.

```rust
pub enum ExportPreset {
    Plain,
    ThawUi,
    LeptosMaterial,
    LeptosUse,
}
```

## State Management

### Global Signals

The application uses Leptos signals for reactive state management:

- `components: RwSignal<Vec<CanvasComponent>>` - Current layout components
- `selected: RwSignal<SelectedComponent>` - Currently selected component
- `undo_stack: RwSignal<Vec<Vec<CanvasComponent>>>` - Undo history
- `redo_stack: RwSignal<Vec<Vec<CanvasComponent>>>` - Redo history
- `custom_components: RwSignal<Vec<LibraryComponent>>` - Custom component library
- `theme: RwSignal<Theme>` - Current theme
- `responsive_mode: RwSignal<ResponsiveMode>` - Current responsive preview mode

## Core Functions

### Canvas Module

#### `Canvas(props: CanvasProps) -> impl IntoView`

Main canvas component that displays and manages the layout.

**Props:**
- `components` - Signal containing canvas components
- `selected` - Signal for selected component
- `on_select` - Callback when component is selected
- `on_drop` - Callback when component is dropped
- `responsive_mode` - Current responsive mode
- `theme` - Current theme

### Export Module

#### `generate_leptos_code(components: &[CanvasComponent], custom_components: &[LibraryComponent], preset: ExportPreset) -> String`

Generates Leptos code from the canvas layout.

**Parameters:**
- `components` - Array of canvas components
- `custom_components` - Array of custom component definitions
- `preset` - Export preset to use

**Returns:** Generated Leptos code as a string

#### `generate_html_code(components: &[CanvasComponent], custom_components: &[LibraryComponent], preset: ExportPreset) -> String`

Generates HTML code from the canvas layout.

#### `generate_markdown_code(components: &[CanvasComponent]) -> String`

Generates Markdown documentation from the canvas layout.

### Sidebar Module

#### `Sidebar(props: SidebarProps) -> impl IntoView`

Sidebar component containing the component library and property editor.

**Props:**
- `components` - Signal containing canvas components
- `selected` - Signal for selected component
- `custom_components` - Signal for custom component library
- `theme` - Current theme
- `responsive_mode` - Current responsive mode
- Various callbacks for component actions

### Preview Module

#### `Preview(props: PreviewProps) -> impl IntoView`

Live preview panel showing the rendered output.

**Props:**
- `components` - Components to preview
- `custom_components` - Custom components for rendering

## Serialization

All core types implement `serde::Serialize` and `serde::Deserialize` for:
- Saving layouts to localStorage
- Exporting/importing project files
- Component library management

## Usage Examples

### Creating a Button Component

```rust
let button = CanvasComponent::Button {
    label: "Click Me".to_string()
};
```

### Adding a Component to Canvas

```rust
let mut components = components_signal.get();
components.push(new_component);
components_signal.set(components);
```

### Exporting Code

```rust
let code = generate_leptos_code(
    &components_signal.get(),
    &custom_components_signal.get(),
    ExportPreset::Plain
);
```

### Saving Layout

```rust
if let Ok(json) = serde_json::to_string(&components_signal.get()) {
    window()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item("leptos_studio_layout", &json)
        .ok();
}
```

## Testing

Core functionality is tested in the `tests/` directory:
- `tests/basic.rs` - Basic component tests
- `tests/integration.rs` - Integration tests for export and serialization
- `tests/tests.rs` - Additional test cases

Run tests with:
```bash
cargo test
```

---

For more information, see the source code documentation with:
```bash
cargo doc --open
```
