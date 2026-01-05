pub mod app;
pub mod builder;
pub mod constants;
pub mod domain;
pub mod services;
pub mod state;
pub mod utils;

#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
#[cfg(not(test))]
pub fn main() {
    console_error_panic_hook::set_once();

    // Get the #leptos element to mount to
    let document = web_sys::window()
        .expect("Failed to get window")
        .document()
        .expect("Failed to get document");

    let target = document
        .get_element_by_id("leptos")
        .expect("Failed to find #leptos element")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("Failed to cast to HtmlElement");

    // Clear the loading spinner content before mounting
    target.set_inner_html("");

    // Reset the #leptos styling (remove loading state flexbox centering)
    let style = target.style();
    style.set_property("display", "block").ok();
    style.set_property("align-items", "").ok();
    style.set_property("justify-content", "").ok();
    style.set_property("min-height", "").ok();
    style.set_property("background", "").ok();

    // Mount to #leptos div to replace the loading spinner
    // Keep the view mounted permanently with .forget()
    leptos::mount::mount_to(target, app::App).forget();
}
