
use wasm_bindgen::JsCast;
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

/// Read a file as text using FileReader
pub async fn read_file_as_text(file: web_sys::File) -> AppResult<String> {
    let (tx, rx) = futures::channel::oneshot::channel();
    let tx = std::rc::Rc::new(std::cell::RefCell::new(Some(tx)));

    let reader = web_sys::FileReader::new()
        .map_err(|e| AppError::Export(e.as_string().unwrap_or_else(|| "Unknown FileReader error".to_string())))?;

    let reader_clone = reader.clone();
    let tx_clone = tx.clone();
    let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        let result = reader_clone.result().map(|v| v.as_string().unwrap_or_default());
        if let Some(tx) = tx_clone.borrow_mut().take() {
            let _ = tx.send(result.map_err(|e| AppError::Export(format!("Read error: {:?}", e))));
        }
    }) as Box<dyn FnMut()>);

    let reader_clone2 = reader.clone();
    let tx_clone2 = tx.clone();
    let onerror = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        let err = reader_clone2.error().map(|e| e.message()).unwrap_or_else(|| "Unknown error".to_string());
        if let Some(tx) = tx_clone2.borrow_mut().take() {
            let _ = tx.send(Err(AppError::Export(format!("FileReader error: {}", err))));
        }
    }) as Box<dyn FnMut()>);

    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));

    reader.read_as_text(&file).map_err(|e| AppError::Export(e.as_string().unwrap_or_else(|| "Read start error".to_string())))?;

    // Keep closures alive until the future completes
    let result = rx.await.map_err(|_| AppError::Export("FileReader cancelled".to_string()))??;

    // Explicitly drop closures to clean up memory (optional but good practice)
    // Actually, simply letting them go out of scope here AFTER await is correct because
    // we need them to live until the callback fires.
    // Wait, the closures are passed to JS, so we need to `forget` them or keep them alive.
    // If we `forget` them, they leak. If we don't, they drop immediately.
    // BUT since we are awaiting `rx`, this function frame stays alive... NO, `onload` and `onerror` are dropped at end of scope.
    // We must ensure they live long enough.

    // Better approach: Use `wasm_bindgen_futures::JsFuture` if possible, but FileReader is event based.
    // Standard pattern: forget closures, but that leaks.
    // Or, store them in a struct that we drop after await.

    // Let's use a struct to hold them.
    struct ClosureGuard {
        _onload: wasm_bindgen::closure::Closure<dyn FnMut()>,
        _onerror: wasm_bindgen::closure::Closure<dyn FnMut()>,
    }

    let _guard = ClosureGuard {
        _onload: onload,
        _onerror: onerror,
    };

    Ok(result)
}
