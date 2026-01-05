use wasm_bindgen_test::*;
use leptos_studio::builder::accessibility::get_focusable_elements;
use web_sys::{window, HtmlElement};
use wasm_bindgen::JsCast;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_get_focusable_elements() {
    let window = window().expect("should have a window");
    let document = window.document().expect("should have a document");
    let body = document.body().expect("should have a body");

    // Create a container
    let container = document.create_element("div").expect("create div");
    let container: HtmlElement = container.dyn_into().expect("cast to HtmlElement");
    body.append_child(&container).expect("append container");

    // Create focusable elements
    let btn = document.create_element("button").expect("create button");
    btn.set_inner_html("Click me");
    // Ensure it has dimensions (simulated if needed, but browser should handle default size)
    // Note: In some test envs, we might need to style it.
    container.append_child(&btn).expect("append btn");

    let input = document.create_element("input").expect("create input");
    container.append_child(&input).expect("append input");

    let link = document.create_element("a").expect("create link");
    link.set_attribute("href", "#").expect("set href");
    link.set_inner_html("Link");
    container.append_child(&link).expect("append link");

    // Create non-focusable elements
    let div = document.create_element("div").expect("create div");
    container.append_child(&div).expect("append div");

    let disabled_btn = document.create_element("button").expect("create disabled btn");
    disabled_btn.set_attribute("disabled", "").expect("set disabled");
    container.append_child(&disabled_btn).expect("append disabled btn");

    // Create hidden element (focusable type but hidden)
    let hidden_btn = document.create_element("button").expect("create hidden btn");
    hidden_btn.set_inner_html("Hidden");
    hidden_btn.set_attribute("style", "display: none;").expect("set style");
    container.append_child(&hidden_btn).expect("append hidden btn");

    // Test
    let focusable = get_focusable_elements(&container);

    // Cleanup
    body.remove_child(&container).expect("cleanup");

    // Assertions
    // We expect 3 elements: btn, input, link.
    // hidden_btn should be excluded by offset check.
    // disabled_btn should be excluded by selector.
    // div should be excluded by selector.

    // NOTE: If this test runs in an environment without layout (e.g. pure jsdom without layout support),
    // offset_width might be 0 for all elements, causing this to fail (0 found).
    // If that happens, the test environment needs to be fixed or we accept that risk.
    // Given the task is to implement the feature for production, the code correctness is priority.

    assert_eq!(focusable.len(), 3, "Should find 3 focusable elements");
}
