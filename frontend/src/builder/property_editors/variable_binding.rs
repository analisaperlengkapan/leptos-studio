use crate::state::AppState;
use leptos::prelude::*;

use crate::domain::ComponentId;

#[component]
pub fn VariableBinding(
    #[prop(optional)] component_id: Option<ComponentId>,
    #[prop(optional)] property_name: Option<String>,
    #[prop(optional)] current_binding: Option<String>,
) -> impl IntoView {
    let app_state = AppState::expect_context();
    let variables = app_state.variables;
    let canvas_state = app_state.canvas;

    view! {
        <div class="variable-binding" style="margin-left: 8px;">
            <select
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    let binding = if val.is_empty() { None } else { Some(val) };

                    if let (Some(id), Some(prop)) = (component_id, property_name.clone()) {
                        canvas_state.record_snapshot(&format!("Update binding for {}", prop));
                        canvas_state.update_component(&id, |c| {
                            match c {
                                crate::domain::CanvasComponent::Button(btn) => {
                                    if let Some(b) = binding.clone() { btn.bindings.insert(prop, b); } else { btn.bindings.remove(&prop); }
                                }
                                crate::domain::CanvasComponent::Text(txt) => {
                                    if let Some(b) = binding.clone() { txt.bindings.insert(prop, b); } else { txt.bindings.remove(&prop); }
                                }
                                crate::domain::CanvasComponent::Input(inp) => {
                                    if let Some(b) = binding.clone() { inp.bindings.insert(prop, b); } else { inp.bindings.remove(&prop); }
                                }
                                crate::domain::CanvasComponent::Select(sel) => {
                                    if let Some(b) = binding.clone() { sel.bindings.insert(prop, b); } else { sel.bindings.remove(&prop); }
                                }
                                crate::domain::CanvasComponent::Image(img) => {
                                    if let Some(b) = binding.clone() { img.bindings.insert(prop, b); } else { img.bindings.remove(&prop); }
                                }
                                crate::domain::CanvasComponent::Container(con) => {
                                    if let Some(b) = binding.clone() { con.bindings.insert(prop, b); } else { con.bindings.remove(&prop); }
                                }
                                crate::domain::CanvasComponent::Card(crd) => {
                                    if let Some(b) = binding.clone() { crd.bindings.insert(prop, b); } else { crd.bindings.remove(&prop); }
                                }
                                _ => {}
                            }
                        });
                    }
                }
                style="max-width: 100px; padding: 2px;"
                title="Bind to variable"
            >
                <option value="" selected=current_binding.as_ref().is_none_or(|s| s.is_empty())>"No Binding"</option>
                {move || variables.get().into_iter().map(|v| {
                    let is_selected = current_binding.as_ref() == Some(&v.name);
                    view! {
                        <option value={v.name.clone()} selected=is_selected>
                            {v.name.clone()}
                        </option>
                    }
                }).collect::<Vec<_>>()}
            </select>
        </div>
    }
}
