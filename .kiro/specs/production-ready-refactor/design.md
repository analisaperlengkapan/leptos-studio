# Design Document

## Overview

Leptos Studio akan direfactor menjadi aplikasi production-ready dengan arsitektur modular, type-safe, performant, dan mengikuti best practices Rust/Leptos. Design ini fokus pada separation of concerns, extensibility, dan maintainability.

### Design Principles

1. **Modularity**: Setiap modul memiliki tanggung jawab yang jelas dan terisolasi
2. **Type Safety**: Gunakan type system Rust untuk mencegah bugs di compile time
3. **Performance**: Optimasi reactivity dan minimize re-renders
4. **Extensibility**: Plugin architecture untuk custom components dan export presets
5. **Developer Experience**: Clear APIs, good documentation, helpful error messages

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Application Layer                     │
│  (app.rs - Main component, global state, event handlers)    │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐   ┌────────▼────────┐   ┌──────▼──────┐
│  UI Components │   │  State Manager  │   │   Services  │
│   (builder/)   │   │   (state.rs)    │   │ (services/) │
└────────────────┘   └─────────────────┘   └─────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │   Core Domain     │
                    │   (domain/)       │
                    └───────────────────┘
```

### Module Structure


```
src/
├── app.rs                      # Main application component
├── lib.rs                      # Library root & re-exports
│
├── domain/                     # Core domain models (pure Rust)
│   ├── mod.rs
│   ├── component.rs            # Component types & traits
│   ├── layout.rs               # Layout structure
│   ├── validation.rs           # Validation logic
│   └── error.rs                # Custom error types
│
├── state/                      # State management
│   ├── mod.rs
│   ├── app_state.rs            # Global application state
│   ├── canvas_state.rs         # Canvas-specific state
│   ├── history.rs              # Undo/redo implementation
│   └── persistence.rs          # LocalStorage integration
│
├── services/                   # Business logic services
│   ├── mod.rs
│   ├── export_service.rs       # Code generation
│   ├── validation_service.rs  # Input validation
│   ├── git_service.rs          # Git operations
│   └── component_registry.rs  # Component management
│
├── builder/                    # UI components
│   ├── mod.rs
│   ├── canvas/                 # Canvas module
│   │   ├── mod.rs
│   │   ├── canvas.rs           # Main canvas component
│   │   ├── drag_drop.rs        # Drag & drop logic
│   │   └── renderer.rs         # Component rendering
│   ├── sidebar/                # Sidebar module
│   │   ├── mod.rs
│   │   ├── sidebar.rs          # Main sidebar
│   │   ├── component_library.rs
│   │   └── theme_selector.rs
│   ├── property_editor/        # Property editor module
│   │   ├── mod.rs
│   │   ├── property_editor.rs
│   │   └── validators.rs
│   ├── preview/                # Preview module
│   │   ├── mod.rs
│   │   └── preview.rs
│   ├── toolbar/                # Toolbar module
│   │   ├── mod.rs
│   │   └── toolbar.rs
│   ├── command_palette.rs      # Command palette
│   ├── breadcrumb.rs           # Breadcrumb navigation
│   ├── keyboard.rs             # Keyboard shortcuts
│   ├── design_tokens.rs        # Design system
│   └── snackbar.rs             # Notifications
│
├── components/                 # Reusable UI primitives
│   ├── mod.rs
│   ├── button.rs
│   ├── input.rs
│   ├── text.rs
│   └── container.rs
│
└── utils/                      # Utility functions
    ├── mod.rs
    ├── clipboard.rs
    ├── dom.rs
    └── format.rs
```

## Components and Interfaces

### 1. Domain Layer

#### Component Types


```rust
// domain/component.rs

/// Core component trait for all UI components
pub trait Component: Clone + Serialize + DeserializeOwned {
    fn component_type(&self) -> ComponentType;
    fn validate(&self) -> Result<(), ValidationError>;
    fn render_preview(&self) -> String;
}

/// Component type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    Button,
    Text,
    Input,
    Container,
    Custom,
}

