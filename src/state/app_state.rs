use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::{CanvasComponent, ComponentId};
use crate::builder::drag_drop::DragState;
use crate::builder::component_library::LibraryComponent;
use super::history::{History, Snapshot};
use super::persistence::Persistable;

/// Canvas-specific state
#[derive(Clone, Copy)]
pub struct CanvasState {
    pub components: RwSignal<Vec<CanvasComponent>>,
    pub selected: RwSignal<Option<ComponentId>>,
    pub history: RwSignal<History>,
    pub drag_state: RwSignal<DragState>,
}

impl CanvasState {
    pub fn new() -> Self {
        Self {
            components: RwSignal::new(Vec::new()),
            selected: RwSignal::new(None),
            history: RwSignal::new(History::new()),
            drag_state: RwSignal::new(DragState::NotDragging),
        }
    }
    
    /// Add a component to the canvas
    pub fn add_component(&self, component: CanvasComponent) {
        self.components.update(|components| {
            components.push(component);
        });
    }
    
    /// Remove a component by ID
    pub fn remove_component(&self, id: &ComponentId) {
        self.components.update(|components| {
            components.retain(|c| c.id() != id);
        });
    }
    
    /// Get a component by ID
    pub fn get_component(&self, id: &ComponentId) -> Option<CanvasComponent> {
        self.components.with(|components| {
            components.iter().find(|c| c.id() == id).cloned()
        })
    }
    
    /// Update a component
    pub fn update_component(&self, id: &ComponentId, new_component: CanvasComponent) {
        self.components.update(|components| {
            if let Some(component) = components.iter_mut().find(|c| c.id() == id) {
                *component = new_component;
            }
        });
    }
    
    /// Record a snapshot for undo/redo
    pub fn record_snapshot(&self) {
        let snapshot = Snapshot::new(
            self.components.get(),
            self.selected.get(),
        );
        self.history.update(|h| h.push(snapshot));
    }
    
    /// Apply a snapshot to the canvas
    pub fn apply_snapshot(&self, snapshot: &Snapshot) {
        self.components.set(snapshot.components.clone());
        self.selected.set(snapshot.selected.clone());
    }
}

impl Default for CanvasState {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NotificationType {
    Success,
    Error,
    Warning,
    Info,
}

/// Notification message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub message: String,
    pub notification_type: NotificationType,
    pub duration: Option<u32>, // milliseconds
}

impl Notification {
    pub fn success(message: String) -> Self {
        Self {
            message,
            notification_type: NotificationType::Success,
            duration: Some(3000),
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            message,
            notification_type: NotificationType::Error,
            duration: Some(5000),
        }
    }
    
    pub fn warning(message: String) -> Self {
        Self {
            message,
            notification_type: NotificationType::Warning,
            duration: Some(4000),
        }
    }
    
    pub fn info(message: String) -> Self {
        Self {
            message,
            notification_type: NotificationType::Info,
            duration: Some(3000),
        }
    }
}

/// Theme options
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Custom,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

/// Responsive preview modes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponsiveMode {
    Desktop,
    Tablet,
    Mobile,
}

impl Default for ResponsiveMode {
    fn default() -> Self {
        Self::Desktop
    }
}

