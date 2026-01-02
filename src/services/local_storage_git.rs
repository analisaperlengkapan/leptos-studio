use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::{AppError, AppResult};
use crate::state::AppState;
use crate::state::persistence::Persistable;
use crate::state::project::Project;

use super::git_service::{GitBackend, CommitInfo, RepoStatus};

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
    fn status(&self) -> AppResult<RepoStatus> {
        let repo = Self::get_repo()?;
        let commit_count = repo.commits.len();

        // In this simple model, we are always clean because we snapshot on commit
        // But for UI purposes, we could check if current AppState differs from HEAD.
        // For now, let's just return basic stats.

        Ok(RepoStatus {
            branch: "main".to_string(),
            commit_count,
            clean: true, // We don't track dirty state yet
            active: true,
        })
    }

    fn log(&self) -> AppResult<Vec<CommitInfo>> {
        let repo = Self::get_repo()?;
        if repo.commits.is_empty() {
            return Ok(Vec::new());
        }

        // Return commits reversed (newest first)
        let commits = repo.commits.iter().rev().map(|c| CommitInfo {
            id: c.id.clone(),
            message: c.message.clone(),
            timestamp: c.timestamp,
        }).collect();

        Ok(commits)
    }

    fn commit(&self, message: &str) -> AppResult<()> {
        let trimmed_msg = message.trim();
        if trimmed_msg.is_empty() {
            return Err(AppError::Validation(
                crate::domain::error::ValidationError::Generic(
                    "Commit message cannot be empty".to_string(),
                ),
            ));
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

    fn push(&self) -> AppResult<Option<String>> {
        // Return the whole repo state as JSON for download
        let repo = Self::get_repo()?;
        let json = serde_json::to_string_pretty(&repo)
            .map_err(|e| AppError::Export(format!("Failed to serialize repo: {}", e)))?;
        Ok(Some(json))
    }

    fn clone_repo(&self, json: &str) -> AppResult<()> {
        let repo: RepositoryState = serde_json::from_str(json)
            .map_err(|e| AppError::Export(format!("Failed to deserialize repo: {}", e)))?;

        Self::save_repo(&repo)?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "local_storage_git_test.rs"]
mod tests;
