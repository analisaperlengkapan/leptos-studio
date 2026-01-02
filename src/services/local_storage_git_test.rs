#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{RepoStatus};
    use crate::state::project::Project;
    use crate::services::local_storage_git::{RepositoryState, LocalCommit};
    use crate::state::app_state::SettingsState;
    use chrono::Utc;

    #[test]
    fn test_repository_state_serialization() {
        let commit_id = "test-id";
        let message = "Test Commit";
        let timestamp = Utc::now();

        let project = Project::new(
            "Test Project".to_string(),
            Vec::new(),
            SettingsState::default(),
        );

        let repo = RepositoryState {
            commits: vec![
                LocalCommit {
                    id: commit_id.to_string(),
                    message: message.to_string(),
                    timestamp,
                    project_snapshot: project.clone(),
                }
            ],
            head: Some(commit_id.to_string()),
        };

        // Test JSON roundtrip (Simulates Push -> Clone)
        let json = serde_json::to_string_pretty(&repo).expect("serialize");
        let restored: RepositoryState = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(restored.commits.len(), 1);
        assert_eq!(restored.commits[0].id, commit_id);
        assert_eq!(restored.commits[0].message, message);
        assert_eq!(restored.head, Some(commit_id.to_string()));
    }

    #[test]
    fn test_repo_status_logic() {
        // This is a weak test but verifies the data structures compile and work.
        let status = RepoStatus {
            branch: "main".to_string(),
            commit_count: 5,
            clean: true,
            active: true,
        };

        assert_eq!(status.branch, "main");
        assert_eq!(status.commit_count, 5);
        assert!(status.clean);
    }
}
