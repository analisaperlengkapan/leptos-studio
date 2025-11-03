pub mod app;
pub mod builder;
pub mod components;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(app::App);
}
