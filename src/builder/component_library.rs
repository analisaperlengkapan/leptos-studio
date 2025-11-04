use serde::{Deserialize, Serialize};

// Re-export types from state module to avoid duplication
pub use crate::state::app_state::{ResponsiveMode, Theme};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropSchema {
    pub name: String,
    pub prop_type: String, // e.g. "string", "number", "bool"
    pub required: bool,
    pub description: Option<String>,
}

// Shared definition for LibraryComponent used in component library management

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LibraryComponent {
    pub name: String,
    pub kind: String, // e.g. "Button", "Text", "Input", "Container", "Custom"
    pub template: Option<String>, // for custom
    pub category: String, // e.g. "Basic", "Custom"
    pub props_schema: Option<Vec<PropSchema>>, // daftar props dan validasi
    pub description: Option<String>,
}
