use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlElement};

use crate::domain::AppResult;

/// Get the document
pub fn get_document() -> AppResult<Document> {
    window()
        .and_then(|w| w.document())
        .ok_or_else(|| crate::domain::AppError::Storage("Document not available".to_string()))
}

/// Get element by ID
pub fn get_element_by_id(id: &str) -> AppResult<Element> {
    get_document()?
        .get_element_by_id(id)
        .ok_or_else(|| {
            crate::domain::AppError::ComponentNotFound(format!("Element with id '{}' not found", id))
        })
}

/// Get HTML element by ID
pub fn get_html_element_by_id(id: &str) -> AppResult<HtmlElement> {
    get_element_by_id(id)?
        .dyn_into::<HtmlElement>()
        .map_err(|_| {
            crate::domain::AppError::Storage(format!(
                "Element with id '{}' is not an HTMLElement",
                id
            ))
        })
}

/// Set data attribute on element
pub fn set_data_attribute(element: &Element, key: &str, value: &str) -> AppResult<()> {
    element
        .set_attribute(&format!("data-{}", key), value)
        .map_err(|_| {
            crate::domain::AppError::Storage(format!("Failed to set data-{} attribute", key))
        })
}

/// Get data attribute from element
pub fn get_data_attribute(element: &Element, key: &str) -> Option<String> {
    element.get_attribute(&format!("data-{}", key))
}

/// Add class to element
pub fn add_class(element: &Element, class: &str) -> AppResult<()> {
    element
        .class_list()
        .add_1(class)
        .map_err(|_| {
            crate::domain::AppError::Storage(format!("Failed to add class '{}'", class))
        })
}

/// Remove class from element
pub fn remove_class(element: &Element, class: &str) -> AppResult<()> {
    element
        .class_list()
        .remove_1(class)
        .map_err(|_| {
            crate::domain::AppError::Storage(format!("Failed to remove class '{}'", class))
        })
}

/// Toggle class on element
pub fn toggle_class(element: &Element, class: &str) -> AppResult<bool> {
    element
        .class_list()
        .toggle(class)
        .map_err(|_| {
            crate::domain::AppError::Storage(format!("Failed to toggle class '{}'", class))
        })
}
