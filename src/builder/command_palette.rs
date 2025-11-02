//! Command Palette Component
//!
//! VS Code-style command palette for quick access to all application commands.
//! Features fuzzy search, keyboard navigation, and command execution.

use crate::builder::keyboard::KeyboardAction;
use leptos::*;

/// Represents a single command in the command palette
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub id: String,
    pub title: String,
    pub category: String,
    pub action: KeyboardAction,
}

impl Command {
    /// Create a new command
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the command
    /// * `title` - Display title shown in palette
    /// * `category` - Category for grouping (e.g., "Edit", "File")
    /// * `action` - The keyboard action to execute
    pub fn new(id: &str, title: &str, category: &str, action: KeyboardAction) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            category: category.to_string(),
            action,
        }
    }
}

/// Fuzzy match algorithm for searching commands
///
/// Returns a score if the pattern matches the text, with higher scores
/// for consecutive character matches.
///
/// # Arguments
/// * `text` - The text to search in
/// * `pattern` - The search pattern
///
/// # Returns
/// * `Some(score)` if pattern matches, `None` otherwise
fn fuzzy_match(text: &str, pattern: &str) -> Option<i32> {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    let mut pattern_idx = 0;
    let mut score = 0;
    let mut consecutive_matches = 0;

    for &text_char in text_chars.iter() {
        if pattern_idx < pattern_chars.len() && text_char == pattern_chars[pattern_idx] {
            pattern_idx += 1;
            consecutive_matches += 1;
            score += consecutive_matches * 10;

            if pattern_idx == pattern_chars.len() {
                return Some(score);
            }
        } else {
            consecutive_matches = 0;
        }
    }

    None
}

/// Get all available commands
///
/// Returns the complete list of commands available in the command palette,
/// organized by category (Edit, File, Components, Selection).
fn get_commands() -> Vec<Command> {
    vec![
        // Edit commands
        Command::new("undo", "Undo", "Edit", KeyboardAction::Undo),
        Command::new("redo", "Redo", "Edit", KeyboardAction::Redo),
        // Delete
        Command::new("delete", "Delete Selected", "Edit", KeyboardAction::Delete),
        // Copy/Paste
        Command::new("copy", "Copy", "Edit", KeyboardAction::Copy),
        // Paste
        Command::new("paste", "Paste", "Edit", KeyboardAction::Paste),
        // Selection
        Command::new(
            "select_all",
            "Select All",
            "Selection",
            KeyboardAction::SelectAll,
        ),
        // Deselect
        Command::new(
            "deselect",
            "Deselect All",
            "Selection",
            KeyboardAction::Deselect,
        ),
        // File operations
        Command::new("save", "Save Project", "File", KeyboardAction::Save),
        // Export
        Command::new("export", "Export Code", "File", KeyboardAction::Export),
        // Components
        Command::new(
            "new_component",
            "Add Component",
            "Components",
            KeyboardAction::NewComponent,
        ),
    ]
}