/// Main component enum with all variants
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CanvasComponent {
    Button(ButtonComponent),
    Text(TextComponent),
    Input(InputComponent),
    Container(ContainerComponent),
    Custom(CustomComponent),
}

/// Individual component structs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ButtonComponent {
    pub id: ComponentId,
    pub label: String,
    pub variant: ButtonVariant,
    pub disabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextComponent {
    pub id: ComponentId,
    pub content: String,
    pub style: TextStyle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputComponent {
    pub id: ComponentId,
    pub placeholder: String,
    pub input_type: InputType,
    pub required: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContainerComponent {
    pub id: ComponentId,
    pub children: Vec<CanvasComponent>,
    pub layout: LayoutType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomComponent {
    pub id: ComponentId,
    pub name: String,
    pub template: String,
    pub props: HashMap<String, PropValue>,
}

/// Component ID for unique identification
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(Uuid);

impl ComponentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
```

#### Validation System


```rust
// domain/validation.rs

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Component name is empty")]
    EmptyName,
    
    #[error("Component name '{0}' is invalid: must be a valid Rust identifier")]
    InvalidName(String),
    
    #[error("Component name '{0}' already exists")]
    DuplicateName(String),
    
    #[error("Template is empty")]
    EmptyTemplate,
    
    #[error("Template is invalid: {0}")]
    InvalidTemplate(String),
    
    #[error("Property '{0}' is required but not provided")]
    MissingRequiredProperty(String),
    
    #[error("Property '{0}' has invalid value: {1}")]
    InvalidPropertyValue(String, String),
}

pub trait Validator<T> {
    fn validate(&self, value: &T) -> Result<(), ValidationError>;
}

/// Component name validator
pub struct ComponentNameValidator;

impl Validator<String> for ComponentNameValidator {
    fn validate(&self, name: &String) -> Result<(), ValidationError> {
        if name.trim().is_empty() {
            return Err(ValidationError::EmptyName);
        }
        
        // Must start with letter or underscore
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return Err(ValidationError::InvalidName(name.clone()));
        }
        
        // Must contain only alphanumeric and underscore
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(ValidationError::InvalidName(name.clone()));
        }
        
        Ok(())
    }
}

/// HTML template validator
pub struct HtmlTemplateValidator;

impl Validator<String> for HtmlTemplateValidator {
    fn validate(&self, template: &String) -> Result<(), ValidationError> {
        if template.trim().is_empty() {
            return Err(ValidationError::EmptyTemplate);
        }
        
        if template.len() < 5 {
            return Err(ValidationError::InvalidTemplate(
                "Template too short".to_string()
            ));
        }
        
        if !template.contains('<') || !template.contains('>') {
            return Err(ValidationError::InvalidTemplate(
                "Must contain at least one HTML tag".to_string()
            ));
        }
        
        // TODO: Add HTML sanitization check
        
        Ok(())
    }
}
```

#### Error Types


```rust
// domain/error.rs

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Git error: {0}")]
    Git(String),
    
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    
    #[error("Export error: {0}")]
    Export(String),
}

pub type AppResult<T> = Result<T, AppError>;
```

### 2. State Management Layer

#### Application State

```rust
// state/app_state.rs

use leptos::*;

/// Global application state
#[derive(Clone)]
pub struct AppState {
    pub canvas: CanvasState,
    pub ui: UiState,
    pub settings: SettingsState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            canvas: CanvasState::new(),
            ui: UiState::new(),
            settings: SettingsState::load_or_default(),
        }
    }
    
    pub fn provide_context() {
        provide_context(create_rw_signal(Self::new()));
    }
    
    pub fn use_context() -> RwSignal<Self> {
        expect_context::<RwSignal<Self>>()
    }
}

/// Canvas-specific state
#[derive(Clone)]
pub struct CanvasState {
    pub components: RwSignal<Vec<CanvasComponent>>,
    pub selected: RwSignal<Option<ComponentId>>,
    pub history: RwSignal<History>,
}

/// UI state (modals, panels, etc)
#[derive(Clone)]
pub struct UiState {
    pub show_command_palette: RwSignal<bool>,
    pub show_export_modal: RwSignal<bool>,
    pub notification: RwSignal<Option<Notification>>,
    pub theme: RwSignal<Theme>,
    pub responsive_mode: RwSignal<ResponsiveMode>,
}

