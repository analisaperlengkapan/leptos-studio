use crate::services::git_service::GitBackend;
use crate::services::local_storage_git::LocalStorageGitBackend;
use crate::services::remote_git::RemoteGitBackend;

/// Factory function to get the configured Git backend.
pub fn get_git_backend(project_id: Option<String>) -> Box<dyn GitBackend> {
    // If we have a valid project ID (and it's not "default" fallback from previous steps), use Remote.
    // Otherwise use LocalStorage for offline/new projects.

    match project_id {
        Some(id) if !id.is_empty() && id != "default" => Box::new(RemoteGitBackend::new(&id)),
        _ => Box::new(LocalStorageGitBackend::new()),
    }
}
