use leptos::*;
use leptos::ev::KeyboardEvent;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Debug, PartialEq)]
pub enum KeyboardAction {
    Undo,
    Redo,
    Delete,
    Copy,
    Paste,
    SelectAll,
    Deselect,
    OpenCommandPalette,
    Save,
    Export,
    NewComponent,
    Duplicate,
}

#[derive(Clone, Debug)]
pub struct KeyboardShortcut {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
    pub action: KeyboardAction,
    pub description: String,
}

impl KeyboardShortcut {
    pub fn new(key: &str, ctrl: bool, shift: bool, alt: bool, meta: bool, action: KeyboardAction, description: &str) -> Self {
        Self {
            key: key.to_string(),
            ctrl,
            shift,
            alt,
            meta,
            action,
            description: description.to_string(),
        }
    }

    pub fn matches(&self, event: &KeyboardEvent) -> bool {
        let key_match = self.key.to_lowercase() == event.key().to_lowercase() || 
                       self.key.to_lowercase() == event.code().to_lowercase();
        
        key_match && 
        self.ctrl == event.ctrl_key() &&
        self.shift == event.shift_key() &&
        self.alt == event.alt_key() &&
        self.meta == event.meta_key()
    }

    pub fn display_string(&self) -> String {
        let mut parts = Vec::new();
        
        if self.meta {
            parts.push("⌘".to_string());
        }
        if self.ctrl {
            parts.push("Ctrl".to_string());
        }
        if self.shift {
            parts.push("⇧".to_string());
        }
        if self.alt {
            parts.push("Alt".to_string());
        }
        
        let key_upper = self.key.to_uppercase();
        parts.push(key_upper);
        parts.join(" + ")
    }
}

pub fn get_default_shortcuts() -> Vec<KeyboardShortcut> {
    vec![
        KeyboardShortcut::new("z", true, false, false, false, KeyboardAction::Undo, "Undo last action"),
        KeyboardShortcut::new("z", true, true, false, false, KeyboardAction::Redo, "Redo last action"),
        KeyboardShortcut::new("y", true, false, false, false, KeyboardAction::Redo, "Redo last action"),
        KeyboardShortcut::new("Delete", false, false, false, false, KeyboardAction::Delete, "Delete selected component"),
        KeyboardShortcut::new("Backspace", false, false, false, false, KeyboardAction::Delete, "Delete selected component"),
        KeyboardShortcut::new("c", true, false, false, false, KeyboardAction::Copy, "Copy selected component"),
        KeyboardShortcut::new("v", true, false, false, false, KeyboardAction::Paste, "Paste component"),
        KeyboardShortcut::new("a", true, false, false, false, KeyboardAction::SelectAll, "Select all components"),
        KeyboardShortcut::new("Escape", false, false, false, false, KeyboardAction::Deselect, "Deselect all"),
        KeyboardShortcut::new("k", true, false, false, false, KeyboardAction::OpenCommandPalette, "Open command palette"),
        KeyboardShortcut::new("s", true, false, false, false, KeyboardAction::Save, "Save project"),
        KeyboardShortcut::new("e", true, false, false, false, KeyboardAction::Export, "Export code"),
        KeyboardShortcut::new("n", true, false, false, false, KeyboardAction::NewComponent, "New component"),
        KeyboardShortcut::new("d", true, false, false, false, KeyboardAction::Duplicate, "Duplicate selected"),
    ]
}

#[component]
pub fn KeyboardHandler<F>(
    shortcuts: Vec<KeyboardShortcut>,
    on_action: F,
) -> impl IntoView 
where 
    F: Fn(KeyboardAction) + 'static + Clone,
{
    let on_keydown = {
        let shortcuts = shortcuts.clone();
        let on_action = on_action.clone();
        move |ev: KeyboardEvent| {
            // Don't handle shortcuts when typing in inputs
            if let Some(target) = ev.target() {
                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                    let tag_name = element.tag_name().to_lowercase();
                    if tag_name == "input" || tag_name == "textarea" || tag_name == "select" {
                        return;
                    }
                    if element.is_content_editable() {
                        return;
                    }
                }
            }

            for shortcut in &shortcuts {
                if shortcut.matches(&ev) {
                    ev.prevent_default();
                    ev.stop_propagation();
                    on_action(shortcut.action.clone());
                    break;
                }
            }
        }
    };

    view! {
        <div 
            style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: -1;"
            on:keydown=on_keydown
            tabindex="-1"
        />
    }
}
