use crate::constants::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH};
use leptos::ev;
use leptos::prelude::*;

/// Controls for a resizable sidebar
#[derive(Clone)]
pub struct ResizeControls {
    /// The current width of the sidebar
    pub width: RwSignal<i32>,
    /// Whether the sidebar is currently being dragged
    pub is_dragging: RwSignal<bool>,
    /// Handler to start dragging (attach to on:mousedown)
    pub start_drag: Callback<ev::MouseEvent>,
}

/// Hook to manage a resizable sidebar with persistence
///
/// # Arguments
/// * `initial_width` - The default width if no stored value exists
/// * `storage_key` - LocalStorage key to persist the width
/// * `is_left` - True if this is the left sidebar (grows with mouse x), False for right (grows with window width - mouse x)
pub fn use_resizable_sidebar(
    initial_width: i32,
    storage_key: &'static str,
    is_left: bool,
) -> ResizeControls {
    // Load from storage or use default
    let stored_width = if let Ok(Some(storage)) = window().local_storage() {
        if let Ok(Some(w_str)) = storage.get_item(storage_key) {
            w_str.parse::<i32>().unwrap_or(initial_width)
        } else {
            initial_width
        }
    } else {
        initial_width
    };

    let width = RwSignal::new(stored_width);
    let is_dragging = RwSignal::new(false);

    // Save to storage on width change
    Effect::new(move |_| {
        let w = width.get();
        if let Ok(Some(storage)) = window().local_storage() {
            let _ = storage.set_item(storage_key, &w.to_string());
        }
    });

    // Handle mouse move
    let _ = window_event_listener(ev::mousemove, move |ev| {
        if is_dragging.get() {
            let new_width = if is_left {
                ev.client_x()
            } else {
                let window_width = window().inner_width().unwrap().as_f64().unwrap() as i32;
                window_width - ev.client_x()
            };

            let clamped = new_width.max(MIN_SIDEBAR_WIDTH).min(MAX_SIDEBAR_WIDTH);
            width.set(clamped);
        }
    });

    // Handle mouse up
    let _ = window_event_listener(ev::mouseup, move |_| {
        if is_dragging.get() {
            is_dragging.set(false);
        }
    });

    ResizeControls {
        width,
        is_dragging,
        start_drag: Callback::new(move |_| is_dragging.set(true)),
    }
}