/// Settings state
#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub theme: Theme,
    pub auto_save: bool,
    pub export_preset: ExportPreset,
}
```

#### History Management (Undo/Redo)


```rust
// state/history.rs

use std::collections::VecDeque;

const MAX_HISTORY_SIZE: usize = 50;

#[derive(Clone)]
pub struct History {
    undo_stack: VecDeque<Snapshot>,
    redo_stack: VecDeque<Snapshot>,
}

#[derive(Clone)]
pub struct Snapshot {
    components: Vec<CanvasComponent>,
    selected: Option<ComponentId>,
    timestamp: f64,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
    
    pub fn push(&mut self, snapshot: Snapshot) {
        // Clear redo stack when new action is performed
        self.redo_stack.clear();
        
        // Add to undo stack
        self.undo_stack.push_back(snapshot);
        
        // Limit stack size
        if self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.pop_front();
        }
    }
    
    pub fn undo(&mut self) -> Option<Snapshot> {
        if let Some(snapshot) = self.undo_stack.pop_back() {
            self.redo_stack.push_back(snapshot.clone());
            self.undo_stack.back().cloned()
        } else {
            None
        }
    }
    
    pub fn redo(&mut self) -> Option<Snapshot> {
        if let Some(snapshot) = self.redo_stack.pop_back() {
            self.undo_stack.push_back(snapshot.clone());
            Some(snapshot)
        } else {
            None
        }
    }
    
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
```

#### Persistence Layer

```rust
// state/persistence.rs

use web_sys::Storage;

pub trait Persistable: Serialize + DeserializeOwned {
    fn storage_key() -> &'static str;
    
    fn save(&self) -> AppResult<()> {
        let storage = get_local_storage()?;
        let json = serde_json::to_string(self)
            .map_err(|e| AppError::Serialization(e.to_string()))?;
        storage.set_item(Self::storage_key(), &json)
            .map_err(|_| AppError::Storage("Failed to save".to_string()))?;
        Ok(())
    }
    
    fn load() -> AppResult<Self> {
        let storage = get_local_storage()?;
        let json = storage.get_item(Self::storage_key())
            .map_err(|_| AppError::Storage("Failed to load".to_string()))?
            .ok_or_else(|| AppError::Storage("No data found".to_string()))?;
        serde_json::from_str(&json)
            .map_err(|e| AppError::Serialization(e.to_string()))
    }
}

fn get_local_storage() -> AppResult<Storage> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .ok_or_else(|| AppError::Storage("LocalStorage not available".to_string()))
}
```

### 3. Services Layer

#### Export Service


```rust
// services/export_service.rs

pub trait CodeGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String>;
    fn file_extension(&self) -> &str;
}

pub struct LeptosCodeGenerator {
    preset: ExportPreset,
}

impl CodeGenerator for LeptosCodeGenerator {
    fn generate(&self, components: &[CanvasComponent]) -> AppResult<String> {
        let mut output = String::new();
        
        // Add imports based on preset
        output.push_str(&self.generate_imports());
        output.push_str("\n\n");
        
        // Add component definition
        output.push_str("#[component]\n");
        output.push_str("pub fn GeneratedView() -> impl IntoView {\n");
        output.push_str("    view! {\n");
        
        // Generate component code
        for component in components {
            self.generate_component(&mut output, component, 2)?;
        }
        
        output.push_str("    }\n");
        output.push_str("}\n");
        
        Ok(output)
    }
    
    fn file_extension(&self) -> &str {
        "rs"
    }
}

impl LeptosCodeGenerator {
    fn generate_imports(&self) -> String {
        match self.preset {
            ExportPreset::Plain => "use leptos::*;".to_string(),
            ExportPreset::ThawUi => "use leptos::*;\nuse thaw::*;".to_string(),
            ExportPreset::LeptosMaterial => "use leptos::*;\nuse leptos_material::*;".to_string(),
            ExportPreset::LeptosUse => "use leptos::*;\nuse leptos_use::*;".to_string(),
        }
    }
    
