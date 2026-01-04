use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{AppError, AppResult};
use crate::state::persistence::Persistable;
use crate::state::project::Project;

use super::git_service::{GitBackend, CommitInfo, RepoStatus};

/// Represents a single commit in our LocalStorageGit backend
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

    // Simulate network delay to mimic real backend behavior
    async fn simulate_delay(&self) {
        gloo_timers::future::TimeoutFuture::new(300).await;
    }
}

impl GitBackend for LocalStorageGitBackend {
    #[allow(clippy::collapsible_if)]
    async fn status(&self, current_project: Option<&Project>) -> AppResult<RepoStatus> {
        self.simulate_delay().await;

        // Propagate get_repo error instead of swallowing it if it fails catastrophically,
        // but for now, get_repo returns a default on error. We should ideally distinguish
        // between "no repo" and "error loading repo", but the current implementation of get_repo
        // masks the error.
        let repo = Self::get_repo()?;
        let commit_count = repo.commits.len();

        let mut has_changes = false;

        // Compare current app state with HEAD to determine if we have changes
        if let Some(current_project) = current_project {
             // Find HEAD commit
             if let Some(head_id) = &repo.head {
                 if let Some(head_commit) = repo.commits.iter().find(|c| &c.id == head_id) {
                     // Check if project state differs (using PartialEq)
                     if *current_project != head_commit.project_snapshot {
                         has_changes = true;
                     }
                 } else {
                     // Head ID exists but commit not found? Should be an error or dirty state.
                     // Treating as dirty.
                     has_changes = true;
                 }
             } else {
                // No commits yet, but if we have content, is it "changed"?
                // Let's say yes if it's not the default empty project, but keeping it simple:
                // If no commits, we are "dirty" effectively until first commit
                has_changes = true;
             }
        }

        Ok(RepoStatus {
            branch: "main".to_string(),
            commit_count,
            clean: !has_changes,
            active: true,
            has_changes,
        })
    }

    async fn log(&self) -> AppResult<Vec<CommitInfo>> {
        self.simulate_delay().await;

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

    async fn commit(&self, project: &Project, message: &str) -> AppResult<()> {
        self.simulate_delay().await;

        let trimmed_msg = message.trim();
        if trimmed_msg.is_empty() {
            return Err(AppError::Validation(
                crate::domain::error::ValidationError::Generic(
                    "Commit message cannot be empty".to_string(),
                ),
            ));
        }

        // Check for changes before committing (Standards/Best Practice)
        // We can reuse our own status logic or duplicate the check here.
        // Reusing status implies another delay simulation, so we duplicate the check logic for efficiency.
        let mut repo = Self::get_repo()?;

        let mut has_changes = false;
        if let Some(head_id) = &repo.head {
             if let Some(head_commit) = repo.commits.iter().find(|c| &c.id == head_id) {
                 if *project != head_commit.project_snapshot {
                     has_changes = true;
                 }
             } else {
                 has_changes = true;
             }
        } else {
            has_changes = true;
        }

        if !has_changes {
             return Err(AppError::Validation(
                crate::domain::error::ValidationError::Generic(
                    "Nothing to commit (working directory clean)".to_string(),
                ),
            ));
        }

        let commit_id = uuid::Uuid::new_v4().to_string();
        let commit = LocalCommit {
            id: commit_id.clone(),
            message: message.to_string(),
            timestamp: Utc::now(),
            project_snapshot: project.clone(),
        };

        repo.commits.push(commit);
        repo.head = Some(commit_id);

        Self::save_repo(&repo)?;

        Ok(())
    }

    async fn push(&self) -> AppResult<Option<String>> {
        self.simulate_delay().await;

        // Return the whole repo state as JSON for download
        let repo = Self::get_repo()?;
        let json = serde_json::to_string_pretty(&repo)
            .map_err(|e| AppError::Export(format!("Failed to serialize repo: {}", e)))?;
        Ok(Some(json))
    }

    async fn clone_repo(&self, json: &str) -> AppResult<()> {
        self.simulate_delay().await;

        let repo: RepositoryState = serde_json::from_str(json)
            .map_err(|e| AppError::Export(format!("Failed to deserialize repo: {}", e)))?;

        Self::save_repo(&repo)?;
        Ok(())
    }

    async fn restore_head(&self) -> AppResult<Option<Project>> {
        self.simulate_delay().await;

        let repo = Self::get_repo()?;
        let project = repo
            .head
            .as_ref()
            .and_then(|head_id| repo.commits.iter().find(|c| &c.id == head_id))
            .map(|commit| commit.project_snapshot.clone());

        Ok(project)
    }

    async fn reset(&self) -> AppResult<()> {
        self.simulate_delay().await;

        // Reset to default state (empty repo)
        let repo = RepositoryState::default();
        Self::save_repo(&repo)?;

        Ok(())
    }
}

#[cfg(test)]
#[path = "local_storage_git_test.rs"]
mod tests;
