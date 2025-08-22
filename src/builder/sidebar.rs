
use leptos::*;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn Sidebar(custom_components: RwSignal<Vec<(String, String)>>) -> impl IntoView {
    // State form custom komponen
    let name = create_rw_signal(String::new());
    let template = create_rw_signal(String::new());

    let add_custom = move |_| {
        let n = name.get().trim().to_string();
        let t = template.get().trim().to_string();
        if !n.is_empty() && !t.is_empty() {
            custom_components.update(|v| v.push((n.clone(), t.clone())));
            name.set(String::new());
            template.set(String::new());
        }
    };

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
                <For
                    each=move || custom_components.get().clone().into_iter().enumerate()
                    key=|(i, _)| *i
                    children=move |(_i, (n, t))| {
                        let n2 = n.clone();
                        view! {
                            <li draggable=true style="color:#7b1fa2;" on:dragstart=move |e: leptos::ev::DragEvent| {
                                let drag_ev = e.clone().unchecked_into::<web_sys::DragEvent>();
                                drag_ev.data_transfer().unwrap().set_data("component", &format!("Custom::{}::{}", n2, t)).unwrap();
                            }>{format!("Custom: {}", n)}</li>
                        }
                    }
                />
            </ul>
            <hr />
            <h4>Tambah Custom Komponen</h4>
            <div>
                <input placeholder="Nama" value=name on:input=move |ev| name.set(event_target_value(&ev)) style="width:90%;margin-bottom:0.5rem;" />
                <textarea placeholder="Template" value=template on:input=move |ev| template.set(event_target_value(&ev)) style="width:90%;height:60px;" />
                <button on:click=add_custom style="margin-top:0.5rem;">Tambah</button>
            </div>
        </aside>
    }
}
