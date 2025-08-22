use leptos::*;


use super::builder::sidebar::Sidebar;
use super::builder::canvas::{Canvas, SelectedComponent, CanvasComponent};
use super::builder::property_editor::PropertyEditor;

#[component]
pub fn App() -> impl IntoView {
    let selected = create_rw_signal(SelectedComponent { idx: None });
    let components = create_rw_signal(Vec::<CanvasComponent>::new());
    // Save/load handlers
    let save_layout = move |_| {
        if let Ok(json) = serde_json::to_string(&components.get()) {
            web_sys::window().unwrap().local_storage().unwrap().unwrap().set_item("leptos_studio_layout", &json).unwrap();
        }
    };
    let load_layout = move |_| {
        if let Ok(Some(json)) = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("leptos_studio_layout") {
            if let Ok(data) = serde_json::from_str(&json) {
                components.set(data);
            }
        }
    };
    view! {
        <div class="leptos-studio">
            <Sidebar />
            <div style="display: flex; flex-direction: column; flex: 1;">
                <div style="padding: 0.5rem; background: #f0f0f0; border-bottom: 1px solid #ddd;">
                    <button on:click=save_layout>Save Layout</button>
                    <button on:click=load_layout style="margin-left: 0.5rem;">Load Layout</button>
                </div>
                <Canvas selected=selected components=components />
            </div>
            <PropertyEditor selected=selected components=components />
        </div>
    }
}
