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
            <h3 class="panel-title">"Global Variables"</h3>
            <p class="panel-description">"Define variables to bind to component properties."</p>

            <form on:submit=add_variable class="variable-form">
                <div class="form-group">
                    <label>"Name"</label>
                    <input
                        type="text"
                        class="input-text"
                        placeholder="e.g., user_name"
                        prop:value=move || new_name.get()
                        on:input=move |ev| new_name.set(event_target_value(&ev))
                    />
                </div>
                <div class="form-group">
                    <label>"Type"</label>
                    <select
                        class="input-select"
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            let t = match val.as_str() {
                                "Number" => VariableType::Number,
                                "Boolean" => VariableType::Boolean,
                                _ => VariableType::String,
                            };
                            new_type.set(t);
                        }
                    >
                        <option value="String">"String"</option>
                        <option value="Number">"Number"</option>
                        <option value="Boolean">"Boolean"</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>"Default Value"</label>
                    <input
                        type="text"
                        class="input-text"
                        placeholder="Value..."
                        prop:value=move || new_default.get()
                        on:input=move |ev| new_default.set(event_target_value(&ev))
                    />
                </div>
                <button type="submit" class="btn btn-primary btn-sm" style="width: 100%">"Add Variable"</button>
            </form>

            {move || if !error_msg.get().is_empty() {
                view! { <div class="error-message">{error_msg.get()}</div> }.into_any()
            } else {
                ().into_any()
            }}

            <div class="variables-list">
                {move || variables.get().into_iter().enumerate().map(|(idx, var)| {
                    view! {
                        <div class="variable-item">
                            <div class="variable-info">
                                <span class="variable-name">{var.name}</span>
                                <span class="variable-meta">{var.data_type.to_string()} " = " {var.default_value}</span>
                            </div>
                            <button
                                class="btn-icon"
                                on:click=move |_| delete_variable(idx)
                                title="Delete variable"
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
