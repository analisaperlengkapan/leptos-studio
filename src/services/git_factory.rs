use crate::services::git_service::GitBackend;
use crate::services::local_storage_git::LocalStorageGitBackend;

/// Factory function to get the configured Git backend.
/// Currently returns LocalStorageGitBackend, but this abstracts the dependency.
pub fn get_git_backend() -> impl GitBackend {
    LocalStorageGitBackend::new()
}
