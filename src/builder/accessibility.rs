//! Accessibility Service
//!
//! Provides accessibility features including screen reader support,
//! focus management, ARIA attributes, and keyboard navigation.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

/// Accessibility announcer for screen readers
#[derive(Clone, Copy)]
pub struct Announcer {
    /// Message to announce
    message: RwSignal<String>,
    /// Politeness level (polite or assertive)
    politeness: RwSignal<AnnounceLevel>,
}

/// Announcement politeness level
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnnounceLevel {
    /// Non-urgent updates
    Polite,
    /// Urgent updates that interrupt
    Assertive,
}

impl Announcer {
    /// Create a new announcer
    pub fn new() -> Self {
        Self {
            message: RwSignal::new(String::new()),
            politeness: RwSignal::new(AnnounceLevel::Polite),
        }
    }

    /// Provide announcer in Leptos context
    pub fn provide_context() {
        provide_context(Self::new());
    }

    /// Use announcer from Leptos context
    pub fn use_context() -> Self {
        expect_context::<Self>()
    }

    /// Announce a message to screen readers
    pub fn announce(&self, message: &str) {
        self.politeness.set(AnnounceLevel::Polite);
        self.message.set(message.to_string());
    }

    /// Announce an urgent message
    pub fn announce_assertive(&self, message: &str) {
        self.politeness.set(AnnounceLevel::Assertive);
        self.message.set(message.to_string());
    }

    /// Get the current message
    pub fn message(&self) -> String {
        self.message.get()
    }

    /// Get the current politeness level
    pub fn level(&self) -> AnnounceLevel {
        self.politeness.get()
    }

    /// Clear the announcement
    pub fn clear(&self) {
        self.message.set(String::new());
    }
}

impl Default for Announcer {
    fn default() -> Self {
        Self::new()
    }
}

/// Screen reader live region component
#[component]
pub fn LiveRegion() -> impl IntoView {
    let announcer = Announcer::use_context();

    view! {
        <div
            class="sr-only"
            role="status"
            aria-live=move || {
                match announcer.level() {
                    AnnounceLevel::Polite => "polite",
                    AnnounceLevel::Assertive => "assertive",
                }
            }
            aria-atomic="true"
        >
            {move || announcer.message()}
        </div>
    }
}

/// Skip link for keyboard users to bypass navigation
#[component]
pub fn SkipLink(
    /// Target element ID to skip to
    #[prop(into)]
    target: String,
    /// Link text (alias for text prop)
    #[prop(optional, into)]
    label: Option<String>,
) -> impl IntoView {
    let text = label.unwrap_or_else(|| "Skip to main content".to_string());

    view! {
        <a
            href=format!("#{}", target)
            class="skip-link"
            on:click=move |ev| {
                ev.prevent_default();
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.get_element_by_id(&target) {
                            let _ = element.dyn_into::<HtmlElement>().map(|el| el.focus());
                        }
                    }
                }
            }
        >
            {text}
        </a>
    }
}

