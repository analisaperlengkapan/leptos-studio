#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropSchema {
    pub name: String,
    pub prop_type: String, // e.g. "string", "number", "bool"
    pub required: bool,
    pub description: Option<String>,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResponsiveMode {
    Desktop,
    Tablet,
    Mobile,
}
#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub enum Theme {
    Light,
    Dark,
    Custom,
}
// Shared definition for LibraryComponent used in component library management
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LibraryComponent {
    pub name: String,
    pub kind: String, // e.g. "Button", "Text", "Input", "Container", "Custom"
    pub template: Option<String>, // for custom
    pub category: String, // e.g. "Basic", "Custom"
    pub props_schema: Option<Vec<PropSchema>>, // daftar props dan validasi
    pub description: Option<String>,
}
