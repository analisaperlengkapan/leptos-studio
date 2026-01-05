use crate::domain::CanvasComponent;
use crate::state::app_state::{AppState, ResponsiveMode, Theme};
use leptos::prelude::*;

#[component]
pub fn Preview() -> impl IntoView {
    // Get app state from context
    let app_state = AppState::expect_context();
    let canvas_state = app_state.canvas;
    let ui_state = app_state.ui;

    let preview_style = Memo::new(move |_| {
        let theme = app_state.settings.with(|s| s.theme.clone());
        let responsive_mode = ui_state.responsive_mode.get();

        let (bg, fg) = match theme {
            Theme::Light => ("#fff", "#222"),
            Theme::Dark => ("#222", "#eee"),
            Theme::Custom => ("#888", "#fff"),
        };
        let width = match responsive_mode {
            ResponsiveMode::Desktop => "100%",
            ResponsiveMode::Tablet => "768px",
            ResponsiveMode::TabletLandscape => "1024px",
            ResponsiveMode::Mobile => "375px",
            ResponsiveMode::MobileLandscape => "667px",
        };

        format!(
            "background:{};color:{};padding:1rem;border:1px solid #eee;width:{};margin:auto;",
            bg, fg, width
        )
    });
    view! {
        <div class="preview-area" style=move || preview_style.get()>
            <For
                each=move || canvas_state.components.get()
                key=|comp| *comp.id()
                children=move |comp| {
                    match comp {
                        CanvasComponent::Button(btn) => view! {
                            <div><button class="preview-inline-margin">{btn.label.clone()}</button></div>
                        }.into_any(),
                        CanvasComponent::Text(txt) => view! {
                            <div><span class="preview-inline-margin">{txt.content.clone()}</span></div>
                        }.into_any(),
                        CanvasComponent::Input(inp) => view! {
                            <div><input placeholder=inp.placeholder.clone() class="preview-inline-margin"/></div>
                        }.into_any(),
                        CanvasComponent::Custom(custom) => {
                            view! {
                                <div class="preview-custom">
                                    {"Custom: "}{custom.name.clone()}
                                    <div inner_html=custom.template.clone()></div>
                                </div>
                            }.into_any()
                        },
                        CanvasComponent::Container(_) => view! {
                            <div class="container preview-inline-margin">{"Container"}</div>
                        }.into_any(),
                        CanvasComponent::Image(img) => view! {
                            <div><img src=img.src alt=img.alt style="max-width: 100px" /></div>
                        }.into_any(),
                        CanvasComponent::Card(_) => view! {
                            <div class="card preview-inline-margin">{"Card"}</div>
                        }.into_any(),
                    }
                }
            />
        </div>
    }
}
