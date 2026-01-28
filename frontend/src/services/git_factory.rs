use crate::services::git_service::GitBackend;
use crate::services::remote_git::RemoteGitBackend;

/// Factory function to get the configured Git backend.
pub fn get_git_backend(project_id: Option<String>) -> impl GitBackend {
    // If we have a project ID, use the remote backend.
    // Otherwise fallback to local storage (or Noop if we preferred).
    // For now, always use Remote if ID is present.
    // Ideally we would support local storage fallback or offline mode,
    // but the requirement is "integration with backend".

    // Note: LocalStorageGitBackend is global (no ID).
    // We could return Box<dyn GitBackend> if we needed dynamic dispatch,
    // but impl Trait requires a single return type branch unless we box.

    // Since we can't easily conditionally return different types without Box,
    // and we want to enforce backend integration, let's use Remote if possible.
    // But wait, `impl GitBackend` requires one type.

    // Simplest approach: Use RemoteGitBackend.
    // If project_id is None (e.g. global context?), we might have issues.
    // But use_git always uses current project.

    let id = project_id.unwrap_or_else(|| "default".to_string());
    RemoteGitBackend::new(&id)
}
