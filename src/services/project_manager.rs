use crate::domain::{AppError, AppResult};
use crate::state::Project;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::window;

const PROJECTS_INDEX_KEY: &str = "leptos_studio_projects_index";
const PROJECT_PREFIX: &str = "leptos_studio_project_";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectMetadata {
    pub id: String,
    pub name: String,
    pub last_modified: f64, // JS Date timestamp
    pub component_count: usize,
}

pub struct ProjectManager;

impl ProjectManager {
    /// List all projects
    pub fn list_projects() -> AppResult<Vec<ProjectMetadata>> {
        let storage = window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .ok_or_else(|| AppError::Storage("LocalStorage not available".to_string()))?;

        if let Ok(Some(json)) = storage.get_item(PROJECTS_INDEX_KEY) {
            serde_json::from_str(&json)
                .map_err(|e| AppError::Storage(format!("Failed to parse project index: {}", e)))
        } else {
            Ok(Vec::new())
        }
    }

    /// Save a project
    pub fn save_project(id: &str, project: &Project) -> AppResult<()> {
        let storage = window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .ok_or_else(|| AppError::Storage("LocalStorage not available".to_string()))?;

        // 1. Save the project data
        let key = format!("{}{}", PROJECT_PREFIX, id);
        let json = serde_json::to_string(project)
            .map_err(|e| AppError::Storage(format!("Failed to serialize project: {}", e)))?;

        storage.set_item(&key, &json)
            .map_err(|_| AppError::Storage("Failed to write to LocalStorage".to_string()))?;

        // 2. Update the index
        let mut projects = Self::list_projects()?;
        let now = js_sys::Date::now();

        let metadata = ProjectMetadata {
            id: id.to_string(),
            name: project.name.clone(),
            last_modified: now,
            component_count: project.layout.len(), // Only top-level count, but sufficient for overview
        };

        if let Some(idx) = projects.iter().position(|p| p.id == id) {
            projects[idx] = metadata;
        } else {
            projects.push(metadata);
        }

        // Sort by last modified (descending)
        projects.sort_by(|a, b| b.last_modified.partial_cmp(&a.last_modified).unwrap_or(std::cmp::Ordering::Equal));

        let index_json = serde_json::to_string(&projects)
            .map_err(|e| AppError::Storage(format!("Failed to serialize index: {}", e)))?;

        storage.set_item(PROJECTS_INDEX_KEY, &index_json)
            .map_err(|_| AppError::Storage("Failed to write index".to_string()))?;

        Ok(())
    }

    /// Load a project
    pub fn load_project(id: &str) -> AppResult<Project> {
        let storage = window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .ok_or_else(|| AppError::Storage("LocalStorage not available".to_string()))?;

        let key = format!("{}{}", PROJECT_PREFIX, id);
        if let Ok(Some(json)) = storage.get_item(&key) {
            serde_json::from_str(&json)
                .map_err(|e| AppError::Storage(format!("Failed to parse project: {}", e)))
        } else {
            Err(AppError::Storage("Project not found".to_string()))
        }
    }

    /// Delete a project
    pub fn delete_project(id: &str) -> AppResult<()> {
        let storage = window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .ok_or_else(|| AppError::Storage("LocalStorage not available".to_string()))?;

        // 1. Remove data
        let key = format!("{}{}", PROJECT_PREFIX, id);
        storage.remove_item(&key)
            .map_err(|_| AppError::Storage("Failed to remove project data".to_string()))?;

        // 2. Update index
        let mut projects = Self::list_projects()?;
        projects.retain(|p| p.id != id);

        let index_json = serde_json::to_string(&projects)
            .map_err(|e| AppError::Storage(format!("Failed to serialize index: {}", e)))?;

        storage.set_item(PROJECTS_INDEX_KEY, &index_json)
            .map_err(|_| AppError::Storage("Failed to write index".to_string()))?;

        Ok(())
    }

    /// Create a new project ID
    pub fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
