use leptos::prelude::*;

#[component]
pub fn StringInput<F>(
    value: String,
    label: String,
    on_change: F,
) -> impl IntoView
where
    F: Fn(String) + 'static + Clone + Send + Sync,
{
    let on_change = on_change.clone();
    view! {
        <div class="property-field">
            <label>
                {label}
                <input
                    type="text"
                    prop:value=value
                    on:input=move |ev| {
                        on_change(event_target_value(&ev));
                    }
                />
            </label>
        </div>
    }
}

#[component]
pub fn NumberInput<F>(
    value: f64,
    label: String,
    on_change: F,
) -> impl IntoView
where
    F: Fn(f64) + 'static + Clone + Send + Sync,
{
    let on_change = on_change.clone();
    view! {
        <div class="property-field">
            <label>
                {label}
                <input
                    type="number"
                    prop:value=value.to_string()
                    on:input=move |ev| {
                        let raw = event_target_value(&ev);
                        if let Ok(parsed) = raw.parse::<f64>() {
                            on_change(parsed);
                        }
                    }
                />
            </label>
        </div>
    }
}

#[component]
pub fn BoolCheckbox<F>(
    checked: bool,
    label: String,
    on_change: F,
) -> impl IntoView
where
    F: Fn(bool) + 'static + Clone + Send + Sync,
{
    let on_change = on_change.clone();
    view! {
        <div class="property-field">
            <label>
                {label}
                <input
                    type="checkbox"
                    prop:checked=checked
                    on:change=move |_| {
                        // For checkboxes, we toggle the current state effectively,
                        // but strictly we should read the new state.
                        // However, standard pattern is usually just passing !checked or reading checked.
                        // Let's read the element's checked state to be precise, or just invert if controlled.
                        // Since we are controlled, inverting is safe if 'checked' is fresh.
                        on_change(!checked);
                    }
                />
            </label>
        </div>
    }
}

#[component]
pub fn EnumSelect<F>(
    value: String,
    label: String,
    options: Vec<String>,
    on_change: F,
) -> impl IntoView
where
    F: Fn(String) + 'static + Clone + Send + Sync,
{
    let on_change = on_change.clone();
    view! {
        <div class="property-field">
            <label>
                {label}
                <select
                    on:change=move |ev| {
                        on_change(event_target_value(&ev));
                    }
                >
                    {options.into_iter().map(|opt| {
                        let opt_clone = opt.clone();
                        let is_selected = opt_clone == value;
                        view! {
                            <option value=opt_clone selected=is_selected>{opt}</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
            </label>
        </div>
    }
}