    fn generate_component(
        &self,
        output: &mut String,
        component: &CanvasComponent,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "    ".repeat(indent_level);
        
        match component {
            CanvasComponent::Button(btn) => {
                output.push_str(&format!(
                    "{}<button>{{r#\"{}\"#}}</button>\n",
                    indent, btn.label
                ));
            }
            CanvasComponent::Text(text) => {
                output.push_str(&format!(
                    "{}<span>{{r#\"{}\"#}}</span>\n",
                    indent, text.content
                ));
            }
            CanvasComponent::Input(input) => {
                output.push_str(&format!(
                    "{}<input placeholder=\"{}\" />\n",
                    indent, input.placeholder
                ));
            }
            CanvasComponent::Container(container) => {
                output.push_str(&format!("{}<div class=\"container\">\n", indent));
                for child in &container.children {
                    self.generate_component(output, child, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!(
                    "{}// Custom: {}\n{}/* {} */\n",
                    indent, custom.name, indent, custom.template
                ));
            }
        }
        
        Ok(())
    }
}

pub struct HtmlCodeGenerator;
pub struct MarkdownCodeGenerator;
pub struct JsonCodeGenerator;

// Similar implementations for other generators...
```

#### Component Registry Service


```rust
// services/component_registry.rs

pub struct ComponentRegistry {
    components: HashMap<String, LibraryComponent>,
    validators: HashMap<String, Box<dyn Validator<LibraryComponent>>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            components: HashMap::new(),
            validators: HashMap::new(),
        };
        
        // Register built-in components
        registry.register_builtin_components();
        
        registry
    }
    
    pub fn register(&mut self, component: LibraryComponent) -> AppResult<()> {
        // Validate component
        if let Some(validator) = self.validators.get(&component.kind) {
            validator.validate(&component)?;
        }
        
        // Check for duplicates
        if self.components.contains_key(&component.name) {
            return Err(AppError::Validation(
                ValidationError::DuplicateName(component.name.clone())
            ));
        }
        
        self.components.insert(component.name.clone(), component);
        Ok(())
    }
    
    pub fn unregister(&mut self, name: &str) -> AppResult<LibraryComponent> {
        self.components.remove(name)
            .ok_or_else(|| AppError::ComponentNotFound(name.to_string()))
    }
    
    pub fn get(&self, name: &str) -> Option<&LibraryComponent> {
        self.components.get(name)
    }
    
    pub fn list(&self) -> Vec<&LibraryComponent> {
        self.components.values().collect()
    }
    
    pub fn search(&self, query: &str) -> Vec<&LibraryComponent> {
        let query_lower = query.to_lowercase();
        self.components.values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    fn register_builtin_components(&mut self) {
        // Register Button, Text, Input, Container
        // ...
    }
}
```

### 4. UI Components Layer

#### Canvas Component

```rust
// builder/canvas/canvas.rs

#[component]
pub fn Canvas() -> impl IntoView {
    let app_state = AppState::use_context();
    let canvas_state = create_memo(move |_| app_state.get().canvas);
    
    let on_drop = move |ev: DragEvent| {
        handle_drop(ev, canvas_state);
    };
    
    let on_drag_over = move |ev: DragEvent| {
        ev.prevent_default();
    };
    
    view! {
        <div 
            class="canvas"
            on:drop=on_drop
            on:dragover=on_drag_over
        >
            <For
                each=move || canvas_state.get().components.get()
                key=|comp| comp.id()
                children=move |comp| {
                    view! {
                        <ComponentRenderer component=comp />
                    }
                }
            />
        </div>
    }
}

fn handle_drop(ev: DragEvent, canvas_state: Memo<CanvasState>) {
    ev.prevent_default();
    
    if let Some(data_transfer) = ev.data_transfer() {
        if let Ok(component_type) = data_transfer.get_data("component") {
            // Create new component based on type
            let component = create_component_from_type(&component_type);
            
            // Add to canvas
            canvas_state.get().components.update(|comps| {
                comps.push(component);
            });
            
            // Save to history
            save_snapshot(canvas_state);
        }
    }
}
```

#### Property Editor Component


```rust
// builder/property_editor/property_editor.rs

