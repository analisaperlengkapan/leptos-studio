use leptos_studio::app::App;
use console_error_panic_hook;
use web_sys::wasm_bindgen::JsCast;

fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"🔥 Leptos Studio main() starting".into());

    let target = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("leptos")
        .unwrap()
        .unchecked_into();
    leptos::mount_to(target, App);

    web_sys::console::log_1(&"🔥 Leptos Studio mounted to #leptos".into());
}
