
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use crate::domain::{AppResult, AppError};

/// Download a string content as a file
pub fn download_file(content: &str, filename: &str, mime_type: &str) -> AppResult<()> {
    let array = js_sys::Array::new();
    array.push(&content.into());

    let blob_options = web_sys::BlobPropertyBag::new();
    blob_options.set_type(mime_type);

    let blob = web_sys::Blob::new_with_str_sequence_and_options(&array, &blob_options)
        .map_err(|e| AppError::Export(e.as_string().unwrap_or_else(|| "Unknown Blob error".to_string())))?;

    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|e| AppError::Export(e.as_string().unwrap_or_else(|| "Unknown URL error".to_string())))?;

    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
             if let Ok(a) = document.create_element("a") {
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
        }
    }

    Err(AppError::Export("Failed to access window or document".to_string()))
}

/// Reads the content of a File object as text
pub async fn read_file_as_text(file: &web_sys::File) -> AppResult<String> {
    // Better approach: wrap in Promise so we can use JsFuture
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let reader = match web_sys::FileReader::new() {
            Ok(r) => r,
            Err(_) => {
                let _ = reject.call1(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str("Failed to create FileReader"));
                return;
            }
        };

        let reader_clone = reader.clone();

        let onload = Closure::wrap(Box::new(move || {
            let result = reader_clone.result().expect("result");
            let _ = resolve.call1(&wasm_bindgen::JsValue::NULL, &result);
        }) as Box<dyn FnMut()>);

        let reject_clone = reject.clone();
        let onerror = Closure::wrap(Box::new(move || {
             let _ = reject_clone.call1(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str("Failed to read file"));
        }) as Box<dyn FnMut()>);

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        // We need to keep closures alive until callback
        // This is tricky with Promise constructor since we can't easily attach them to the reader lifetime without managing them.
        // But since we are inside the Promise executor, they will be dropped when this function ends, which is WRONG.
        // They need to live as long as the read.

        // Standard "leak" pattern for one-off event listeners, or attaching to the object.
        // But we can't attach arbitrary props to FileReader in Rust easily.

        onload.forget();
        onerror.forget();

        if let Err(_) = reader.read_as_text(file) {
             let _ = reject.call1(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str("Failed to start read"));
        }
    });

    let result = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| AppError::Export(e.as_string().unwrap_or_else(|| "File read error".to_string())))?;

    result.as_string().ok_or_else(|| AppError::Export("File content was not text".to_string()))
}