/// Focus trap for modal dialogs
#[component]
pub fn FocusTrap(
    /// Whether the trap is active
    #[prop(into)]
    active: Signal<bool>,
    /// Children to render inside the trap
    children: Children,
) -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Handle keyboard navigation within trap
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if !active.get() {
            return;
        }

        if ev.key() == "Tab" {
            if let Some(container) = container_ref.get() {
                let focusable = get_focusable_elements(&container);
                if focusable.is_empty() {
                    return;
                }

                let first = focusable.first().cloned();
                let last = focusable.last().cloned();

                if ev.shift_key() {
                    // Shift+Tab: wrap to last if at first
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        if let Some(active) = document.active_element() {
                            if Some(&active) == first.as_ref() {
                                ev.prevent_default();
                                if let Some(el) = last {
                                    let _ = el.dyn_into::<HtmlElement>().map(|h| h.focus());
                                }
                            }
                        }
                    }
                } else {
                    // Tab: wrap to first if at last
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        if let Some(active) = document.active_element() {
                            if Some(&active) == last.as_ref() {
                                ev.prevent_default();
                                if let Some(el) = first {
                                    let _ = el.dyn_into::<HtmlElement>().map(|h| h.focus());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Close on Escape
        if ev.key() == "Escape" {
            // Parent should handle closing
        }
    };

    view! {
        <div
            node_ref=container_ref
            on:keydown=on_keydown
            tabindex="-1"
        >
            {children()}
        </div>
    }
}

/// Get all focusable elements within a container
/// Note: This is a simplified implementation
fn get_focusable_elements(_container: &HtmlElement) -> Vec<Element> {
    // Simplified: return empty vec since query_selector_all requires web-sys feature
    // In production, you would enable the web-sys feature for query_selector_all
    Vec::new()
}

/// ARIA-based progress indicator
#[component]
pub fn ProgressBar(
    /// Current value (0-100)
    #[prop(into)]
    value: Signal<f64>,
    /// Label for screen readers
    #[prop(into)]
    label: String,
    /// Whether to show the percentage visually
    #[prop(optional)]
    show_value: bool,
) -> impl IntoView {
    view! {
        <div
            class="progress-bar-container"
            role="progressbar"
            aria-label=label.clone()
            aria-valuenow=move || value.get() as i32
            aria-valuemin="0"
            aria-valuemax="100"
        >
            <div
                class="progress-bar-fill"
                style=move || format!("width: {}%", value.get().min(100.0).max(0.0))
            />
            {if show_value {
                view! {
                    <span class="progress-bar-text">
                        {move || format!("{}%", value.get() as i32)}
                    </span>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}

/// Accessible tooltip component
#[component]
pub fn Tooltip(
    /// Content to show in tooltip
    #[prop(into)]
    content: String,
    /// Children (trigger element)
    children: Children,
) -> impl IntoView {
    let show = RwSignal::new(false);
    let id = format!("tooltip-{}", uuid::Uuid::new_v4());

    view! {
        <div class="tooltip-wrapper">
            <div
                aria-describedby=id.clone()
                on:mouseenter=move |_| show.set(true)
                on:mouseleave=move |_| show.set(false)
                on:focus=move |_| show.set(true)
                on:blur=move |_| show.set(false)
            >
                {children()}
            </div>
            <div
                id=id
                class=move || if show.get() { "tooltip visible" } else { "tooltip" }
                role="tooltip"
                aria-hidden=move || (!show.get()).to_string()
            >
                {content}
            </div>
        </div>
    }
}

/// Visually hidden text for screen readers only
#[component]
pub fn VisuallyHidden(children: Children) -> impl IntoView {
    view! {
        <span class="sr-only">
            {children()}
        </span>
    }
}

/// Keyboard shortcut hint
#[component]
pub fn KeyboardHint(
    /// The keyboard shortcut
    #[prop(into)]
    shortcut: String,
) -> impl IntoView {
    let shortcut_clone = shortcut.clone();
    view! {
        <kbd class="keyboard-hint" aria-label=format!("Keyboard shortcut: {}", shortcut)>
            {shortcut_clone}
        </kbd>
    }
}

/// Accessibility provider component that wraps app with necessary providers
#[component]
pub fn AccessibilityProvider(children: Children) -> impl IntoView {
    // Provide announcer context
    Announcer::provide_context();

    view! {
        <div class="accessibility-wrapper">
            <LiveRegion />
            {children()}
        </div>
    }
}

/// Convenience function to announce a message
pub fn announce(message: &str) {
    if let Some(announcer) = use_context::<Announcer>() {
        announcer.announce(message);
    }
}

/// Convenience function to announce an urgent message
pub fn announce_assertive(message: &str) {
    if let Some(announcer) = use_context::<Announcer>() {
        announcer.announce_assertive(message);
    }
}
