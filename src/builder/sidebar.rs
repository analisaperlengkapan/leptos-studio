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
    // State untuk custom theme color
    let custom_theme_color = create_rw_signal(String::from("#888"));
    // State untuk edit custom component
    let editing_idx = create_rw_signal(None::<usize>);
    let edit_name = create_rw_signal(String::new());
    let edit_template = create_rw_signal(String::new());
    // State untuk validasi props
    let error_msg = create_rw_signal(String::new());
    // Handler ganti responsive mode
    let set_responsive = move |m: ResponsiveMode| responsive_mode.set(m);
    // Handler ganti theme
    let set_theme = move |t: Theme| theme.set(t);
    // Handler ganti warna custom theme
    let set_custom_theme_color = {
        let custom_theme_color = custom_theme_color.clone();
        move |color: String| custom_theme_color.set(color)
    };
    // State untuk form tambah komponen
    let show_add_form = create_rw_signal(false);
    let new_name = create_rw_signal(String::new());
    let new_template = create_rw_signal(String::new());
    // State untuk search/filter komponen
    let filter_query = create_rw_signal(String::new());
    // Handler tambah komponen custom
    let add_custom_component = move |_| {
        let name = new_name.get().trim().to_string();
        let template = new_template.get().trim().to_string();
        if name.is_empty() {
            error_msg.set("Nama komponen wajib diisi.".to_string());
            return;
        }
        // Nama harus valid sebagai identifier Rust (huruf, angka, underscore, tidak boleh diawali angka)
        if !name.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false)
            || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            error_msg.set("Nama komponen hanya boleh huruf, angka, dan underscore, serta tidak boleh diawali angka.".to_string());
            return;
        }
        if custom_components.get().iter().any(|(n,_)| n == &name) {
            error_msg.set("Nama komponen sudah ada.".to_string());
            return;
        }
        // Validasi template: harus mengandung setidaknya satu tag HTML sederhana (misal: <div>, <span>, dst)
        if !template.contains('<') || !template.contains('>') {
            error_msg.set("Template harus mengandung minimal satu tag HTML valid.".to_string());
            return;
        }
        if template.len() < 5 {
            error_msg.set("Template terlalu pendek.".to_string());
            return;
        }
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
        error_msg.set(String::new());
        show_add_form.set(false);
        notification.set(Some(format!("✅ Komponen '{}' berhasil ditambahkan!", name)));
        // Clear notification after 2.5s
        let notification = notification.clone();
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(2500).await;
            notification.set(None);
        });
    };
    // Handler hapus komponen custom
    let delete_custom_component = move |idx: usize| {
        let name = custom_components.get().get(idx).map(|(n,_)| n.clone()).unwrap_or_default();
        custom_components.update(|cc| { cc.remove(idx); });
        component_library.update(|lib| {
            if let Some(pos) = lib.iter().position(|c| c.kind == "Custom" && c.name == name) {
                lib.remove(pos);
            }
        });
        notification.set(Some(format!("🗑️ Komponen '{}' dihapus.", name)));
        let notification = notification.clone();
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(2000).await;
            notification.set(None);
        });
    };
    // Handler mulai edit custom component
    let start_edit_custom_component = {
        let custom_components = custom_components.clone();
        let edit_name = edit_name.clone();
        let edit_template = edit_template.clone();
        let editing_idx = editing_idx.clone();
        move |idx: usize| {
            if let Some((name, template)) = custom_components.get().get(idx) {
                edit_name.set(name.clone());
                edit_template.set(template.clone());
                editing_idx.set(Some(idx));
                error_msg.set(String::new());
            }
        }
    };
    // Handler simpan edit custom component
    let save_edit_custom_component = {
        let custom_components = custom_components.clone();
        let component_library = component_library.clone();
        let edit_name = edit_name.clone();
        let edit_template = edit_template.clone();
        let editing_idx = editing_idx.clone();
        let error_msg = error_msg.clone();
    move |_| {
            let idx = match editing_idx.get() { Some(i) => i, None => return };
            let name = edit_name.get().trim().to_string();
            let template = edit_template.get().trim().to_string();
            if name.is_empty() {
                error_msg.set("Nama komponen wajib diisi.".to_string());
                return;
            }
            if !name.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false)
                || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                error_msg.set("Nama komponen hanya boleh huruf, angka, dan underscore, serta tidak boleh diawali angka.".to_string());
                return;
            }
            if custom_components.get().iter().enumerate().any(|(i, (n, _)): (usize, &(String, String))| n == &name && i != idx) {
                error_msg.set("Nama komponen sudah ada.".to_string());
                return;
            }
            if !template.contains('<') || !template.contains('>') {
                error_msg.set("Template harus mengandung minimal satu tag HTML valid.".to_string());
                return;
            }
            if template.len() < 5 {
                error_msg.set("Template terlalu pendek.".to_string());
                return;
            }
            // Update custom_components
            custom_components.update(|cc| {
                if let Some(item) = cc.get_mut(idx) {
                    item.0 = name.clone();
                    item.1 = template.clone();
                }
            });
            // Update component_library
            component_library.update(|lib| {
                if let Some(item) = lib.iter_mut().find(|c| c.kind == "Custom" && c.name == custom_components.get().get(idx).map(|(n, _): &(String, String)| n.clone()).unwrap_or_default()) {
                    item.name = name.clone();
                    item.template = Some(template.clone());
                }
            });
            editing_idx.set(None);
            error_msg.set(String::new());
            notification.set(Some(format!("✏️ Komponen '{}' berhasil diubah!", name)));
            let notification = notification.clone();
            wasm_bindgen_futures::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(2000).await;
                notification.set(None);
            });
        }
    };
    // Handler batal edit
    let cancel_edit_custom_component = move |_| {
        editing_idx.set(None);
        error_msg.set(String::new());
    };
    let sidebar_bg = match theme.get() {
        Theme::Light => "#fff".to_string(),
        Theme::Dark => "#222".to_string(),
        Theme::Custom => custom_theme_color.get(),
    };
    view! {
    <aside style=format!("background:{};padding:1rem;min-width:260px;", sidebar_bg)>
    // ...existing code...
    // Pass custom_theme_color to canvas as a prop (update App and Canvas signatures as needed)
            <h2>Sidebar</h2>
            <div style="margin-bottom:8px;">
                <b>Theme:</b> {format!("{:?}", theme.get())}
                <div style="margin-top:4px;display:flex;gap:8px;">
                    <button on:click=move |_| set_theme(Theme::Light) disabled=theme.get() == Theme::Light>Light</button>
                    <button on:click=move |_| set_theme(Theme::Dark) disabled=theme.get() == Theme::Dark>Dark</button>
                    <button on:click=move |_| set_theme(Theme::Custom) disabled=theme.get() == Theme::Custom>Custom</button>
                </div>
                {move || if theme.get() == Theme::Custom {
                    view! {
                        <div style="margin-top:8px;">
                            <label for="custom-theme-color"><b>Sidebar Color:</b></label>
                            <input id="custom-theme-color" type="color" prop:value=custom_theme_color on:input=move |ev| set_custom_theme_color(event_target_value(&ev)) style="margin-left:8px;vertical-align:middle;" />
                        </div>
                    }
                } else { view! { <div></div> } }}
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
            <div>
                <b>Component Library:</b>
                <input
                    placeholder="Cari komponen..."
                    prop:value=filter_query
                    on:input=move |ev| filter_query.set(event_target_value(&ev))
                    style="margin:8px 0;width:100%;padding:4px;"
                />
                <ul style="max-height:180px;overflow:auto;">
                    {move || {
                        let query = filter_query.get().to_lowercase();
                        component_library.get()
                            .iter()
                            .enumerate()
                            .filter(|(_, c)| c.name.to_lowercase().contains(&query))
                            .map(|(i, c)| {
                                let is_custom = c.kind == "Custom";
                                let is_editing = editing_idx.get() == Some(i);
                                if is_custom && is_editing {
                                    view! {
                                        <li style="display:flex;flex-direction:column;gap:4px;background:#f3f3f3;padding:8px;border-radius:4px;">
                                            <b>Edit Komponen:</b>
                                            <input placeholder="Nama Komponen" prop:value=edit_name on:input=move |ev| edit_name.set(event_target_value(&ev)) style="margin-bottom:4px;"/>
                                            <textarea placeholder="Template HTML" prop:value=edit_template on:input=move |ev| edit_template.set(event_target_value(&ev)) style="margin-bottom:4px;width:100%;"/>
                                            {move || if !error_msg.get().is_empty() {
                                                view! { <div style="color:red;margin-bottom:4px;">{error_msg.get()}</div> }
                                            } else { view! { <div></div> } }}
                                            <div style="display:flex;gap:8px;">
                                                <button on:click=save_edit_custom_component>Simpan</button>
                                                <button on:click=cancel_edit_custom_component style="color:red;">Batal</button>
                                            </div>
                                        </li>
                                    }
                                } else {
                                    view! {
                                        <li style="display:flex;align-items:center;gap:8px;">
                                            <span>{format!("{}{}", c.name, if is_custom { " (Custom)" } else { "" })}</span>
                                            {is_custom.then(|| view! { <button style="color:orange;" on:click=move |_| start_edit_custom_component(i)>Edit</button> })}
                                            {is_custom.then(|| view! { <button style="color:red;" on:click=move |_| delete_custom_component(i)>Hapus</button> })}
                                        </li>
                                    }
                                }
                            })
                            .collect_view()
                    }}
                </ul>
                <button on:click=move |_| show_add_form.set(true) style="margin-top:8px;">Tambah Komponen</button>
                {move || if show_add_form.get() {
                    view! {
                        <div style="margin-top:8px;padding:8px;border:1px solid #ccc;background:#f9f9f9;">
                            <input placeholder="Nama Komponen" prop:value=new_name on:input=move |ev| new_name.set(event_target_value(&ev)) style="margin-bottom:4px;"/>
                            <textarea placeholder="Template HTML" prop:value=new_template on:input=move |ev| new_template.set(event_target_value(&ev)) style="margin-bottom:4px;width:100%;"/>
                            {move || if !error_msg.get().is_empty() {
                                view! { <div style="color:red;margin-bottom:4px;">{error_msg.get()}</div> }
                            } else { view! { <div></div> } }}
                            <div style="display:flex;gap:8px;">
                                <button on:click=add_custom_component>Tambah</button>
                                <button on:click=move |_| { show_add_form.set(false); error_msg.set(String::new()); } style="color:red;">Batal</button>
                            </div>
                        </div>
                    }
                } else { view! { <div></div> } }}
            </div>
            <div><b>Components on Canvas:</b> {components.get().len()}</div>
            // Tombol export dinonaktifkan sementara, handler dihapus untuk refactor
            <button disabled>Export Project</button>
            {move || notification.get().as_ref().map(|msg| view! { <div style="color:green;font-weight:bold;margin-top:8px;">{msg}</div> })}
        </aside>
    }.into_view()
}