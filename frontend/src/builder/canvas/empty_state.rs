use leptos::prelude::*;

#[component]
pub fn CanvasEmptyState() -> impl IntoView {
    view! {
        <div class="canvas-empty-state">
            <p>"Drag components here to start building"</p>
        </div>
    }
}
