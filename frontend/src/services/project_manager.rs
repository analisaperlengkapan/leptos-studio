use crate::domain::{AppError, AppResult};
use crate::state::Project;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

const API_BASE: &str = "http://localhost:3000/api/projects";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectMetadata {
    pub id: String,
    pub name: String,
    pub last_modified: f64,
    pub component_count: usize,
}

pub struct ProjectManager;

impl ProjectManager {
    /// List all projects
    pub async fn list_projects() -> AppResult<Vec<ProjectMetadata>> {
        let resp = Request::get(API_BASE)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !resp.ok() {
            return Err(AppError::Network(format!("Server returned {}", resp.status())));
        }

        resp.json().await.map_err(|e| AppError::Serialization(e.to_string()))
    }

    /// Save a project
    pub async fn save_project(id: &str, project: &Project) -> AppResult<()> {
        let mut json = serde_json::to_value(project)
             .map_err(|e| AppError::Serialization(e.to_string()))?;

        // Ensure ID is in the payload
        if let Some(obj) = json.as_object_mut() {
            obj.insert("id".to_string(), serde_json::Value::String(id.to_string()));
            obj.insert("last_modified".to_string(), serde_json::Value::from(js_sys::Date::now()));
        }

        let resp = Request::post(API_BASE)
            .json(&json)
            .map_err(|e| AppError::Serialization(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

         if !resp.ok() {
            return Err(AppError::Network(format!("Server returned {}", resp.status())));
        }

        Ok(())
    }

    /// Load a project
    pub async fn load_project(id: &str) -> AppResult<Project> {
        let url = format!("{}/{}", API_BASE, id);
        let resp = Request::get(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !resp.ok() {
            return Err(AppError::Network(format!("Server returned {}", resp.status())));
        }

        resp.json().await.map_err(|e| AppError::Serialization(e.to_string()))
    }

    /// Delete a project
    pub async fn delete_project(id: &str) -> AppResult<()> {
        let url = format!("{}/{}", API_BASE, id);
         let resp = Request::delete(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !resp.ok() {
            return Err(AppError::Network(format!("Server returned {}", resp.status())));
        }

        Ok(())
    }

    /// Rename a project
    pub async fn rename_project(id: &str, new_name: &str) -> AppResult<()> {
        // Fetch, update, save
        let mut project = Self::load_project(id).await?;
        project.name = new_name.to_string();
        Self::save_project(id, &project).await
    }

    /// Create a new project ID
    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}
