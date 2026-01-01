use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::{AppError, AppResult};
use crate::state::AppState;
use crate::state::persistence::Persistable;
use crate::state::project::Project;

use super::git_service::GitBackend;

/// Represents a single commit in our LocalStorage Git backend
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalCommit {
    pub id: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub project_snapshot: Project,
}

/// A container for the repository state that we persist
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RepositoryState {
    pub commits: Vec<LocalCommit>,
    pub head: Option<String>, // Commit ID
}

impl Persistable for RepositoryState {
    fn storage_key() -> &'static str {
        "leptos_studio_git_repo"
    }
}

/// A Git backend that stores commits in browser LocalStorage
pub struct LocalStorageGitBackend;

impl Default for LocalStorageGitBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalStorageGitBackend {
    pub fn new() -> Self {
        Self
    }

    fn get_repo() -> AppResult<RepositoryState> {
        // We reuse the load_or_default logic from Persistable trait,
        // but we need to handle the Result explicitly here.
        // RepositoryState doesn't implement 'load_or_default' directly as a method that returns result,
        // it implements 'load' which returns AppResult.
        match RepositoryState::load() {
            Ok(repo) => Ok(repo),
            Err(_) => Ok(RepositoryState::default()),
        }
    }

    fn save_repo(repo: &RepositoryState) -> AppResult<()> {
        repo.save()
    }
}

impl GitBackend for LocalStorageGitBackend {
    fn status(&self) -> AppResult<String> {
        let repo = Self::get_repo()?;
        let commit_count = repo.commits.len();

        let last_commit_msg = repo.commits.last()
            .map(|c| format!("Last commit: '{}' ({})", c.message, c.id.chars().take(7).collect::<String>()))
            .unwrap_or_else(|| "No commits yet".to_string());

        Ok(format!(
            "On branch main\nLocal storage repository active.\nCommits: {}\n{}",
            commit_count, last_commit_msg
        ))
    }

    fn log(&self) -> AppResult<String> {
        let repo = Self::get_repo()?;
        if repo.commits.is_empty() {
            return Ok("No commits yet.".to_string());
        }

        let mut log_output = String::new();
        // Show newest first
        for commit in repo.commits.iter().rev() {
            log_output.push_str(&format!(
                "commit {}\nDate:   {}\n\n    {}\n\n",
                commit.id,
                commit.timestamp.to_rfc2822(),
                commit.message
            ));
        }
        Ok(log_output)
    }

    fn commit(&self, message: &str) -> AppResult<()> {
        if message.trim().is_empty() {
            // AppError::Validation expects a ValidationError variant, but we don't have a
            // "Generic" validation error there. Let's use Git error for now or add a proper one.
            // Looking at domain/error.rs, ValidationError::InvalidName is close but not exact.
            // Let's use Git error since it's a Git operation.
            return Err(AppError::Git("Commit message cannot be empty".to_string()));
        }

        // Get current app state to snapshot the project
        // Note: This relies on being called inside a reactive context where AppState is available
        let app_state = expect_context::<AppState>();
        let project = app_state.to_project();

        let mut repo = Self::get_repo()?;

        let commit_id = uuid::Uuid::new_v4().to_string();
        let commit = LocalCommit {
            id: commit_id.clone(),
            message: message.to_string(),
            timestamp: Utc::now(),
            project_snapshot: project,
        };

        repo.commits.push(commit);
        repo.head = Some(commit_id);

        Self::save_repo(&repo)?;

        Ok(())
    }

    fn push(&self) -> AppResult<()> {
        // In a local storage backend, "push" could mean exporting to a file,
        // or it could just be a no-op since we are already "saved".
        // For now, let's treat it as a sync confirmation.
        Ok(())
    }
}
