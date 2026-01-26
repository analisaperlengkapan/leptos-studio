//! Event Bus Service
//!
//! Provides a centralized event system for decoupled communication
//! between components and services. Follows the Observer pattern
//! for reactive updates across the application.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Application events that can be dispatched through the event bus
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppEvent {
    // Component events
    ComponentAdded {
        component_type: String,
    },
    ComponentRemoved {
        component_id: String,
    },
    ComponentUpdated {
        component_id: String,
    },
    ComponentSelected {
        component_id: Option<String>,
    },

    // Canvas events
    CanvasCleared,
    CanvasLoaded,
    CanvasSnapshot,

    // Project events
    ProjectSaved {
        name: String,
    },
    ProjectLoaded {
        name: String,
    },
    ProjectExported {
        format: String,
    },

    // UI events
    ThemeChanged {
        theme: String,
    },
    ResponsiveModeChanged {
        mode: String,
    },
    PanelToggled {
        panel: String,
        visible: bool,
    },

    // History events
    HistoryUndo,
    HistoryRedo,
    HistoryCleared,

    // Notification events
    NotificationShown {
        message: String,
        level: NotificationLevel,
    },

    // Custom events for extensibility
    Custom {
        name: String,
        data: String,
    },
}

/// Notification severity levels
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl fmt::Display for AppEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEvent::ComponentAdded { component_type } => {
                write!(f, "Component added: {}", component_type)
            }
            AppEvent::ComponentRemoved { component_id } => {
                write!(f, "Component removed: {}", component_id)
            }
            AppEvent::ComponentUpdated { component_id } => {
                write!(f, "Component updated: {}", component_id)
            }
            AppEvent::ComponentSelected { component_id } => {
                write!(f, "Component selected: {:?}", component_id)
            }
            AppEvent::CanvasCleared => write!(f, "Canvas cleared"),
            AppEvent::CanvasLoaded => write!(f, "Canvas loaded"),
            AppEvent::CanvasSnapshot => write!(f, "Canvas snapshot created"),
            AppEvent::ProjectSaved { name } => write!(f, "Project saved: {}", name),
            AppEvent::ProjectLoaded { name } => write!(f, "Project loaded: {}", name),
            AppEvent::ProjectExported { format } => write!(f, "Project exported: {}", format),
            AppEvent::ThemeChanged { theme } => write!(f, "Theme changed: {}", theme),
            AppEvent::ResponsiveModeChanged { mode } => {
                write!(f, "Responsive mode changed: {}", mode)
            }
            AppEvent::PanelToggled { panel, visible } => {
                write!(
                    f,
                    "Panel {}: {}",
                    panel,
                    if *visible { "shown" } else { "hidden" }
                )
            }
            AppEvent::HistoryUndo => write!(f, "History: Undo"),
            AppEvent::HistoryRedo => write!(f, "History: Redo"),
            AppEvent::HistoryCleared => write!(f, "History cleared"),
            AppEvent::NotificationShown { message, level } => {
                write!(f, "Notification ({:?}): {}", level, message)
            }
            AppEvent::Custom { name, data } => write!(f, "Custom event {}: {}", name, data),
        }
    }
}

/// Event bus for application-wide event dispatching
#[derive(Clone, Copy)]
pub struct EventBus {
    /// Recent events log (limited to last N events for debugging)
    events: RwSignal<Vec<AppEvent>>,
    /// Current event being dispatched
    current_event: RwSignal<Option<AppEvent>>,
    /// Event counter for debugging
    event_count: RwSignal<u32>,
}

impl EventBus {
    /// Maximum number of events to keep in history
    const MAX_EVENT_HISTORY: usize = 100;

    /// Create a new event bus instance
    pub fn new() -> Self {
        Self {
            events: RwSignal::new(Vec::new()),
            current_event: RwSignal::new(None),
            event_count: RwSignal::new(0),
        }
    }

    /// Provide event bus in Leptos context
    pub fn provide_context() {
        provide_context(Self::new());
    }

    /// Use event bus from Leptos context
    pub fn use_context() -> Self {
        expect_context::<Self>()
    }

    /// Dispatch an event to all listeners
    pub fn dispatch(&self, event: AppEvent) {
        // Update current event
        self.current_event.set(Some(event.clone()));

        // Add to history with size limit
        self.events.update(|events| {
            events.push(event);
            if events.len() > Self::MAX_EVENT_HISTORY {
                events.remove(0);
            }
        });

        // Increment counter
        self.event_count.update(|c| *c += 1);

        // Log for debugging in development
        #[cfg(debug_assertions)]
        web_sys::console::log_1(&format!("📢 Event: {}", self.current_event.get().unwrap()).into());
    }

    /// Get the current/latest event
    pub fn current(&self) -> Option<AppEvent> {
        self.current_event.get()
    }

    /// Get all events in history
    pub fn history(&self) -> Vec<AppEvent> {
        self.events.get()
    }

    /// Get event count
    pub fn count(&self) -> u32 {
        self.event_count.get()
    }

    /// Clear event history
    pub fn clear_history(&self) {
        self.events.set(Vec::new());
    }

    /// Get current event signal for reactive subscriptions
    pub fn current_signal(&self) -> ReadSignal<Option<AppEvent>> {
        self.current_event.read_only()
    }

    /// Check if last event matches a specific type
    pub fn is_component_event(&self) -> bool {
        self.current().is_some_and(|e| {
            matches!(
                e,
                AppEvent::ComponentAdded { .. }
                    | AppEvent::ComponentRemoved { .. }
                    | AppEvent::ComponentUpdated { .. }
                    | AppEvent::ComponentSelected { .. }
            )
        })
    }

    /// Check if last event is a canvas event
    pub fn is_canvas_event(&self) -> bool {
        self.current().is_some_and(|e| {
            matches!(
                e,
                AppEvent::CanvasCleared | AppEvent::CanvasLoaded | AppEvent::CanvasSnapshot
            )
        })
    }

    /// Check if last event is a project event
    pub fn is_project_event(&self) -> bool {
        self.current().is_some_and(|e| {
            matches!(
                e,
                AppEvent::ProjectSaved { .. }
                    | AppEvent::ProjectLoaded { .. }
                    | AppEvent::ProjectExported { .. }
            )
        })
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// Integration tests that require reactive runtime (WASM only)
#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_event_dispatch() {
        let bus = EventBus::new();
        assert!(bus.current().is_none());
        assert_eq!(bus.count(), 0);

        bus.dispatch(AppEvent::CanvasCleared);
        assert_eq!(bus.current(), Some(AppEvent::CanvasCleared));
        assert_eq!(bus.count(), 1);
    }

    #[wasm_bindgen_test]
    fn test_event_history() {
        let bus = EventBus::new();

        bus.dispatch(AppEvent::CanvasCleared);
        bus.dispatch(AppEvent::ProjectSaved {
            name: "test".to_string(),
        });
        bus.dispatch(AppEvent::HistoryUndo);

        let history = bus.history();
        assert_eq!(history.len(), 3);
    }

    #[wasm_bindgen_test]
    fn test_history_limit() {
        let bus = EventBus::new();

        // Dispatch more than MAX_EVENT_HISTORY events
        for i in 0..150 {
            bus.dispatch(AppEvent::Custom {
                name: format!("event_{}", i),
                data: "".to_string(),
            });
        }

        assert!(bus.history().len() <= EventBus::MAX_EVENT_HISTORY);
    }
}
