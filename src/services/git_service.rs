use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::AppResult;
use crate::state::project::Project;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepoStatus {
    pub branch: String,
    pub commit_count: usize,
    pub clean: bool,
    pub active: bool,
    pub has_changes: bool, // Explicit flag for UI showing unsaved changes
}

impl Default for RepoStatus {
    fn default() -> Self {
        Self {
            branch: "main".to_string(),
            commit_count: 0,
            clean: true,
            active: false,
            has_changes: false,
        }
    }
}

/// Abstraction over Git operations so that different backends (web server,
/// Tauri desktop, etc.) can implement Git integration without coupling the UI
/// to a specific environment.
#[allow(async_fn_in_trait)]
pub trait GitBackend {
    async fn status(&self, current_project: Option<&Project>) -> AppResult<RepoStatus>;
    async fn log(&self) -> AppResult<Vec<CommitInfo>>;
    async fn commit(&self, project: &Project, message: &str) -> AppResult<()>;
    async fn push(&self) -> AppResult<Option<String>>;
    async fn clone_repo(&self, json: &str) -> AppResult<()>;
    async fn restore_head(&self) -> AppResult<Option<Project>>;
    async fn reset(&self) -> AppResult<()>;
}

/// No-op Git backend used in pure browser mode where no real Git integration
/// is available. It returns helpful informational messages instead of errors.
pub struct NoopGitBackend;

impl GitBackend for NoopGitBackend {
    async fn status(&self, _current_project: Option<&Project>) -> AppResult<RepoStatus> {
        Ok(RepoStatus {
            branch: "main".to_string(),
            commit_count: 0,
            clean: true,
            active: false,
            has_changes: false,
        })
    }

    async fn log(&self) -> AppResult<Vec<CommitInfo>> {
        Ok(Vec::new())
    }

    async fn commit(&self, _project: &Project, _message: &str) -> AppResult<()> {
        Ok(())
    }

    async fn push(&self) -> AppResult<Option<String>> {
        Ok(None)
    }

    async fn clone_repo(&self, _json: &str) -> AppResult<()> {
        Ok(())
    }

    async fn restore_head(&self) -> AppResult<Option<Project>> {
        Ok(None)
    }

    async fn reset(&self) -> AppResult<()> {
        Ok(())
    }
}
