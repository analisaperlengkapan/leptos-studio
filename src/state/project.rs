use serde::{Deserialize, Serialize};

use crate::domain::CanvasComponent;
use super::app_state::SettingsState;

/// Persistable project representation combining layout and settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub layout: Vec<CanvasComponent>,
    pub settings: SettingsState,
}

impl Project {
    /// Create a new project with the given name, layout, and settings
    pub fn new(name: String, layout: Vec<CanvasComponent>, settings: SettingsState) -> Self {
        Self {
            name,
            description: None,
            layout,
            settings,
        }
    }
}
