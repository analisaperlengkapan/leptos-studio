
use leptos::*;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <h3>Komponen</h3>
            <ul>
                <li draggable=true on:dragstart=move |e: leptos::ev::DragEvent| {
                    let drag_ev = e.clone().unchecked_into::<web_sys::DragEvent>();
                    drag_ev.data_transfer().unwrap().set_data("component", "Button").unwrap();
                }>Button</li>
                <li draggable=true on:dragstart=move |e: leptos::ev::DragEvent| {
                    let drag_ev = e.clone().unchecked_into::<web_sys::DragEvent>();
                    drag_ev.data_transfer().unwrap().set_data("component", "Text").unwrap();
                }>Text</li>
                <li draggable=true on:dragstart=move |e: leptos::ev::DragEvent| {
                    let drag_ev = e.clone().unchecked_into::<web_sys::DragEvent>();
                    drag_ev.data_transfer().unwrap().set_data("component", "Input").unwrap();
                }>Input</li>
                <li draggable=true on:dragstart=move |e: leptos::ev::DragEvent| {
                    let drag_ev = e.clone().unchecked_into::<web_sys::DragEvent>();
                    drag_ev.data_transfer().unwrap().set_data("component", "Container").unwrap();
                }>Container</li>
            </ul>
        </aside>
    }
}
