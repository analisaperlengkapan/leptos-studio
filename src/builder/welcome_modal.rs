use leptos::prelude::*;

#[component]
pub fn WelcomeModal(
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="welcome-modal-overlay">
            <div class="welcome-modal-content">
                <div class="welcome-header">
                    <h2>"Welcome to Leptos Studio! 🚀"</h2>
                </div>
                <div class="welcome-body">
                    <p>"Leptos Studio is your visual builder for the Leptos web framework."</p>

                    <div class="welcome-features">
                        <div class="feature-item">
                            <span class="feature-icon">"👈"</span>
                            <div class="feature-text">
                                <strong>"Left Panel"</strong>
                                <p>"Drag and drop components from the library."</p>
                            </div>
                        </div>
                        <div class="feature-item">
                            <span class="feature-icon">"🎨"</span>
                            <div class="feature-text">
                                <strong>"Center Canvas"</strong>
                                <p>"Visually arrange your layout."</p>
                            </div>
                        </div>
                        <div class="feature-item">
                            <span class="feature-icon">"👉"</span>
                            <div class="feature-text">
                                <strong>"Right Panel"</strong>
                                <p>"Edit properties, view history, and export code."</p>
                            </div>
                        </div>
                    </div>

                    <div class="welcome-tips">
                        <p><strong>"Pro Tip:"</strong> "Use " <code>"Ctrl+Z"</code> " to undo mistakes!"</p>
                    </div>
                </div>
                <div class="welcome-footer">
                    <button class="btn btn-primary btn-lg" on:click=move |_| on_close.run(())>
                        "Start Building"
                    </button>
                </div>
            </div>
        </div>
    }
}
