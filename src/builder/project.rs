// ...existing code...
use serde::{Serialize, Deserialize};
use crate::builder::canvas::CanvasComponent;
use crate::builder::component_library::LibraryComponent;

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub layout: Vec<CanvasComponent>,
    pub component_library: Vec<LibraryComponent>,
}

impl ProjectFile {
    pub fn new(layout: Vec<CanvasComponent>, component_library: Vec<LibraryComponent>) -> Self {
        Self { layout, component_library }
    }
}
