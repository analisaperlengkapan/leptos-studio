use wasm_bindgen::JsValue;
use web_sys::{window, Clipboard, Navigator};

use crate::domain::AppResult;

/// Copy text to clipboard
pub async fn copy_to_clipboard(text: &str) -> AppResult<()> {
    let window = window().ok_or_else(|| {
        crate::domain::AppError::Storage("Window not available".to_string())
    })?;
    
    let navigator: Navigator = window.navigator();
    let clipboard: Clipboard = navigator.clipboard();
    
    let promise = clipboard.write_text(text);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    
    result.map(|_| ()).map_err(|e| {
        crate::domain::AppError::Storage(format!("Failed to copy to clipboard: {:?}", e))
    })
}

/// Read text from clipboard
pub async fn read_from_clipboard() -> AppResult<String> {
    let window = window().ok_or_else(|| {
        crate::domain::AppError::Storage("Window not available".to_string())
    })?;
    
    let navigator: Navigator = window.navigator();
    let clipboard: Clipboard = navigator.clipboard();
    
    let promise = clipboard.read_text();
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    
    result
        .and_then(|val| val.as_string().ok_or(JsValue::NULL))
        .map_err(|e| {
            crate::domain::AppError::Storage(format!("Failed to read from clipboard: {:?}", e))
        })
}
