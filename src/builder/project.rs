// ...existing code...
use crate::builder::canvas::CanvasComponent;
use crate::builder::component_library::LibraryComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub layout: Vec<CanvasComponent>,
    pub component_library: Vec<LibraryComponent>,
}

impl ProjectFile {
    pub fn new(layout: Vec<CanvasComponent>, component_library: Vec<LibraryComponent>) -> Self {
        Self {
            layout,
            component_library,
        }
    }
}
