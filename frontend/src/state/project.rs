use serde::{Deserialize, Serialize};

use super::app_state::SettingsState;
use crate::builder::design_tokens::DesignTokens;
use crate::domain::{CanvasComponent, Variable};

/// Persistable project representation combining layout and settings
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub layout: Vec<CanvasComponent>,
    pub settings: SettingsState,
    #[serde(default)]
    pub design_tokens: DesignTokens,
    #[serde(default)]
    pub variables: Vec<Variable>,
}

impl Project {
    /// Create a new project with the given name, layout, and settings
    pub fn new(
        name: String,
        layout: Vec<CanvasComponent>,
        settings: SettingsState,
        design_tokens: DesignTokens,
        variables: Vec<Variable>,
    ) -> Self {
        Self {
            name,
            description: None,
            layout,
            settings,
            design_tokens,
            variables,
        }
    }
}
