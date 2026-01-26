use serde::{Deserialize, Serialize};
use web_sys::Storage;

use crate::domain::{AppError, AppResult};

/// Trait for types that can be persisted to LocalStorage
pub trait Persistable: Serialize + for<'de> Deserialize<'de> {
    fn storage_key() -> &'static str;

    /// Save to LocalStorage
    fn save(&self) -> AppResult<()> {
        let storage = get_local_storage()?;
        let json =
            serde_json::to_string(self).map_err(|e| AppError::Serialization(e.to_string()))?;
        storage
            .set_item(Self::storage_key(), &json)
            .map_err(|_| AppError::Storage("Failed to save to LocalStorage".to_string()))?;
        Ok(())
    }

    /// Load from LocalStorage
    fn load() -> AppResult<Self> {
        let storage = get_local_storage()?;
        let json = storage
            .get_item(Self::storage_key())
            .map_err(|_| AppError::Storage("Failed to read from LocalStorage".to_string()))?
            .ok_or_else(|| AppError::Storage("No data found in LocalStorage".to_string()))?;

        serde_json::from_str(&json).map_err(|e| AppError::Serialization(e.to_string()))
    }

    /// Load with default value if not found
    fn load_or_default() -> Self
    where
        Self: Default,
    {
        Self::load().unwrap_or_default()
    }
}

/// Get LocalStorage instance
fn get_local_storage() -> AppResult<Storage> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .ok_or_else(|| AppError::Storage("LocalStorage not available".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)] // Test helper struct
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestData {
        value: String,
    }

    impl Persistable for TestData {
        fn storage_key() -> &'static str {
            "test_data"
        }
    }

    // Note: These tests would need to run in a browser environment
    // For now, they serve as examples of how persistence should work
}
