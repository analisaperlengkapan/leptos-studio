use leptos::prelude::*;
use js_sys::encode_uri_component;
use crate::state::app_state::Notification;
use crate::utils::copy_to_clipboard;

#[component]
pub fn ExportModal(
    show: RwSignal<bool>,
    code: RwSignal<String>,
    format: RwSignal<String>,
    on_close: Callback<(), ()>,
    notification_signal: RwSignal<Option<Notification>>,
) -> impl IntoView {

    let copy_handler = move |_| {
        let code_text = code.get();
        let notif_signal = notification_signal;

        wasm_bindgen_futures::spawn_local(async move {
            match copy_to_clipboard(&code_text).await {
                Ok(()) => {
                    notif_signal.set(Some(Notification::success(
                        "📋 Code copied to clipboard!".to_string(),
                    )));
                }
                Err(e) => {
                    notif_signal.set(Some(Notification::error(format!(
                        "❌ {}",
                        e.user_message()
                    ))));
                }
            }
        });
    };

    let download_handler = move |_| {
        let code_text = code.get();
        let mime = match format.get().as_str() {
            "html" => "text/html",
            "markdown" => "text/markdown",
            "json" => "application/json",
            _ => "text/plain",
        };

        let encoded = encode_uri_component(&code_text);
        let url = format!("data:{};charset=utf-8,{}", mime, encoded);

        if let Some(window) = web_sys::window() {
            let _ = window.open_with_url_and_target(&url, "_blank");
        } else {
            notification_signal.set(Some(Notification::error(
                "❌ Unable to open download window".to_string(),
            )));
        }
    };

    view! {
        <Show when=move || show.get()>
            <div
                class="modal-overlay"
                role="dialog"
                aria-modal="true"
                aria-labelledby="export-dialog-title"
            >
                <div class="modal-content">
                    <h3 id="export-dialog-title">{"Export Code"}</h3>
                    <label for="export-format" class="visually-hidden">{"Export format"}</label>
                    <select
                        id="export-format"
                        prop:value=format
                        on:input=move |ev| format.set(event_target_value(&ev))
                        style="margin-bottom:1em;width:100%;padding:0.5rem;"
                    >
                        <optgroup label="Framework Code">
                            <option value="leptos">{"Leptos Component"}</option>
                            <option value="react">{"React/JSX Component"}</option>
                            <option value="svelte">{"Svelte Component"}</option>
                        </optgroup>
                        <optgroup label="Web Output">
                            <option value="html">{"Plain HTML"}</option>
                            <option value="tailwind">{"HTML + Tailwind CSS"}</option>
                        </optgroup>
                        <optgroup label="Data Formats">
                            <option value="json">{"Raw JSON"}</option>
                            <option value="jsonschema">{"JSON Schema"}</option>
                            <option value="typescript">{"TypeScript Types"}</option>
                        </optgroup>
                        <optgroup label="Documentation">
                            <option value="markdown">{"Markdown"}</option>
                        </optgroup>
                    </select>
                    <textarea
                        style="width:100%;height:300px;margin-bottom:0.75rem;font-family:monospace;font-size:0.875rem;"
                        readonly
                        aria-label="Generated code"
                    >
                        {move || code.get()}
                    </textarea>
                    <div style="display:flex;justify-content:flex-end;gap:0.5rem;">
                        <button on:click=copy_handler class="btn btn-secondary">{"📋 Copy"}</button>
                        <button on:click=download_handler class="btn btn-secondary">{"⬇️ Download"}</button>
                        <button on:click=move |_| on_close.run(()) class="btn btn-outline">{"Close"}</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
