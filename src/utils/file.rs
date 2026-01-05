use crate::domain::{AppError, AppResult};
use wasm_bindgen::JsCast;

/// Download a string content as a file
pub fn download_file(content: &str, filename: &str, mime_type: &str) -> AppResult<()> {
    let array = js_sys::Array::new();
    array.push(&content.into());

    let blob_options = web_sys::BlobPropertyBag::new();
    blob_options.set_type(mime_type);

    let blob =
        web_sys::Blob::new_with_str_sequence_and_options(&array, &blob_options).map_err(|e| {
            AppError::Export(
                e.as_string()
                    .unwrap_or_else(|| "Unknown Blob error".to_string()),
            )
        })?;

    let url = web_sys::Url::create_object_url_with_blob(&blob).map_err(|e| {
        AppError::Export(
            e.as_string()
                .unwrap_or_else(|| "Unknown URL error".to_string()),
        )
    })?;

if let Some(window) = web_sys::window() && let Some(document) = window.document() && let Ok(a) = document.create_element("a") {
        let _ = a.set_attribute("href", &url);
        let _ = a.set_attribute("download", filename);

        // Required for Firefox
        // Use set_attribute for style since set_style_property is on HtmlElement style object
        let _ = a.set_attribute("style", "display: none");

        if let Some(body) = document.body() {
            let _ = body.append_child(&a);
        }

        if let Some(html_element) = a.dyn_ref::<web_sys::HtmlElement>() {
            html_element.click();
        }

        if let Some(body) = document.body() {
           let _ = body.remove_child(&a);
        }

        // Revoke URL to free memory
        let _ = web_sys::Url::revoke_object_url(&url);

        return Ok(());
    }

    Err(AppError::Export(
        "Failed to access window or document".to_string(),
    ))
}

/// Reads the content of a File object as text
pub async fn read_file_as_text(file: &web_sys::File) -> AppResult<String> {
    let promise = file.text();
    let result = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| {
            AppError::Export(
                e.as_string()
                    .unwrap_or_else(|| "File read error".to_string()),
            )
        })?;

    result
        .as_string()
        .ok_or_else(|| AppError::Export("File content was not text".to_string()))
}
