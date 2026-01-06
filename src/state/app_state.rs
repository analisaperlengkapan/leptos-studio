use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use super::history::{History, Snapshot};
use super::persistence::Persistable;
use super::project::Project;
use crate::builder::component_library::{LibraryComponent, builtin_library_components};
use crate::builder::drag_drop::DragState;
use crate::domain::{CanvasComponent, ComponentId};

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

    /// Add a component to the canvas (internal helper without snapshot)
    pub fn add_component_without_snapshot(&self, component: CanvasComponent) {
        self.components.update(|components| {
            components.push(component);
        });
    }

    /// Add a component to the canvas
    pub fn add_component(&self, component: CanvasComponent) {
        self.record_snapshot("Add Component");
        self.add_component_without_snapshot(component);
    }

    /// Add a child component to a specific parent
    pub fn add_child_component(&self, parent_id: &ComponentId, component: CanvasComponent) {
        self.record_snapshot("Add Child Component");
        self.add_child_component_without_snapshot(parent_id, component);
    }

    pub fn add_child_component_without_snapshot(
        &self,
        parent_id: &ComponentId,
        component: CanvasComponent,
    ) {
        self.components.update(|components| {
            Self::add_child_recursive(&mut components[..], parent_id, component);
        });
    }

    fn add_child_recursive(
        components: &mut [CanvasComponent],
        parent_id: &ComponentId,
        child: CanvasComponent,
    ) -> bool {
        for comp in components.iter_mut() {
            if comp.id() == parent_id {
                if let CanvasComponent::Container(container) = comp {
                    container.children.push(child);
                    return true;
                }
                return false;
            }
            if let CanvasComponent::Container(container) = comp
                && Self::add_child_recursive(&mut container.children[..], parent_id, child.clone())
            {
                return true;
            }
        }
        false
    }

    /// Remove a component by ID
    pub fn remove_component(&self, id: &ComponentId) {
        self.record_snapshot("Remove Component");
        self.components.update(|components| {
            Self::remove_recursive(components, id);
        });
    }

    fn remove_recursive(components: &mut Vec<CanvasComponent>, id: &ComponentId) {
        components.retain(|c| c.id() != id);
        for comp in components.iter_mut() {
            if let CanvasComponent::Container(container) = comp {
                Self::remove_recursive(&mut container.children, id);
            }
        }
    }

    /// Get a component by ID
    pub fn get_component(&self, id: &ComponentId) -> Option<CanvasComponent> {
        self.components
            .with(|components| Self::get_recursive(components, id))
    }

    fn get_recursive(components: &[CanvasComponent], id: &ComponentId) -> Option<CanvasComponent> {
        for comp in components {
            if comp.id() == id {
                return Some(comp.clone());
            }
            if let CanvasComponent::Container(container) = comp
                && let Some(found) = Self::get_recursive(&container.children, id)
            {
                return Some(found);
            }
        }
        None
    }

    /// Update a component
    pub fn update_component(&self, id: &ComponentId, new_component: CanvasComponent) {
        self.components.update(|components| {
            Self::update_recursive(components, id, new_component);
        });
    }

    fn update_recursive(
        components: &mut [CanvasComponent],
        id: &ComponentId,
        new_component: CanvasComponent,
    ) -> bool {
        for comp in components.iter_mut() {
            if comp.id() == id {
                *comp = new_component;
                return true;
            }
            if let CanvasComponent::Container(container) = comp
                && Self::update_recursive(&mut container.children[..], id, new_component.clone())
            {
                return true;
            }
        }
        false
    }

    /// Update a component and record a snapshot
    pub fn update_component_with_snapshot(
        &self,
        id: &ComponentId,
        new_component: CanvasComponent,
        description: &str,
    ) {
        self.record_snapshot(description);
        self.update_component(id, new_component);
    }

    /// Record a snapshot for undo/redo
    pub fn record_snapshot(&self, description: &str) {
        let snapshot = Snapshot::new(
            self.components.get(),
            self.selected.get(),
            description.to_string(),
        );
        self.history.update(|h| h.push(snapshot));
    }

    /// Apply a snapshot to the canvas
    pub fn apply_snapshot(&self, snapshot: &Snapshot) {
        self.components.set(snapshot.components.clone());
        self.selected.set(snapshot.selected);
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    Custom,
}

/// Responsive preview modes for the canvas
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ResponsiveMode {
    #[default]
    Desktop,
    Tablet,
    TabletLandscape,
    Mobile,
    MobileLandscape,
}

/// UI state (modals, panels, etc)
#[derive(Clone, Copy)]
pub struct UiState {
    pub show_command_palette: RwSignal<bool>,
    pub show_export_modal: RwSignal<bool>,
    pub show_git_panel: RwSignal<bool>,
    pub show_debug_panel: RwSignal<bool>,
    pub preview_mode: RwSignal<bool>,
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
            preview_mode: RwSignal::new(false),
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
        let mut components = builtin_library_components();
        components.extend_from_slice(&[
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
        ]);
        components
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ExportPreset {
    #[default]
    Plain,
    ThawUi,
    LeptosMaterial,
    LeptosUse,
}

/// Settings state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub project_name: RwSignal<String>,
    pub last_modified: RwSignal<f64>,
}

impl AppState {
    pub fn new() -> Self {
        // Try to load settings from LocalStorage
        let settings = SettingsState::load_or_default();

        let state = Self {
            canvas: CanvasState::new(),
            ui: UiState::new(),
            settings: RwSignal::new(settings),
            project_name: RwSignal::new("Untitled Project".to_string()),
            last_modified: RwSignal::new(js_sys::Date::now()),
        };

        // Setup reactivity for last_modified
        let components = state.canvas.components;
        let last_modified = state.last_modified;
        Effect::new(move |_| {
            components.track();
            last_modified.set(js_sys::Date::now());
        });

        state
    }

    /// Provide AppState as context
    pub fn provide_context() {
        let state = Self::new();
        provide_context(state);
    }

    /// Expect AppState from context (panics if missing)
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }

    /// Try to get AppState from context
    pub fn try_use_context() -> Option<Self> {
        use_context::<Self>()
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

    /// Build a Project from current state
    pub fn to_project(&self) -> Project {
        Project::new(
            self.project_name.get(),
            self.canvas.components.get(),
            self.settings.get(),
        )
    }

    /// Apply a Project to the current state
    pub fn apply_project(&self, project: Project) {
        self.project_name.set(project.name);
        self.canvas.components.set(project.layout);
        self.canvas.selected.set(None);
        self.canvas.history.update(|h| h.clear());
        self.settings.set(project.settings);
        self.update_last_modified();
    }

    fn update_last_modified(&self) {
        self.last_modified.set(js_sys::Date::now());
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
