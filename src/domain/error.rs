use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Component name is empty")]
    EmptyName,
    
    #[error("Component name '{0}' is invalid: must be a valid Rust identifier")]
    InvalidName(String),
    
    #[error("Component name '{0}' already exists")]
    DuplicateName(String),
    
    #[error("Template is empty")]
    EmptyTemplate,
    
    #[error("Template is invalid: {0}")]
    InvalidTemplate(String),
    
    #[error("Property '{0}' is required but not provided")]
    MissingRequiredProperty(String),
    
    #[error("Property '{0}' has invalid value: {1}")]
    InvalidPropertyValue(String, String),
}

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Git error: {0}")]
    Git(String),
    
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    
    #[error("Export error: {0}")]
    Export(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    /// Convert error to user-friendly message
    pub fn user_message(&self) -> String {
        match self {
            AppError::Validation(e) => format!("Validation failed: {}", e),
            AppError::Serialization(msg) => format!("Failed to process data: {}", msg),
            AppError::Storage(msg) => format!("Storage error: {}", msg),
            AppError::Git(msg) => format!("Git operation failed: {}", msg),
            AppError::ComponentNotFound(name) => format!("Component '{}' not found", name),
            AppError::Export(msg) => format!("Export failed: {}", msg),
        }
    }
}
