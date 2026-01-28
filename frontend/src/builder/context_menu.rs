use crate::domain::ComponentId;
use leptos::prelude::*;

#[component]
pub fn ContextMenu(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] position: Signal<(f64, f64)>,
    #[prop(into)] component_id: Signal<Option<ComponentId>>,
    on_close: Callback<()>,
    on_delete: Callback<ComponentId>,
    on_duplicate: Callback<ComponentId>,
    on_select_parent: Callback<ComponentId>,
) -> impl IntoView {
    // We attach a click listener to the window to close the menu when clicking outside
    // This is handled by the parent component or via a global listener, but usually
    // a transparent overlay is easier for modals/popups.
    // Here we assume the parent renders an overlay or handles outside clicks.

    let style = move || {
        let (x, y) = position.get();
        if visible.get() {
            format!(
                "display: block; position: fixed; left: {}px; top: {}px; z-index: 9999;",
                x, y
            )
        } else {
            "display: none;".to_string()
        }
    };

    view! {
        <div
            class="context-menu bg-white shadow-xl rounded-lg border border-gray-200 py-1 min-w-[160px]"
            style=style
            on:contextmenu=|ev| ev.prevent_default() // Prevent native menu on our menu
        >
            <div class="px-3 py-2 text-xs text-gray-500 font-medium border-b border-gray-100">
                {move || format!("Component {}", component_id.get().map(|id| id.to_string().chars().take(8).collect::<String>()).unwrap_or_default())}
            </div>

            <button
                class="w-full text-left px-4 py-2 hover:bg-blue-50 text-sm text-gray-700 flex items-center gap-2 transition-colors"
                on:click=move |_| {
                    if let Some(id) = component_id.get() {
                        on_select_parent.run(id);
                    }
                    on_close.run(());
                }
            >
                <span>"⬆️"</span> "Select Parent"
            </button>

            <button
                class="w-full text-left px-4 py-2 hover:bg-blue-50 text-sm text-gray-700 flex items-center gap-2 transition-colors"
                on:click=move |_| {
                    if let Some(id) = component_id.get() {
                        on_duplicate.run(id);
                    }
                    on_close.run(());
                }
            >
                <span>"📋"</span> "Duplicate"
            </button>

            <div class="h-px bg-gray-100 my-1"></div>

            <button
                class="w-full text-left px-4 py-2 hover:bg-red-50 text-sm text-red-600 flex items-center gap-2 transition-colors"
                on:click=move |_| {
                    if let Some(id) = component_id.get() {
                        on_delete.run(id);
                    }
                    on_close.run(());
                }
            >
                <span>"🗑️"</span> "Delete"
            </button>
        </div>

        // Invisible overlay to catch clicks outside
        <Show when=move || visible.get()>
            <div
                class="fixed inset-0 z-[9998]"
                on:click=move |_| on_close.run(())
                on:contextmenu=move |ev| {
                    ev.prevent_default();
                    on_close.run(());
                }
            ></div>
        </Show>
    }
}
