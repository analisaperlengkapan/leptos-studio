use leptos::*;
use super::canvas::CanvasComponent;
use super::component_library::Theme;
use super::component_library::ResponsiveMode;

#[component]
pub fn Preview(components: RwSignal<Vec<CanvasComponent>>, theme: RwSignal<Theme>, responsive_mode: RwSignal<ResponsiveMode>) -> impl IntoView {
    let (bg, fg) = match theme.get() {
        Theme::Light => ("#fff", "#222"),
        Theme::Dark => ("#222", "#eee"),
        Theme::Custom => ("#f5f5dc", "#1a237e"), // sinkron dengan canvas.rs, bisa dihubungkan ke context jika ingin dinamis
    };
    let width = match responsive_mode.get() {
        ResponsiveMode::Desktop => "100%",
        ResponsiveMode::Tablet => "768px",
        ResponsiveMode::Mobile => "375px",
    };
    view! {
        <div class="preview-area" style={format!("background:{};color:{};padding:1rem;border:1px solid #eee;width:{};margin:auto;", bg, fg, width)}>
            <For
                each=move || components.get().into_iter().enumerate()
                key=|(idx, _)| *idx
                children=move |(_idx, comp)| {
                    match comp {
                        CanvasComponent::Button { label } => view! { <div><button style="margin:0.5rem;">{label.clone()}</button></div> },
                        CanvasComponent::Text { content } => view! { <div><span style="margin:0.5rem;">{content.clone()}</span></div> },
                        CanvasComponent::Input { placeholder } => view! { <div><input placeholder=placeholder.clone() style="margin:0.5rem;"/></div> },
                        CanvasComponent::Custom { name, template } => view! { <div style="color:#7b1fa2;margin:0.5rem;">Custom: {name.clone()}<div>{template.clone()}</div></div> },
                        CanvasComponent::Container { .. } => view! { <div class="container" style="margin:0.5rem;">Container</div> },
                    }
                }
            />
        </div>
    }
}
