use crate::domain::{Variable, VariableType};
use crate::state::AppState;
use leptos::prelude::*;
use web_sys::SubmitEvent;

#[component]
pub fn VariablePanel() -> impl IntoView {
    let app_state = AppState::expect_context();
    let variables = app_state.variables;

    let new_name = RwSignal::new(String::new());
    let new_type = RwSignal::new(VariableType::String);
    let new_default = RwSignal::new(String::new());
    let error_msg = RwSignal::new(String::new());

    let add_variable = move |ev: SubmitEvent| {
        ev.prevent_default();
        let name = new_name.get().trim().to_string();
        if name.is_empty() {
            error_msg.set("Name cannot be empty".to_string());
            return;
        }

        // Basic validation for variable name
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            error_msg.set("Name must be alphanumeric".to_string());
            return;
        }

        // Check duplicate
        if variables.get().iter().any(|v| v.name == name) {
            error_msg.set("Variable name already exists".to_string());
            return;
        }

        let var = Variable::new(name, new_type.get(), new_default.get());

        variables.update(|vars| vars.push(var));
        new_name.set(String::new());
        new_default.set(String::new());
        error_msg.set(String::new());
    };

    let delete_variable = move |idx: usize| {
        variables.update(|vars| {
            vars.remove(idx);
        });
    };

    view! {
        <div class="sidebar-section variable-panel">
            <b>"Global Variables"</b>

            <form on:submit=add_variable style="display: flex; flex-direction: column; gap: 8px; margin-top: 8px;">
                <input
                    type="text"
                    placeholder="Name (e.g., count)"
                    prop:value=move || new_name.get()
                    on:input=move |ev| new_name.set(event_target_value(&ev))
                    style="width: 100%; padding: 4px;"
                />
                <select
                    on:change=move |ev| {
                        let val = event_target_value(&ev);
                        let t = match val.as_str() {
                            "Number" => VariableType::Number,
                            "Boolean" => VariableType::Boolean,
                            _ => VariableType::String,
                        };
                        new_type.set(t);
                    }
                    style="width: 100%; padding: 4px;"
                >
                    <option value="String">"String"</option>
                    <option value="Number">"Number"</option>
                    <option value="Boolean">"Boolean"</option>
                </select>
                <input
                    type="text"
                    placeholder="Default Value"
                    prop:value=move || new_default.get()
                    on:input=move |ev| new_default.set(event_target_value(&ev))
                    style="width: 100%; padding: 4px;"
                />
                <button type="submit" style="padding: 4px;">"Add Variable"</button>
            </form>

            {move || if !error_msg.get().is_empty() {
                view! { <div style="color: red; font-size: 12px; margin-top: 4px;">{error_msg.get()}</div> }.into_any()
            } else {
                ().into_any()
            }}

            <div style="margin-top: 12px; display: flex; flex-direction: column; gap: 4px;">
                {move || variables.get().into_iter().enumerate().map(|(idx, var)| {
                    view! {
                        <div style="display: flex; justify-content: space-between; align-items: center; background: #f0f0f0; padding: 4px 8px; border-radius: 4px;">
                            <div style="display: flex; flex-direction: column; overflow: hidden;">
                                <span style="font-weight: bold; font-size: 12px;">{var.name}</span>
                                <span style="font-size: 10px; color: #666;">{var.data_type.to_string()} " = " {var.default_value}</span>
                            </div>
                            <button
                                on:click=move |_| delete_variable(idx)
                                style="border: none; background: none; color: #999; cursor: pointer;"
                                title="Delete"
                            >
                                "×"
                            </button>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