#[component]
pub fn PropertyEditor() -> impl IntoView {
    let app_state = AppState::use_context();
    let selected_id = create_memo(move |_| {
        app_state.get().canvas.selected.get()
    });
    
    let selected_component = create_memo(move |_| {
        let id = selected_id.get()?;
        let components = app_state.get().canvas.components.get();
        components.iter().find(|c| c.id() == id).cloned()
    });
    
    view! {
        <div class="property-editor">
            <h3>"Properties"</h3>
            {move || match selected_component.get() {
                Some(component) => {
                    view! {
                        <PropertyForm component=component />
                    }
                }
                None => {
                    view! {
                        <p>"Select a component to edit properties"</p>
                    }
                }
            }}
        </div>
    }
}

#[component]
fn PropertyForm(component: CanvasComponent) -> impl IntoView {
    match component {
        CanvasComponent::Button(btn) => view! {
            <ButtonPropertyForm button=btn />
        },
        CanvasComponent::Text(text) => view! {
            <TextPropertyForm text=text />
        },
        // ... other component types
        _ => view! { <div>"Unsupported component"</div> }
    }
}
```

## Data Models

### Component Models

```rust
// Detailed component models with all properties

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ButtonComponent {
    pub id: ComponentId,
    pub label: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub on_click: Option<String>, // Event handler code
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextComponent {
    pub id: ComponentId,
    pub content: String,
    pub style: TextStyle,
    pub tag: TextTag,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TextStyle {
    Heading1,
    Heading2,
    Heading3,
    Paragraph,
    Caption,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TextTag {
    H1, H2, H3, P, Span,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContainerComponent {
    pub id: ComponentId,
    pub children: Vec<CanvasComponent>,
    pub layout: LayoutType,
    pub gap: u32,
    pub padding: Spacing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LayoutType {
    Flex { direction: FlexDirection, wrap: bool },
    Grid { columns: u32, rows: u32 },
    Stack,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spacing {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}
```

### Library Component Model


```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LibraryComponent {
    pub name: String,
    pub kind: ComponentType,
    pub category: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub template: Option<String>, // For custom components
    pub props_schema: Vec<PropSchema>,
    pub default_props: HashMap<String, PropValue>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropSchema {
    pub name: String,
    pub prop_type: PropType,
    pub required: bool,
    pub default_value: Option<PropValue>,
    pub description: Option<String>,
    pub validation: Option<ValidationRule>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropType {
    String,
    Number,
    Boolean,
    Enum(Vec<String>),
    Object,
    Array,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValidationRule {
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    Range { min: f64, max: f64 },
    Custom(String), // Custom validation function name
}
```

## Error Handling

### Error Handling Strategy

1. **Use Result types**: All fallible operations return `Result<T, AppError>`
2. **Custom error types**: Use `thiserror` for descriptive errors
3. **Error propagation**: Use `?` operator for clean error propagation
4. **User-facing errors**: Convert technical errors to user-friendly messages
5. **Logging**: Log errors with appropriate levels (error, warn, info)

### Error Display

```rust
// User-facing error messages
impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            AppError::Validation(e) => e.to_string(),
            AppError::Serialization(_) => {
                "Failed to save/load data. Please try again.".to_string()
            }
            AppError::Storage(_) => {
                "Storage error. Please check browser settings.".to_string()
            }
            AppError::Git(msg) => {
                format!("Git operation failed: {}", msg)
            }
            AppError::ComponentNotFound(name) => {
                format!("Component '{}' not found", name)
            }
            AppError::Export(_) => {
                "Failed to export code. Please try again.".to_string()
            }
        }
    }
}
```

## Testing Strategy

### Unit Tests

- Test domain logic (validation, component creation)
- Test state management (history, persistence)
- Test services (export, component registry)
- Use property-based testing for edge cases

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_name_validation() {
        let validator = ComponentNameValidator;
        
        assert!(validator.validate(&"ValidName".to_string()).is_ok());
        assert!(validator.validate(&"_valid".to_string()).is_ok());
        assert!(validator.validate(&"valid123".to_string()).is_ok());
        
        assert!(validator.validate(&"".to_string()).is_err());
        assert!(validator.validate(&"123invalid".to_string()).is_err());
        assert!(validator.validate(&"invalid-name".to_string()).is_err());
    }
    
    #[test]
    fn test_history_undo_redo() {
        let mut history = History::new();
        let snapshot1 = create_test_snapshot();
        let snapshot2 = create_test_snapshot();
        
        history.push(snapshot1.clone());
        history.push(snapshot2.clone());
        
        assert!(history.can_undo());
        assert!(!history.can_redo());
        
        let undone = history.undo();
        assert!(undone.is_some());
        assert!(history.can_redo());
        
        let redone = history.redo();
        assert!(redone.is_some());
    }
}
```

### Integration Tests


- Test complete user flows (drag-drop, edit, export)
- Test keyboard shortcuts
- Test state persistence
- Test error scenarios

```rust
#[cfg(test)]
mod integration_tests {
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_drag_drop_component() {
        // Setup test environment
        let app_state = AppState::new();
        
        // Simulate drag-drop
        let component_type = "Button";
        let result = add_component_to_canvas(&app_state, component_type);
        
        assert!(result.is_ok());
        assert_eq!(app_state.canvas.components.get().len(), 1);
    }
    
    #[wasm_bindgen_test]
    fn test_undo_redo_flow() {
        let app_state = AppState::new();
        
        // Add component
        add_component_to_canvas(&app_state, "Button").unwrap();
        assert_eq!(app_state.canvas.components.get().len(), 1);
        
        // Undo
        undo(&app_state);
        assert_eq!(app_state.canvas.components.get().len(), 0);
        
        // Redo
        redo(&app_state);
        assert_eq!(app_state.canvas.components.get().len(), 1);
    }
}
```

### Performance Tests

- Measure render time for large layouts
- Test memory usage with many components
- Benchmark serialization/deserialization
- Profile WASM bundle size

## Performance Optimization

### Reactivity Optimization

1. **Fine-grained signals**: Use separate signals for independent state
2. **Memoization**: Use `create_memo` for derived values
3. **Selective updates**: Only update affected components
4. **Batching**: Batch multiple state updates

```rust
// Good: Fine-grained reactivity
let label = create_rw_signal("Button".to_string());
let disabled = create_rw_signal(false);

// Bad: Coarse-grained reactivity
let button_state = create_rw_signal(ButtonState { label, disabled });
```

### Rendering Optimization

1. **Keyed lists**: Use stable keys for `<For>` components
2. **Lazy loading**: Load components on-demand
3. **Virtual scrolling**: For large component lists
4. **Debouncing**: Debounce expensive operations (search, validation)

```rust
// Use stable keys
<For
    each=move || components.get()
    key=|comp| comp.id()  // Stable ComponentId
    children=move |comp| { /* ... */ }
/>

// Debounce search
let search_query = create_rw_signal(String::new());
let debounced_search = use_debounce(search_query, 300.0);
```

### Bundle Size Optimization

1. **Code splitting**: Split by route/feature
2. **Tree shaking**: Remove unused code
3. **Compression**: Enable gzip/brotli
4. **Lazy imports**: Dynamic imports for heavy features

```toml
# Cargo.toml optimizations
[profile.release]
lto = true
codegen-units = 1
opt-level = "z"  # Optimize for size
panic = "abort"
```

## Security Considerations

### Input Sanitization


```rust
// Sanitize HTML templates
pub fn sanitize_html(html: &str) -> String {
    // Remove script tags
    let re = regex::Regex::new(r"<script[^>]*>.*?</script>").unwrap();
    let sanitized = re.replace_all(html, "");
    
    // Remove event handlers
    let re = regex::Regex::new(r#"\s+on\w+\s*=\s*["'][^"']*["']"#).unwrap();
    let sanitized = re.replace_all(&sanitized, "");
    
    sanitized.to_string()
}

// Validate component names (prevent code injection)
pub fn validate_component_name(name: &str) -> Result<(), ValidationError> {
    let validator = ComponentNameValidator;
    validator.validate(&name.to_string())
}
```

### Content Security Policy

```html
<!-- index.html -->
<meta http-equiv="Content-Security-Policy" 
      content="default-src 'self'; 
               script-src 'self' 'wasm-unsafe-eval'; 
               style-src 'self' 'unsafe-inline';">
```

### Data Validation

- Validate all user inputs before processing
- Sanitize HTML templates before rendering
- Validate JSON before deserialization
- Check file sizes for LocalStorage limits

## Accessibility

### WCAG 2.1 Compliance

1. **Keyboard Navigation**: All features accessible via keyboard
2. **Screen Reader Support**: Proper ARIA labels and roles
3. **Color Contrast**: Meet WCAG AA standards (4.5:1 for text)
4. **Focus Management**: Clear focus indicators
5. **Semantic HTML**: Use proper HTML elements

```rust
// Accessible button component
view! {
    <button
        aria-label="Add component"
        aria-pressed=is_active
        tabindex="0"
        on:click=on_click
        on:keydown=on_keydown
    >
        {label}
    </button>
}
```

### Keyboard Shortcuts

- All shortcuts documented
- No conflicts with browser shortcuts
- Shortcuts disabled in input fields
- Visual indicators for active shortcuts

## Design Tokens

### CSS Custom Properties

```css
:root {
    /* Colors */
    --color-primary: #3b82f6;
    --color-secondary: #64748b;
    --color-success: #10b981;
    --color-error: #ef4444;
    --color-warning: #f59e0b;
    
    /* Spacing */
    --spacing-xs: 0.25rem;
    --spacing-sm: 0.5rem;
    --spacing-md: 1rem;
    --spacing-lg: 1.5rem;
    --spacing-xl: 2rem;
    
    /* Typography */
    --font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    --font-size-xs: 0.75rem;
    --font-size-sm: 0.875rem;
    --font-size-md: 1rem;
    --font-size-lg: 1.125rem;
    --font-size-xl: 1.25rem;
    
    /* Shadows */
    --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
    --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.1);
    --shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.1);
    
    /* Border Radius */
    --radius-sm: 0.25rem;
    --radius-md: 0.375rem;
    --radius-lg: 0.5rem;
    
    /* Transitions */
    --transition-fast: 150ms ease;
    --transition-base: 200ms ease;
    --transition-slow: 300ms ease;
}

/* Dark theme */
[data-theme="dark"] {
    --color-bg: #1a1a1a;
    --color-text: #ffffff;
    --color-border: #333333;
}
```

## Migration Strategy

### Phase 1: Foundation (Week 1-2)

1. Create new module structure
2. Implement domain models
3. Implement error types
4. Setup validation system

### Phase 2: State Management (Week 2-3)

1. Refactor state management
2. Implement history system
3. Add persistence layer
4. Migrate existing state

### Phase 3: Services (Week 3-4)

1. Extract business logic to services
2. Implement component registry
3. Refactor export system
4. Add validation service

### Phase 4: UI Refactor (Week 4-6)

1. Refactor canvas component
2. Refactor sidebar component
3. Refactor property editor
4. Update all UI components

### Phase 5: Testing & Polish (Week 6-7)

1. Add unit tests
2. Add integration tests
3. Performance optimization
4. Documentation

### Phase 6: Production Ready (Week 7-8)

1. Security audit
2. Accessibility audit
3. Final optimizations
4. Release preparation

## Backward Compatibility

- Support loading old layout format
- Migrate data on first load
- Provide migration guide
- Keep old export formats available

```rust
// Data migration
pub fn migrate_layout(old_format: OldLayout) -> Result<Vec<CanvasComponent>, MigrationError> {
    // Convert old format to new format
    // ...
}
```

## Documentation

### Code Documentation

- Document all public APIs with rustdoc
- Include examples in documentation
- Document error cases
- Add architecture diagrams

### User Documentation

- Getting started guide
- Feature documentation
- Keyboard shortcuts reference
- Troubleshooting guide
- Video tutorials

### Developer Documentation

- Architecture overview
- Contributing guide
- Development setup
- Testing guide
- Release process
