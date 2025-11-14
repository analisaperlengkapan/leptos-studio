use crate::domain::AppResult;

/// Abstraction over Git operations so that different backends (web server,
/// Tauri desktop, etc.) can implement Git integration without coupling the UI
/// to a specific environment.
pub trait GitBackend {
    fn status(&self) -> AppResult<String>;
    fn log(&self) -> AppResult<String>;
    fn commit(&self, message: &str) -> AppResult<()>;
    fn push(&self) -> AppResult<()>;
}

/// No-op Git backend used in pure browser mode where no real Git integration
/// is available. It returns helpful informational messages instead of errors.
pub struct NoopGitBackend;

impl GitBackend for NoopGitBackend {
    fn status(&self) -> AppResult<String> {
        Ok("Git backend not configured (running in browser-only mode).".to_string())
    }

    fn log(&self) -> AppResult<String> {
        Ok("Git log not available: backend not configured.".to_string())
    }

    fn commit(&self, _message: &str) -> AppResult<()> {
        Ok(())
    }

    fn push(&self) -> AppResult<()> {
        Ok(())
    }
}
