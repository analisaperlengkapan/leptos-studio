use leptos::prelude::*;
use crate::builder::design_tokens::DesignTokens;
use crate::builder::property_inputs::{ColorInput, StringInput};

#[component]
pub fn ThemeEditor(tokens: RwSignal<DesignTokens>) -> impl IntoView {
    view! {
        <div class="theme-editor">
            <h3 class="theme-title">"Global Theme"</h3>
            <p class="theme-description">
                "Customize global design tokens. Changes apply immediately."
            </p>

            <div class="theme-section">
                <h4 class="theme-section-title">"Colors"</h4>
                <div class="colors-list">
                    <For
                        each=move || tokens.get().colors
                        key=|color| color.name.clone()
                        children=move |color| {
                            let color_name = color.name.clone();
                            view! {
                                <ColorInput
                                    label=color.name.clone()
                                    value=color.value
                                    on_change=move |new_val| {
                                        tokens.update(|t| t.update_color(&color_name, new_val));
                                    }
                                />
                            }
                        }
                    />
                </div>
            </div>

            <div class="theme-section">
                <h4 class="theme-section-title">"Typography"</h4>
                <div class="typography-list">
                    <For
                        each=move || tokens.get().typography
                        key=|typo| typo.name.clone()
                        children=move |typo| {
                            let name_clone_fs = typo.name.clone();
                            let name_clone_lh = typo.name.clone();
                            view! {
                                <div class="typography-item">
                                    <div class="typography-name">{typo.name}</div>
                                    <StringInput
                                        label="Font Size".to_string()
                                        value=typo.font_size
                                        on_change=move |val| {
                                            tokens.update(|t| t.update_typography(&name_clone_fs, "font_size", val));
                                        }
                                    />
                                    <StringInput
                                        label="Line Height".to_string()
                                        value=typo.line_height
                                        on_change=move |val| {
                                            tokens.update(|t| t.update_typography(&name_clone_lh, "line_height", val));
                                        }
                                    />
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
