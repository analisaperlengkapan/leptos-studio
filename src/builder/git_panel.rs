use leptos::prelude::*;

#[component]
pub fn GitPanel() -> impl IntoView {
    view! {
        <div class="git-panel-content">
            <div class="git-status">
                <p style="color: #64748b; font-size: 13px; line-height: 1.6;">
                    "🔧 Git integration requires a backend server."
                </p>
                <p style="color: #94a3b8; font-size: 12px; margin-top: 8px;">
                    "This is a client-side app. Git features are available when running with a server backend."
                </p>
            </div>
            <div class="git-placeholder" style="margin-top: 12px; padding: 12px; background: #f1f5f9; border-radius: 6px; border-left: 3px solid #3b82f6;">
                <p style="font-size: 12px; color: #475569; margin-bottom: 8px;">
                    <strong>"💡 Tip:"</strong> " Git features will work when:"
                </p>
                <ul style="font-size: 12px; color: #64748b; margin-left: 20px;">
                    <li>"Running with Actix/Axum backend"</li>
                    <li>"Using Tauri desktop app"</li>
                    <li>"Deploying with server-side rendering"</li>
                </ul>
            </div>
        </div>
    }
}
