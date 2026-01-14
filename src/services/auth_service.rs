use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::state::persistence::Persistable;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AuthData {
    pub user: Option<User>,
}

impl Persistable for AuthData {
    fn storage_key() -> &'static str {
        "leptos_studio_auth"
    }
}

#[derive(Clone, Copy)]
pub struct AuthService {
    pub user: RwSignal<Option<User>>,
}

impl AuthService {
    pub fn new() -> Self {
        // Try to load from storage, default to None if failed
        #[cfg(target_arch = "wasm32")]
        let data = AuthData::load_or_default();
        #[cfg(not(target_arch = "wasm32"))]
        let data = AuthData::default();

        Self {
            user: RwSignal::new(data.user),
        }
    }

    pub fn login(&self, username: &str, _password: &str) -> bool {
        if !username.trim().is_empty() {
            let user = User {
                username: username.to_string(),
                email: format!("{}@example.com", username.to_lowercase()),
            };
            self.user.set(Some(user.clone()));

            // Persist
            #[cfg(target_arch = "wasm32")]
            {
                let data = AuthData { user: Some(user) };
                let _ = data.save(); // Ignore error
            }

            return true;
        }
        false
    }

    pub fn logout(&self) {
        self.user.set(None);
        #[cfg(target_arch = "wasm32")]
        {
            let data = AuthData { user: None };
            let _ = data.save();
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.with(|u| u.is_some())
    }

    pub fn get_user(&self) -> Option<User> {
        self.user.get()
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}