/// UI state (modals, panels, etc)
#[derive(Clone, Copy)]
pub struct UiState {
    pub show_command_palette: RwSignal<bool>,
    pub show_export_modal: RwSignal<bool>,
    pub show_git_panel: RwSignal<bool>,
    pub show_debug_panel: RwSignal<bool>,
    pub notification: RwSignal<Option<Notification>>,
    pub responsive_mode: RwSignal<ResponsiveMode>,
    pub custom_components: RwSignal<Vec<LibraryComponent>>,
    pub component_library: RwSignal<Vec<LibraryComponent>>,
    pub render_count: RwSignal<u32>,
    pub render_time: RwSignal<f64>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            show_command_palette: RwSignal::new(false),
            show_export_modal: RwSignal::new(false),
            show_git_panel: RwSignal::new(false),
            show_debug_panel: RwSignal::new(false),
            notification: RwSignal::new(None),
            responsive_mode: RwSignal::new(ResponsiveMode::default()),
            custom_components: RwSignal::new(Vec::new()),
            component_library: RwSignal::new(Self::default_components()),
            render_count: RwSignal::new(0),
            render_time: RwSignal::new(0.0),
        }
    }
    
    /// Get default component library
    fn default_components() -> Vec<LibraryComponent> {
        vec![
            LibraryComponent {
                name: "Button".to_string(),
                kind: "Button".to_string(),
                template: None,
                category: "Basic".to_string(),
                props_schema: None,
                description: Some("Interactive button component".to_string()),
            },
            LibraryComponent {
                name: "Text".to_string(),
                kind: "Text".to_string(),
                template: None,
                category: "Basic".to_string(),
                props_schema: None,
                description: Some("Text label or paragraph".to_string()),
            },
            LibraryComponent {
                name: "Input".to_string(),
                kind: "Input".to_string(),
                template: None,
                category: "Basic".to_string(),
                props_schema: None,
                description: Some("Text input field".to_string()),
            },
            LibraryComponent {
                name: "Container".to_string(),
                kind: "Container".to_string(),
                template: None,
                category: "Layout".to_string(),
                props_schema: None,
                description: Some("Container for other components".to_string()),
            },
            LibraryComponent {
                name: "Div".to_string(),
                kind: "Container".to_string(),
                template: None,
                category: "Layout".to_string(),
                props_schema: None,
                description: Some("Generic div container".to_string()),
            },
            LibraryComponent {
                name: "Heading".to_string(),
                kind: "Text".to_string(),
                template: None,
                category: "Typography".to_string(),
                props_schema: None,
                description: Some("Heading text (H1-H6)".to_string()),
            },
            LibraryComponent {
                name: "Link".to_string(),
                kind: "Text".to_string(),
                template: None,
                category: "Navigation".to_string(),
                props_schema: None,
                description: Some("Hyperlink component".to_string()),
            },
            LibraryComponent {
                name: "Image".to_string(),
                kind: "Container".to_string(),
                template: None,
                category: "Media".to_string(),
                props_schema: None,
                description: Some("Image component".to_string()),
            },
        ]
    }
    
    /// Show a notification
    pub fn notify(&self, notification: Notification) {
        self.notification.set(Some(notification));
    }
    
    /// Clear notification
    pub fn clear_notification(&self) {
        self.notification.set(None);
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

/// Export preset options
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportPreset {
    Plain,
    ThawUi,
    LeptosMaterial,
    LeptosUse,
}

impl Default for ExportPreset {
    fn default() -> Self {
        Self::Plain
    }
}

/// Settings state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsState {
    pub theme: Theme,
    pub auto_save: bool,
    pub export_preset: ExportPreset,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            auto_save: true,
            export_preset: ExportPreset::default(),
        }
    }
}

impl Default for SettingsState {
    fn default() -> Self {
        Self::new()
    }
}

impl Persistable for SettingsState {
    fn storage_key() -> &'static str {
        "leptos_studio_settings"
    }
}

/// Persistable canvas data (for LocalStorage)
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CanvasData {
    components: Vec<CanvasComponent>,
    selected: Option<ComponentId>,
}

impl Persistable for CanvasData {
    fn storage_key() -> &'static str {
        "leptos_studio_canvas"
    }
}

/// Global application state
#[derive(Clone, Copy)]
pub struct AppState {
    pub canvas: CanvasState,
    pub ui: UiState,
    pub settings: RwSignal<SettingsState>,
}

impl AppState {
    pub fn new() -> Self {
        // Try to load settings from LocalStorage
        let settings = SettingsState::load_or_default();
        
        Self {
            canvas: CanvasState::new(),
            ui: UiState::new(),
            settings: RwSignal::new(settings),
        }
    }
    
    /// Provide AppState as context
    pub fn provide_context() {
        let state = Self::new();
        provide_context(state);
    }
    
    /// Use AppState from context
    pub fn use_context() -> Self {
        expect_context::<Self>()
    }
    
    /// Save settings to LocalStorage
    pub fn save_settings(&self) {
        if let Err(e) = self.settings.get().save() {
            web_sys::console::error_1(&format!("Failed to save settings: {}", e).into());
        }
    }
    
    /// Save canvas data to LocalStorage
    pub fn save(&self) -> Result<(), crate::domain::AppError> {
        let data = CanvasData {
            components: self.canvas.components.get(),
            selected: self.canvas.selected.get(),
        };
        data.save()
    }
    
    /// Load canvas data from LocalStorage
    pub fn load(&self) -> Result<(), crate::domain::AppError> {
        let data = CanvasData::load()?;
        self.canvas.components.set(data.components);
        self.canvas.selected.set(data.selected);
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
