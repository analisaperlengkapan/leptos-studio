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

/// Simple registry helper for working with LibraryComponent collections.
pub struct ComponentRegistry;

impl ComponentRegistry {
    /// Return all components with category "Custom" from a library.
    pub fn custom_from_library(library: &[LibraryComponent]) -> Vec<LibraryComponent> {
        library
            .iter()
            .filter(|c| c.category == "Custom")
            .cloned()
            .collect()
    }

    /// Check whether a library already contains a component with the given name.
    pub fn exists_by_name(library: &[LibraryComponent], name: &str) -> bool {
        library.iter().any(|c| c.name == name)
    }

    /// Add a custom component to both the custom_components list and the
    /// component_library collection.
    pub fn add_custom(
        custom_components: &mut Vec<LibraryComponent>,
        component_library: &mut Vec<LibraryComponent>,
        component: LibraryComponent,
    ) {
        custom_components.push(component.clone());
        component_library.push(component);
    }

    /// Delete a custom component by index from custom_components and remove the
    /// corresponding entry from component_library by name.
    pub fn delete_custom_by_index(
        custom_components: &mut Vec<LibraryComponent>,
        component_library: &mut Vec<LibraryComponent>,
        idx: usize,
    ) {
        if idx >= custom_components.len() {
            return;
        }

        let name = custom_components[idx].name.clone();
        custom_components.remove(idx);

        if let Some(pos) = component_library.iter().position(|c| c.name == name) {
            component_library.remove(pos);
        }
    }

    /// Update the name and template of a custom component at the given index in
    /// custom_components and in component_library (matched by old name).
    pub fn update_custom_by_index(
        custom_components: &mut Vec<LibraryComponent>,
        component_library: &mut Vec<LibraryComponent>,
        idx: usize,
        new_name: String,
        new_template: String,
    ) {
        if idx >= custom_components.len() {
            return;
        }

        let old_name = custom_components[idx].name.clone();

        if let Some(item) = custom_components.get_mut(idx) {
            item.name = new_name.clone();
            item.template = Some(new_template.clone());
        }

        if let Some(item) = component_library.iter_mut().find(|c| c.name == old_name) {
            item.name = new_name;
            item.template = Some(new_template);
        }
    }
}
