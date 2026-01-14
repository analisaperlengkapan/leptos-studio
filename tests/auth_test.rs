use leptos_studio::services::AuthService;

#[test]
fn test_auth_initial_state() {
    let auth = AuthService::new();
    assert_eq!(auth.is_authenticated(), false);
    assert_eq!(auth.get_user(), None);
}

#[test]
fn test_login_success() {
    let auth = AuthService::new();
    let success = auth.login("testuser", "password");
    assert!(success);
    assert!(auth.is_authenticated());

    let user = auth.get_user().unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "testuser@example.com");
}

#[test]
fn test_login_empty_username() {
    let auth = AuthService::new();
    let success = auth.login("", "password");
    assert!(!success);
    assert!(!auth.is_authenticated());
}

#[test]
fn test_logout() {
    let auth = AuthService::new();
    auth.login("testuser", "password");
    assert!(auth.is_authenticated());

    auth.logout();
    assert!(!auth.is_authenticated());
    assert_eq!(auth.get_user(), None);
}
