use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::constants::MAX_HISTORY_SIZE;
use crate::domain::{CanvasComponent, ComponentId};

/// Snapshot of canvas state at a point in time
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub components: Vec<CanvasComponent>,
    pub selected: Option<ComponentId>,
    pub timestamp: f64,
    pub description: String,
}

impl Snapshot {
    pub fn new(
        components: Vec<CanvasComponent>,
        selected: Option<ComponentId>,
        description: String,
    ) -> Self {
        let timestamp = get_timestamp();
        Self {
            components,
            selected,
            timestamp,
            description,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_timestamp() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_timestamp() -> f64 {
    0.0 // Mock timestamp for tests
}

/// History system for undo/redo functionality
#[derive(Clone, Debug)]
pub struct History {
    undo_stack: VecDeque<Snapshot>,
    redo_stack: VecDeque<Snapshot>,
}

impl History {
    pub fn get_undo_stack(&self) -> Vec<Snapshot> {
        self.undo_stack.iter().rev().cloned().collect()
    }

    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    /// Push a new snapshot to the history
    pub fn push(&mut self, snapshot: Snapshot) {
        // Clear redo stack when new action is performed
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push_back(snapshot);

        // Limit stack size
        if self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.pop_front();
        }
    }

    /// Undo the last action
    pub fn undo(&mut self) -> Option<Snapshot> {
        if let Some(snapshot) = self.undo_stack.pop_back() {
            self.redo_stack.push_back(snapshot.clone());
            // Return the previous state
            self.undo_stack.back().cloned()
        } else {
            None
        }
    }

    /// Redo the last undone action
    pub fn redo(&mut self) -> Option<Snapshot> {
        if let Some(snapshot) = self.redo_stack.pop_back() {
            self.undo_stack.push_back(snapshot.clone());
            Some(snapshot)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.undo_stack.len() > 1 // Need at least 2 items to undo
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Restore to a specific index in the undo stack
    /// Index 0 is the oldest action.
    /// This function moves newer actions to the redo stack.
    pub fn restore_to_index(&mut self, index: usize) -> Option<Snapshot> {
        if index >= self.undo_stack.len() {
            return None;
        }

        // We want to keep elements up to `index` (inclusive) in the undo stack.
        // Elements after `index` should be moved to redo stack in reverse order.
        let keep_count = index + 1;
        let remove_count = self.undo_stack.len() - keep_count;

        for _ in 0..remove_count {
            if let Some(snapshot) = self.undo_stack.pop_back() {
                self.redo_stack.push_back(snapshot);
            }
        }

        // Return the snapshot at the new tip of undo stack
        self.undo_stack.back().cloned()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ButtonComponent, CanvasComponent};

    fn create_test_snapshot(label: &str) -> Snapshot {
        let button = ButtonComponent::new(label.to_string());
        let button_id = button.id;
        let component = CanvasComponent::Button(button.clone());
        Snapshot::new(
            vec![component],
            Some(button_id),
            format!("Update {}", label),
        )
    }

    #[test]
    fn test_history_push() {
        let mut history = History::new();
        let snapshot1 = create_test_snapshot("Button 1");
        let snapshot2 = create_test_snapshot("Button 2");

        history.push(snapshot1);
        history.push(snapshot2);

        assert_eq!(history.undo_stack.len(), 2);
        assert_eq!(history.redo_stack.len(), 0);
    }

    #[test]
    fn test_history_max_size() {
        let mut history = History::new();

        // Push more than MAX_HISTORY_SIZE snapshots
        for i in 0..MAX_HISTORY_SIZE + 10 {
            let snapshot = create_test_snapshot(&format!("Button {}", i));
            history.push(snapshot);
        }

        assert_eq!(history.undo_stack.len(), MAX_HISTORY_SIZE);
    }

    #[test]
    fn test_history_undo() {
        let mut history = History::new();
        let snapshot1 = create_test_snapshot("Button 1");
        let snapshot2 = create_test_snapshot("Button 2");

        history.push(snapshot1.clone());
        history.push(snapshot2);

        let undone = history.undo();
        assert!(undone.is_some());
        assert_eq!(history.undo_stack.len(), 1);
        assert_eq!(history.redo_stack.len(), 1);
    }

    #[test]
    fn test_history_redo() {
        let mut history = History::new();
        let snapshot1 = create_test_snapshot("Button 1");
        let snapshot2 = create_test_snapshot("Button 2");

        history.push(snapshot1);
        history.push(snapshot2.clone());
        history.undo();

        let redone = history.redo();
        assert!(redone.is_some());
        assert_eq!(history.undo_stack.len(), 2);
        assert_eq!(history.redo_stack.len(), 0);
    }

    #[test]
    fn test_history_can_undo() {
        let mut history = History::new();
        assert!(!history.can_undo());

        history.push(create_test_snapshot("Button 1"));
        assert!(!history.can_undo()); // Need at least 2 items

        history.push(create_test_snapshot("Button 2"));
        assert!(history.can_undo());
    }

    #[test]
    fn test_history_can_redo() {
        let mut history = History::new();
        assert!(!history.can_redo());

        history.push(create_test_snapshot("Button 1"));
        history.push(create_test_snapshot("Button 2"));
        assert!(!history.can_redo());

        history.undo();
        assert!(history.can_redo());
    }

    #[test]
    fn test_history_clear_on_new_action() {
        let mut history = History::new();

        history.push(create_test_snapshot("Button 1"));
        history.push(create_test_snapshot("Button 2"));
        history.undo();

        assert!(history.can_redo());

        // New action should clear redo stack
        history.push(create_test_snapshot("Button 3"));
        assert!(!history.can_redo());
    }
}
