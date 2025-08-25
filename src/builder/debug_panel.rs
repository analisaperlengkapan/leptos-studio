use leptos::*;

#[component]
pub fn DebugPanel(
    components: RwSignal<Vec<crate::builder::canvas::CanvasComponent>>,
    custom_components: RwSignal<Vec<crate::builder::component_library::LibraryComponent>>,
    undo_stack: RwSignal<Vec<Vec<crate::builder::canvas::CanvasComponent>>>,
    redo_stack: RwSignal<Vec<Vec<crate::builder::canvas::CanvasComponent>>>,
    render_count: std::rc::Rc<std::cell::Cell<u32>>,
    render_time: std::rc::Rc<std::cell::Cell<f64>>,
) -> impl IntoView {
    view! {
        <div style="font-size:12px;background:#222;color:#fff;padding:8px;border-radius:6px;margin-bottom:12px;">
            <b>Debug Panel</b>
            <div><b>Components:</b> {format!("{}", components.get().len())}</div>
            <div><b>Custom Components:</b> {format!("{}", custom_components.get().len())}</div>
            <div><b>Undo stack:</b> {undo_stack.get().len()}</div>
            <div><b>Redo stack:</b> {redo_stack.get().len()}</div>
            <div><b>Render count:</b> {render_count.get()}</div>
            <div><b>Render time:</b> {format!("{:.2} ms", render_time.get())}</div>
            <details style="margin-top:8px;"><summary>Dump State</summary>
                <pre style="font-size:11px;max-height:200px;overflow:auto;background:#111;color:#eee;padding:6px;">{format!("{:?}", components.get())}</pre>
            </details>
        </div>
    }
}
