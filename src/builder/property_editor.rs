use crate::domain::CanvasComponent;
use crate::state::AppState;
use leptos::prelude::*;
use super::property_editors::{
    ButtonPropertyEditor, TextPropertyEditor, InputPropertyEditor, ContainerPropertyEditor, CustomPropertyEditor
};

#[component]
pub fn PropertyEditor() -> impl IntoView {
    // Get app state from context
    let app_state = AppState::use_context();
    let canvas_state = app_state.canvas;

    view! {
        <section class="property-editor">
            <h3>{"Property Editor"}</h3>
            {move || {
                if let Some(selected_id) = canvas_state.selected.get() {
                    if let Some(comp) = canvas_state.get_component(&selected_id) {
                        match comp {
                            CanvasComponent::Button(btn) => {
                                view! {
                                    <ButtonPropertyEditor id=selected_id button=btn />
                                }.into_any()
                            },
                            CanvasComponent::Text(txt) => {
                                view! {
                                    <TextPropertyEditor id=selected_id text=txt />
                                }.into_any()
                            },
                            CanvasComponent::Input(inp) => {
                                view! {
                                    <InputPropertyEditor id=selected_id input=inp />
                                }.into_any()
                            },
                            CanvasComponent::Container(container) => {
                                view! {
                                    <ContainerPropertyEditor id=selected_id container=container />
                                }.into_any()
                            },
                            CanvasComponent::Custom(custom) => {
                                view! {
                                    <CustomPropertyEditor _id=selected_id custom=custom />
                                }.into_any()
                            },
                        }
                    } else {
                        view! { <div><p>{"Component not found"}</p></div> }.into_any()
                    }
                } else {
                    view! { <div><p>{"Select a component to edit properties"}</p></div> }.into_any()
                }
            }}
        </section>
    }
}
