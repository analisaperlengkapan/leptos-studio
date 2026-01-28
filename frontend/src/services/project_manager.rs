use crate::domain::{AppError, AppResult};
use crate::state::Project;
use gloo_net::http::Request;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

fn get_api_base() -> String {
    // Try to get from runtime window.LEPTOS_API_URL first
    let runtime_base = window()
        .get("LEPTOS_API_URL")
        .and_then(|val| val.as_string());

    let base = runtime_base
        .or_else(|| option_env!("API_URL").map(|s| s.to_string()))
        .unwrap_or_else(|| "http://localhost:3000".to_string());

    format!("{}/api/projects", base.trim_end_matches('/'))
}

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
        let resp = Request::get(&get_api_base())
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !resp.ok() {
            return Err(AppError::Network(format!(
                "Server returned {}",
                resp.status()
            )));
        }

        resp.json()
            .await
            .map_err(|e| AppError::Serialization(e.to_string()))
    }

    /// Save a project
    pub async fn save_project(id: &str, project: &Project) -> AppResult<()> {
        let mut json =
            serde_json::to_value(project).map_err(|e| AppError::Serialization(e.to_string()))?;

        // Ensure ID is in the payload
        if let Some(obj) = json.as_object_mut() {
            obj.insert("id".to_string(), serde_json::Value::String(id.to_string()));
            obj.insert(
                "last_modified".to_string(),
                serde_json::Value::from(js_sys::Date::now()),
            );
        }

        let resp = Request::post(&get_api_base())
            .json(&json)
            .map_err(|e| AppError::Serialization(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !resp.ok() {
            return Err(AppError::Network(format!(
                "Server returned {}",
                resp.status()
            )));
        }

        Ok(())
    }

    /// Load a project
    pub async fn load_project(id: &str) -> AppResult<Project> {
        let url = format!("{}/{}", get_api_base(), id);
        let resp = Request::get(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !resp.ok() {
            return Err(AppError::Network(format!(
                "Server returned {}",
                resp.status()
            )));
        }

        resp.json()
            .await
            .map_err(|e| AppError::Serialization(e.to_string()))
    }

    /// Delete a project
    pub async fn delete_project(id: &str) -> AppResult<()> {
        let url = format!("{}/{}", get_api_base(), id);
        let resp = Request::delete(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !resp.ok() {
            return Err(AppError::Network(format!(
                "Server returned {}",
                resp.status()
            )));
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
