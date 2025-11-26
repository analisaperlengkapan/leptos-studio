use crate::domain::{AppError, AppResult};
use crate::state::Project;

/// Serialize a Project to a JSON string
pub fn project_to_json(project: &Project) -> AppResult<String> {
    serde_json::to_string_pretty(project)
        .map_err(|e| AppError::Export(format!("Failed to serialize project: {}", e)))
}

/// Deserialize a Project from a JSON string
pub fn project_from_json(json: &str) -> AppResult<Project> {
    serde_json::from_str::<Project>(json)
        .map_err(|e| AppError::Export(format!("Failed to deserialize project: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ButtonComponent, CanvasComponent};
    use crate::state::SettingsState;

    #[test]
    fn project_roundtrip_json() {
        let layout = vec![CanvasComponent::Button(ButtonComponent::new(
            "Test".to_string(),
        ))];
        let settings = SettingsState::default();
        let project = Project::new("My Project".to_string(), layout, settings);

        let json = project_to_json(&project).expect("serialize project");
        let restored = project_from_json(&json).expect("deserialize project");

        assert_eq!(restored.name, "My Project");
        assert_eq!(restored.layout.len(), 1);
    }
}
