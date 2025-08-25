use leptos::*;
use super::component_library::{LibraryComponent, Theme, ResponsiveMode};
// ...existing code...

#[component]
pub fn sidebar(
    custom_components: RwSignal<Vec<(String, String)>>,
    theme: RwSignal<Theme>,
    responsive_mode: RwSignal<ResponsiveMode>,
    selected: RwSignal<crate::builder::canvas::SelectedComponent>,
    undo_stack: RwSignal<Vec<Vec<crate::builder::canvas::CanvasComponent>>>,
    redo_stack: RwSignal<Vec<Vec<crate::builder::canvas::CanvasComponent>>>,
    components: RwSignal<Vec<crate::builder::canvas::CanvasComponent>>,
    render_count: std::rc::Rc<std::cell::Cell<u32>>,
    render_time: std::rc::Rc<std::cell::Cell<f64>>,
    notification: RwSignal<Option<String>>,
    component_library: RwSignal<Vec<LibraryComponent>>,
) -> impl IntoView {
    // Handler ganti responsive mode
    let set_responsive = move |m: ResponsiveMode| responsive_mode.set(m);
    // Handler ganti theme
    let set_theme = move |t: Theme| theme.set(t);
    // State untuk form tambah komponen
    let show_add_form = create_rw_signal(false);
    let new_name = create_rw_signal(String::new());
    let new_template = create_rw_signal(String::new());
    // Handler tambah komponen custom
    let add_custom_component = move |_| {
        let name = new_name.get().trim().to_string();
        let template = new_template.get().trim().to_string();
        if !name.is_empty() {
            custom_components.update(|cc| cc.push((name.clone(), template.clone())));
            component_library.update(|lib| lib.push(LibraryComponent {
                name: name.clone(),
                kind: "Custom".to_string(),
                template: Some(template.clone()),
                category: "Custom".to_string(),
                props_schema: None,
                description: None,
            }));
            new_name.set(String::new());
            new_template.set(String::new());
            show_add_form.set(false);
        }
    };
    // Handler hapus komponen custom
    let delete_custom_component = move |idx: usize| {
        custom_components.update(|cc| { cc.remove(idx); });
        component_library.update(|lib| {
            if let Some(pos) = lib.iter().position(|c| c.kind == "Custom" && c.name == custom_components.get().get(idx).map(|(n,_)| n.clone()).unwrap_or_default()) {
                lib.remove(pos);
            }
        });
    };
    view! {
        <aside style=format!("background:{};padding:1rem;min-width:260px;", match theme.get() {
            Theme::Light => "#fff",
            Theme::Dark => "#222",
            Theme::Custom => "#888",
        })>
            <h2>Sidebar</h2>
            <div style="margin-bottom:8px;">
                <b>Theme:</b> {format!("{:?}", theme.get())}
                <div style="margin-top:4px;display:flex;gap:8px;">
                    <button on:click=move |_| set_theme(Theme::Light) disabled=theme.get() == Theme::Light>Light</button>
                    <button on:click=move |_| set_theme(Theme::Dark) disabled=theme.get() == Theme::Dark>Dark</button>
                    <button on:click=move |_| set_theme(Theme::Custom) disabled=theme.get() == Theme::Custom>Custom</button>
                </div>
            </div>
            <div style="margin-bottom:8px;">
                <b>Responsive:</b> {format!("{:?}", responsive_mode.get())}
                <div style="margin-top:4px;display:flex;gap:8px;">
                    <button on:click=move |_| set_responsive(ResponsiveMode::Desktop) disabled=responsive_mode.get() == ResponsiveMode::Desktop>Desktop</button>
                    <button on:click=move |_| set_responsive(ResponsiveMode::Tablet) disabled=responsive_mode.get() == ResponsiveMode::Tablet>Tablet</button>
                    <button on:click=move |_| set_responsive(ResponsiveMode::Mobile) disabled=responsive_mode.get() == ResponsiveMode::Mobile>Mobile</button>
                </div>
            </div>
            <div><b>Selected:</b> {selected.get().idx.map(|i| i.to_string()).unwrap_or("None".to_string())}</div>
            <div><b>Undo stack:</b> {undo_stack.get().len()}</div>
            <div><b>Redo stack:</b> {redo_stack.get().len()}</div>
            <div><b>Render count:</b> {render_count.get()}</div>
            <div><b>Render time:</b> {format!("{:.2} ms", render_time.get())}</div>
            <div><b>Custom Components:</b>
                <ul>
                    {move || custom_components.get().iter().enumerate().map(|(i, (name, _))| view! {
                        <li style="display:flex;align-items:center;gap:8px;">
                            <span>{name.clone()}</span>
                            <button style="color:red;" on:click=move |_| delete_custom_component(i)>Hapus</button>
                        </li>
                    }).collect_view()}
                </ul>
                <button on:click=move |_| show_add_form.set(true) style="margin-top:8px;">Tambah Komponen</button>
                {move || if show_add_form.get() {
                    view! {
                        <div style="margin-top:8px;padding:8px;border:1px solid #ccc;background:#f9f9f9;">
                            <input placeholder="Nama Komponen" prop:value=new_name on:input=move |ev| new_name.set(event_target_value(&ev)) style="margin-bottom:4px;"/>
                            <textarea placeholder="Template HTML" prop:value=new_template on:input=move |ev| new_template.set(event_target_value(&ev)) style="margin-bottom:4px;width:100%;"/>
                            <div style="display:flex;gap:8px;">
                                <button on:click=add_custom_component>Tambah</button>
                                <button on:click=move |_| show_add_form.set(false) style="color:red;">Batal</button>
                            </div>
                        </div>
                    }
                } else { view! { <div></div> } }}
            </div>
            <div><b>Components on Canvas:</b> {components.get().len()}</div>
            // Tombol export dinonaktifkan sementara, handler dihapus untuk refactor
            <button disabled>Export Project</button>
            {move || notification.get().as_ref().map(|msg| view! { <div style="color:green;">{msg}</div> })}
        </aside>
    }.into_view()
}