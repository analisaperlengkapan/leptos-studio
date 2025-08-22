use leptos::*;


use super::builder::sidebar::Sidebar;
use super::builder::canvas::{Canvas, SelectedComponent, CanvasComponent};
use super::builder::property_editor::PropertyEditor;
use super::builder::preview::Preview;

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
    // State untuk export modal
    let show_export = create_rw_signal(false);
    let export_code = create_rw_signal(String::new());

    let do_export = move |_| {
        // Placeholder: akan diisi generator kode
        let code = crate::builder::export::generate_leptos_code(&components.get());
        export_code.set(code);
        show_export.set(true);
    };
    let close_export = move |_| show_export.set(false);

    // State custom komponen
    let custom_components = create_rw_signal(Vec::<(String, String)>::new()); // (name, template)

    view! {
        <div class="leptos-studio">
            <Sidebar custom_components=custom_components />
            <div style="display: flex; flex-direction: column; flex: 1;">
                <div style="padding: 0.5rem; background: #f0f0f0; border-bottom: 1px solid #ddd;">
                    <button on:click=save_layout>Save Layout</button>
                    <button on:click=load_layout style="margin-left: 0.5rem;">Load Layout</button>
                    <button on:click=do_export style="margin-left: 0.5rem;">Export</button>
                </div>
                <Canvas selected=selected components=components />
                <div style="margin-top:2rem;">
                    <h4>Live Preview</h4>
                    <Preview components=components />
                </div>
            </div>
            <PropertyEditor selected=selected components=components />
            {move || if show_export.get() {
                view! {
                    <div style="position:fixed;top:0;left:0;width:100vw;height:100vh;background:rgba(0,0,0,0.3);z-index:1000;display:flex;align-items:center;justify-content:center;">
                        <div style="background:#fff;padding:2rem;border-radius:8px;min-width:400px;max-width:90vw;">
                            <h3>Export Leptos Code</h3>
                            <textarea style="width:100%;height:300px;" readonly>{export_code.get()}</textarea>
                            <div style="text-align:right;margin-top:1rem;">
                                <button on:click=close_export>Tutup</button>
                            </div>
                        </div>
                    </div>
                }
            } else { view! { <div></div> } }}
        </div>
    }
}
