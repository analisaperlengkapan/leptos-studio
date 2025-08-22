use leptos::*;
use super::canvas::CanvasComponent;

#[component]
pub fn Preview(components: RwSignal<Vec<CanvasComponent>>) -> impl IntoView {
    view! {
        <div class="preview-area" style="background:#f9f9f9;padding:1rem;border:1px solid #eee;">
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
