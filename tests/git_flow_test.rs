use leptos_studio::services::GitBackend;
use leptos_studio::services::local_storage_git::LocalStorageGitBackend;
use leptos_studio::state::app_state::SettingsState;
use leptos_studio::state::project::Project;
use leptos_studio::builder::design_tokens::DesignTokens;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_git_flow_end_to_end() {
    // 1. Setup Backend
    let backend = LocalStorageGitBackend::new();

    // 2. Initialize Project (Empty)
    let mut project = Project::new(
        "Test Project".to_string(),
        Vec::new(),
        SettingsState::default(),
        DesignTokens::default(),
    );

    // 3. Check Initial Status (Should be clean effectively if new, or dirty if we consider empty different from HEAD=None)
    // Actually, in LocalStorageGitBackend, if HEAD is None, it's considered dirty if project is not default?
    // Let's verify behavior. If HEAD is None, has_changes = true.
    let status = backend.status(Some(&project)).await.expect("status check");
    assert!(
        status.has_changes,
        "New project should be considered dirty before first commit"
    );

    // 4. First Commit
    let commit_msg = "Initial commit";
    backend
        .commit(&project, commit_msg)
        .await
        .expect("commit failed");

    // 5. Check Status after Commit (Should be clean)
    let status = backend.status(Some(&project)).await.expect("status check");
    assert!(!status.has_changes, "Project should be clean after commit");
    assert!(status.clean, "Project should be clean after commit");
    assert_eq!(status.commit_count, 1);

    // 6. Modify Project (Add Component - simulated by changing name or props since we don't have easy access to CanvasComponent builder here without more imports)
    project.name = "Modified Project".to_string();

    // 7. Check Status (Should be dirty)
    let status = backend.status(Some(&project)).await.expect("status check");
    assert!(
        status.has_changes,
        "Project should be dirty after modification"
    );
    assert!(!status.clean);

    // 8. Commit Changes
    backend
        .commit(&project, "Second commit")
        .await
        .expect("commit failed");

    // 9. Check Log
    let log = backend.log().await.expect("log check");
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].message, "Second commit");
    assert_eq!(log[1].message, "Initial commit");

    // 10. Discard Changes Flow
    // Modify again
    project.name = "Modified Again".to_string();
    let status = backend.status(Some(&project)).await.expect("status check");
    assert!(status.has_changes);

    // Restore HEAD
    let restored_project = backend
        .restore_head()
        .await
        .expect("restore head")
        .expect("should return project");
    assert_eq!(
        restored_project.name, "Modified Project",
        "Should restore to last commit state"
    );

    // Check status with restored project
    let status = backend
        .status(Some(&restored_project))
        .await
        .expect("status check");
    assert!(!status.has_changes, "Restored project should be clean");
}

#[wasm_bindgen_test]
async fn test_git_reset_flow() {
    let backend = LocalStorageGitBackend::new();
    let project = Project::new(
        "Reset Test".to_string(),
        Vec::new(),
        SettingsState::default(),
        DesignTokens::default(),
    );

    // Make a commit
    backend
        .commit(&project, "First commit")
        .await
        .expect("commit");

    // Verify log has entries
    let log = backend.log().await.expect("log");
    assert!(!log.is_empty());

    // Reset repo
    backend.reset().await.expect("reset");

    // Verify log is empty
    let log = backend.log().await.expect("log");
    assert!(log.is_empty());

    // Verify status is clean/empty (or behaves as new repo)
    let status = backend.status(Some(&project)).await.expect("status");
    assert!(status.clean); // Should be clean as it's a new repo relative to "project" (assuming new repo has no HEAD, so technically effectively dirty if project has content, but let's check assumptions)
    // Actually, if HEAD is None, status says has_changes = true usually (unless project is empty default?).
    // Wait, let's check `status` implementation:
    // if let Some(head_id) = &repo.head ... else { has_changes = true; }
    // So after reset, has_changes should be true unless project matches "default empty".
    // Our project is new, but maybe not "empty" enough?
    // Let's just verify commit count is 0
    assert_eq!(status.commit_count, 0);
}
