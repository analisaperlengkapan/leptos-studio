use crate::domain::{AppError, AppResult};
use crate::services::git_service::{CommitInfo, GitBackend, RepoStatus};
use crate::state::project::Project;
use chrono::DateTime;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RemoteGitCommit {
    pub id: String,
    pub message: String,
    pub timestamp: f64,
    pub snapshot: serde_json::Value,
}

pub struct RemoteGitBackend {
    project_id: String,
}

impl RemoteGitBackend {
    pub fn new(project_id: &str) -> Self {
        Self {
            project_id: project_id.to_string(),
        }
    }

    fn get_api_base() -> String {
        let runtime_base = leptos::prelude::window()
            .get("LEPTOS_API_URL")
            .and_then(|val| val.as_string());

        let base = runtime_base
            .or_else(|| option_env!("API_URL").map(|s| s.to_string()))
            .unwrap_or_else(|| "http://localhost:3000".to_string());

        format!("{}/api/projects", base.trim_end_matches('/'))
    }

    async fn get_commits(&self) -> AppResult<Vec<RemoteGitCommit>> {
        let url = format!("{}/{}/commits", Self::get_api_base(), self.project_id);
        let resp = Request::get(&url)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !resp.ok() {
            // If 404, might mean no commits yet, return empty
            if resp.status() == 404 {
                return Ok(Vec::new());
            }
            return Err(AppError::Network(format!(
                "Server returned {}: {}",
                resp.status(),
                resp.status_text()
            )));
        }

        resp.json()
            .await
            .map_err(|e| AppError::Serialization(e.to_string()))
    }
}

#[async_trait::async_trait(?Send)]
impl GitBackend for RemoteGitBackend {
    async fn status(&self, current_project: Option<&Project>) -> AppResult<RepoStatus> {
        // Simple status check: compare current project with HEAD
        let commits = self.get_commits().await?;
        let commit_count = commits.len();

        let has_changes = if let Some(current) = current_project {
            if let Some(head) = commits.last() {
                // Deserialize snapshot to Project to compare
                match serde_json::from_value::<Project>(head.snapshot.clone()) {
                    Ok(head_project) => *current != head_project,
                    Err(_) => true, // Error parsing HEAD means dirty or broken
                }
            } else {
                true // No commits = dirty
            }
        } else {
            false
        };

        Ok(RepoStatus {
            branch: "main".to_string(),
            commit_count,
            clean: !has_changes,
            active: true,
            has_changes,
        })
    }

    async fn log(&self) -> AppResult<Vec<CommitInfo>> {
        let commits = self.get_commits().await?;

        // Convert to CommitInfo
        let mut infos: Vec<CommitInfo> = commits
            .into_iter()
            .map(|c| {
                // Convert f64 timestamp to DateTime<Utc>
                let secs = (c.timestamp / 1000.0) as i64;
                let nsecs = ((c.timestamp % 1000.0) * 1_000_000.0) as u32;
                let dt = DateTime::from_timestamp(secs, nsecs).unwrap_or_default();

                CommitInfo {
                    id: c.id,
                    message: c.message,
                    timestamp: dt,
                }
            })
            .collect();

        // Reverse to show newest first
        infos.reverse();
        Ok(infos)
    }

    async fn commit(&self, project: &Project, message: &str) -> AppResult<()> {
        let url = format!("{}/{}/commits", Self::get_api_base(), self.project_id);

        let payload = serde_json::json!({
            "message": message,
            "timestamp": js_sys::Date::now(),
            "snapshot": project
        });

        let resp = Request::post(&url)
            .json(&payload)
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

    async fn push(&self) -> AppResult<Option<String>> {
        // "Push" in this context (Remote Backend) is redundant as we are already "pushed".
        // But to keep consistent with UI, we can return the JSON of the repo (commits).
        let commits = self.get_commits().await?;
        let json = serde_json::to_string_pretty(&commits)
            .map_err(|e| AppError::Serialization(e.to_string()))?;
        Ok(Some(json))
    }

    async fn clone_repo(&self, _json: &str) -> AppResult<()> {
        // Not implemented for RemoteBackend via this method for now,
        // or we could POST the whole history.
        Ok(())
    }

    async fn restore_head(&self) -> AppResult<Option<Project>> {
        let commits = self.get_commits().await?;
        if let Some(head) = commits.last() {
            let project: Project = serde_json::from_value(head.snapshot.clone())
                .map_err(|e| AppError::Serialization(e.to_string()))?;
            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    async fn reset(&self) -> AppResult<()> {
        let url = format!("{}/{}/commits", Self::get_api_base(), self.project_id);
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
}
