use leptos::prelude::*;
use crate::state::AppState;

#[component]
pub fn VariableBinding<F>(
    value: Option<String>,
    on_change: F,
) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Clone + Send + Sync,
{
    let app_state = AppState::expect_context();
    let variables = app_state.variables;

    let on_change = on_change.clone();

    view! {
        <div class="variable-binding" style="margin-left: 8px;">
            <select
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    if val.is_empty() {
                        on_change(None);
                    } else {
                        on_change(Some(val));
                    }
                }
                style="max-width: 100px; padding: 2px;"
                title="Bind to variable"
            >
                <option value="" selected=value.is_none()>"No Binding"</option>
                {move || variables.get().into_iter().map(|v| {
                    let is_selected = value.as_ref() == Some(&v.name);
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
