pub mod app;
pub mod builder;
pub mod components;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(app::App);
}
