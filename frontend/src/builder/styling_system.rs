use crate::domain::{ComponentStyle, ThemePreset};
use leptos::prelude::*;

/// Styling Editor Component
#[component]
pub fn StyleEditor(
    #[prop(into)] style: Signal<ComponentStyle>,
    #[prop(into)] on_change: Callback<ComponentStyle>,
) -> impl IntoView {
    // Initialize local signals from the passed prop value.
    // We start with the current value of the signal/static prop.
    let initial_style = style.get();

    let bg_color = RwSignal::new(initial_style.background_color.clone().unwrap_or_default());
    let color = RwSignal::new(initial_style.color.clone().unwrap_or_default());
    let padding = RwSignal::new(initial_style.padding.clone().unwrap_or_default());
    let margin = RwSignal::new(initial_style.margin.clone().unwrap_or_default());
    let border_radius = RwSignal::new(
        initial_style
            .border_radius
            .map(|v| v.to_string())
            .unwrap_or_default(),
    );
    let border_width = RwSignal::new(
        initial_style
            .border_width
            .map(|v| v.to_string())
            .unwrap_or_default(),
    );
    let border_color = RwSignal::new(initial_style.border_color.clone().unwrap_or_default());
    let font_size = RwSignal::new(
        initial_style
            .font_size
            .map(|v| v.to_string())
            .unwrap_or_default(),
    );

    // Use an RwSignal to store the current accumulated style state
    let current_style = RwSignal::new(initial_style.clone());

    // Effect to sync local signals when the `style` prop changes.
    // This supports both component recreation (initial run) and reactive updates if `style` is a signal.
    Effect::new(move |_| {
        let new_style = style.get();
        // Update local signals
        bg_color.set(new_style.background_color.clone().unwrap_or_default());
        color.set(new_style.color.clone().unwrap_or_default());
        padding.set(new_style.padding.clone().unwrap_or_default());
        margin.set(new_style.margin.clone().unwrap_or_default());
        border_radius.set(
            new_style
                .border_radius
                .map(|v| v.to_string())
                .unwrap_or_default(),
        );
        border_width.set(
            new_style
                .border_width
                .map(|v| v.to_string())
                .unwrap_or_default(),
        );
        border_color.set(new_style.border_color.clone().unwrap_or_default());
        font_size.set(
            new_style
                .font_size
                .map(|v| v.to_string())
                .unwrap_or_default(),
        );
        current_style.set(new_style);
    });

    let update = move |field: &str, value: String| {
        let mut new_style = current_style.get();

        match field {
            "background_color" => {
                new_style.background_color = if value.is_empty() { None } else { Some(value) };
                bg_color.set(new_style.background_color.clone().unwrap_or_default());
            }
            "color" => {
                new_style.color = if value.is_empty() { None } else { Some(value) };
                color.set(new_style.color.clone().unwrap_or_default());
            }
            "padding" => {
                new_style.padding = if value.is_empty() { None } else { Some(value) };
                padding.set(new_style.padding.clone().unwrap_or_default());
            }
            "margin" => {
                new_style.margin = if value.is_empty() { None } else { Some(value) };
                margin.set(new_style.margin.clone().unwrap_or_default());
            }
            "border_radius" => {
                new_style.border_radius = value.parse().ok();
                border_radius.set(value);
            }
            "border_width" => {
                new_style.border_width = value.parse().ok();
                border_width.set(value);
            }
            "border_color" => {
                new_style.border_color = if value.is_empty() { None } else { Some(value) };
                border_color.set(new_style.border_color.clone().unwrap_or_default());
            }
            "font_size" => {
                new_style.font_size = value.parse().ok();
                font_size.set(value);
            }
            _ => {}
        }
        current_style.set(new_style.clone());
        on_change.run(new_style);
    };

    view! {
        <div class="style-editor">
            <h4>{"Styling"}</h4>

            <div class="style-grid" style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
                <div class="style-control">
                    <label>{"Background"}</label>
                    <div style="display: flex; gap: 4px;">
                        <input
                            type="color"
                            prop:value=bg_color
                            on:input={
                                // Removed .clone() as update is Copy
                                move |ev| update("background_color", event_target_value(&ev))
                            }
                            style="width: 30px; padding: 0; border: none;"
                        />
                        <input
                            type="text"
                            prop:value=bg_color
                            on:input={
                                // Removed .clone() as update is Copy
                                move |ev| update("background_color", event_target_value(&ev))
                            }
                            style="flex: 1; min-width: 0;"
                        />
                    </div>
                </div>

                <div class="style-control">
                    <label>{"Text Color"}</label>
                    <div style="display: flex; gap: 4px;">
                         <input
                            type="color"
                            prop:value=color
                            on:input={
                                // Removed .clone() as update is Copy
                                move |ev| update("color", event_target_value(&ev))
                            }
                            style="width: 30px; padding: 0; border: none;"
                        />
                        <input
                            type="text"
                            prop:value=color
                            on:input={
                                // Removed .clone() as update is Copy
                                move |ev| update("color", event_target_value(&ev))
                            }
                            style="flex: 1; min-width: 0;"
                        />
                    </div>
                </div>

                <div class="style-control">
                    <label>{"Padding"}</label>
                    <input
                        type="text"
                        placeholder="8px"
                        prop:value=padding
                        on:input={
                            // Removed .clone() as update is Copy
                            move |ev| update("padding", event_target_value(&ev))
                        }
                    />
                </div>

                <div class="style-control">
                    <label>{"Margin"}</label>
                    <input
                        type="text"
                        placeholder="8px"
                        prop:value=margin
                        on:input={
                            // Removed .clone() as update is Copy
                            move |ev| update("margin", event_target_value(&ev))
                        }
                    />
                </div>

                <div class="style-control">
                    <label>{"Border Radius"}</label>
                    <input
                        type="number"
                        min="0"
                        prop:value=border_radius
                        on:input={
                            // Removed .clone() as update is Copy
                            move |ev| update("border_radius", event_target_value(&ev))
                        }
                    />
                </div>

                <div class="style-control">
                    <label>{"Border Width"}</label>
                    <input
                        type="number"
                        min="0"
                        prop:value=border_width
                        on:input={
                            // Removed .clone() as update is Copy
                            move |ev| update("border_width", event_target_value(&ev))
                        }
                    />
                </div>

                <div class="style-control">
                    <label>{"Border Color"}</label>
                     <div style="display: flex; gap: 4px;">
                         <input
                            type="color"
                            prop:value=border_color
                            on:input={
                                // Removed .clone() as update is Copy
                                move |ev| update("border_color", event_target_value(&ev))
                            }
                            style="width: 30px; padding: 0; border: none;"
                        />
                        <input
                            type="text"
                            prop:value=border_color
                            on:input={
                                // Removed .clone() as update is Copy
                                move |ev| update("border_color", event_target_value(&ev))
                            }
                            style="flex: 1; min-width: 0;"
                        />
                    </div>
                </div>

                <div class="style-control">
                    <label>{"Font Size (px)"}</label>
                    <input
                        type="number"
                        min="0"
                        prop:value=font_size
                        on:input={
                            // Removed .clone() as update is Copy
                            move |ev| update("font_size", event_target_value(&ev))
                        }
                    />
                </div>
            </div>
        </div>
    }
}

/// Theme Selector Component
#[component]
pub fn ThemeSelector(#[prop(into)] on_theme_select: Callback<ThemePreset>) -> impl IntoView {
    let themes = vec![
        ThemePreset::Light,
        ThemePreset::Dark,
        ThemePreset::HighContrast,
        ThemePreset::Colorful,
        ThemePreset::Minimal,
    ];

    view! {
        <div class="theme-selector">
            <h4>{"Theme Presets"}</h4>
            <div class="theme-buttons">
                {themes
                    .into_iter()
                    .map(|theme| {
                        view! {
                            <button
                                class="theme-btn"
                                on:click=move |_| on_theme_select.run(theme.clone())
                            >
                                {theme.as_str()}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
