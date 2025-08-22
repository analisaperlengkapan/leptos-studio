use leptos::*;
use super::canvas::{CanvasComponent, SelectedComponent};

#[component]
pub fn PropertyEditor(selected: RwSignal<SelectedComponent>, components: RwSignal<Vec<CanvasComponent>>) -> impl IntoView {
    let idx = move || selected.get().idx;
    let comp = move || idx().and_then(|i| components.get().get(i).cloned());

    view! {
        <section class="property-editor">
            <h3>Property Editor</h3>
            {move || match comp() {
                Some(CanvasComponent::Button { label }) => view! {
                    <div>
                        <label>Label: <input value=label on:input=move |ev| {
                            let val = event_target_value(&ev);
                            if let Some(i) = idx() {
                                components.update(|c| {
                                    if let CanvasComponent::Button { label } = &mut c[i] {
                                        *label = val.clone();
                                    }
                                });
                            }
                        } /></label>
                    </div>
                },
                _ => view! { <div><p>Pilih komponen untuk mengedit properti.</p></div> },
            }}
        </section>
    }
}