/// VS Code-style Command Palette Component
///
/// A modal overlay that provides quick access to all application commands
/// through fuzzy search and keyboard navigation.
///
/// # Features
/// * Fuzzy search across command titles and categories
/// * Keyboard navigation (Arrow Up/Down, Enter, Escape)
/// * Mouse hover selection
/// * Command execution through callback
///
/// # Keyboard Shortcuts
/// * `↑/↓` - Navigate commands
/// * `Enter` - Execute selected command
/// * `Escape` - Close palette
///
/// # Props
/// * `is_open` - Read signal controlling visibility
/// * `close` - Write signal to close the palette
/// * `search` - RwSignal for search input
/// * `on_action` - Callback executed when a command is selected
#[component]
pub fn CommandPalette<F>(
    is_open: ReadSignal<bool>,
    close: WriteSignal<bool>,
    #[prop(into)] search: RwSignal<String>,
    on_action: F,
) -> impl IntoView
where
    F: Fn(KeyboardAction) + 'static + Clone,
{
    let (filtered_commands, set_filtered_commands) = create_signal(get_commands());
    let (selected_index, set_selected_index) = create_signal(0);

    // Update filtered commands when search changes
    create_effect(move |_| {
        let search_term = search.get().to_lowercase();
        let commands = get_commands();
        let filtered = if search_term.is_empty() {
            commands
        } else {
            commands
                .into_iter()
                .filter_map(|cmd| {
                    let score = fuzzy_match(&cmd.title.to_lowercase(), &search_term)
                        .or_else(|| fuzzy_match(&cmd.category.to_lowercase(), &search_term))?;
                    Some((cmd, score))
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|(cmd, _)| cmd)
                .collect()
        };
        set_filtered_commands.set(filtered);
        set_selected_index.set(0);
    });

    // Handle keyboard navigation with stored action
    let stored_on_action = store_value(on_action);

    view! {
        <Show when=move || is_open.get()>
            <div
                class="command-palette-backdrop"
                style="
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100vw;
                    height: 100vh;
                    background: rgba(0, 0, 0, 0.5);
                    z-index: 1000;
                    display: flex;
                    align-items: flex-start;
                    justify-content: center;
                    padding-top: 20vh;
                "
                on:click=move |_| close.set(false)
            >
                <div
                    class="command-palette"
                    style="
                        background: var(--color-surface);
                        border: 1px solid var(--color-border);
                        border-radius: 8px;
                        box-shadow: 0 16px 32px rgba(0, 0, 0, 0.24);
                        width: min(640px, 90vw);
                        max-height: 60vh;
                        overflow: hidden;
                        display: flex;
                        flex-direction: column;
                    "
                    on:click=move |ev| ev.stop_propagation()
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                        let key = ev.key();
                        match key.as_str() {
                            "ArrowUp" => {
                                ev.prevent_default();
                                set_selected_index.update(|idx| {
                                    let len = filtered_commands.get().len();
                                    *idx = if *idx > 0 { *idx - 1 } else { len.saturating_sub(1) };
                                });
                            }
                            "ArrowDown" => {
                                ev.prevent_default();
                                set_selected_index.update(|idx| {
                                    let len = filtered_commands.get().len();
                                    *idx = (*idx + 1) % len;
                                });
                            }
                            "Enter" => {
                                ev.prevent_default();
                                let commands = filtered_commands.get();
                                if let Some(command) = commands.get(selected_index.get()) {
                                    stored_on_action.with_value(|action| action(command.action.clone()));
                                    close.set(false);
                                }
                            }
                            "Escape" => {
                                ev.prevent_default();
                                close.set(false);
                            }
                            _ => {}
                        }
                    }
                >
                    <div class="command-palette-search" style="
                        padding: 16px;
                        border-bottom: 1px solid var(--color-border);
                    ">
                        <input
                            type="text"
                            placeholder="Search commands..."
                            prop:value=move || search.get()
                            on:input=move |ev| {
                                search.set(event_target_value(&ev));
                            }
                            style="
                                width: 100%;
                                padding: 12px;
                                border: 1px solid var(--color-border);
                                border-radius: 6px;
                                background: var(--color-background);
                                color: var(--color-text);
                                font-size: 16px;
                                outline: none;
                            "
                        />
                    </div>

                    <div class="command-palette-results" style="
                        max-height: 400px;
                        overflow-y: auto;
                        padding: 8px 0;
                    ">
                        <For
                            each=move || filtered_commands.get().into_iter().enumerate()
                            key=|(idx, cmd)| format!("{}-{}", idx, cmd.id)
                            children=move |(idx, command)| {
                                let is_selected = move || selected_index.get() == idx;
                                let command_clone = command.clone();

                                view! {
                                    <div
                                        class="command-palette-item"
                                        class:selected=is_selected
                                        style=move || format!(
                                            "
                                            padding: 12px 16px;
                                            cursor: pointer;
                                            border-left: 3px solid {};
                                            background: {};
                                            color: var(--color-text);
                                            display: flex;
                                            justify-content: space-between;
                                            align-items: center;
                                            ",
                                            if is_selected() { "var(--color-primary)" } else { "transparent" },
                                            if is_selected() { "var(--color-primary-subtle)" } else { "transparent" }
                                        )
                                        on:click={
                                            let command = command_clone.clone();
                                            move |_| {
                                                stored_on_action.with_value(|action| action(command.action.clone()));
                                                close.set(false);
                                            }
                                        }
                                        on:mouseenter=move |_| set_selected_index.set(idx)
                                    >
                                        <div>
                                            <div style="font-weight: 500; margin-bottom: 2px;">
                                                {command.title}
                                            </div>
                                            <div style="
                                                font-size: 12px; 
                                                color: var(--color-text-secondary);
                                            ">
                                                {command.category}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }
                        />

                        <Show when=move || filtered_commands.get().is_empty()>
                            <div style="
                                padding: 24px;
                                text-align: center;
                                color: var(--color-text-secondary);
                                font-style: italic;
                            ">
                                No commands found
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </Show>
    }
}
