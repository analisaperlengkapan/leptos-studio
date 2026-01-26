use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, PartialEq)]
pub enum DragState {
    NotDragging,
    Dragging {
        component_type: String,
        ghost_x: f64,
        ghost_y: f64,
    },
    DraggingOver {
        component_type: String,
        drop_zone: String,
        ghost_x: f64,
        ghost_y: f64,
    },
}

#[component]
pub fn DragPreview(drag_state: RwSignal<DragState>) -> impl IntoView {
    view! {
        <Show when=move || !matches!(drag_state.get(), DragState::NotDragging)>
            <div
                class="drag-ghost"
                style=move || {
                    match drag_state.get() {
                        DragState::Dragging { ghost_x, ghost_y, .. } |
                        DragState::DraggingOver { ghost_x, ghost_y, .. } => {
                            format!("
                                position: fixed;
                                left: {}px;
                                top: {}px;
                                pointer-events: none;
                                z-index: 9999;
                                transform: translate(-50%, -50%);
                                background: rgba(59, 130, 246, 0.9);
                                border: 2px solid white;
                                border-radius: 8px;
                                padding: 8px 16px;
                                font-size: 14px;
                                color: white;
                                font-weight: 600;
                                box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.2), 0 4px 6px -4px rgba(0, 0, 0, 0.1);
                                display: flex;
                                align-items: center;
                                gap: 8px;
                            ", ghost_x, ghost_y)
                        }
                        _ => "display: none;".to_string()
                    }
                }
            >
                <span style="font-size: 18px;">"🖐️"</span>
                {move || {
                    match drag_state.get() {
                        DragState::Dragging { component_type, .. } |
                        DragState::DraggingOver { component_type, .. } => {
                            format!("Dragging {}", component_type)
                        }
                        _ => String::new()
                    }
                }}
            </div>
        </Show>
    }
}

#[derive(Clone, Copy)]
pub struct DragDropConfig {
    pub enable_ghost: bool,
    pub enable_drop_zones: bool,
    pub enable_auto_scroll: bool,
    pub scroll_threshold: f64,
    pub scroll_speed: f64,
}

impl Default for DragDropConfig {
    fn default() -> Self {
        Self {
            enable_ghost: true,
            enable_drop_zones: true,
            enable_auto_scroll: true,
            scroll_threshold: 50.0,
            scroll_speed: 10.0,
        }
    }
}

pub fn create_drag_handlers(
    component_type: String,
    drag_state: RwSignal<DragState>,
    config: DragDropConfig,
) -> (
    impl Fn(leptos::ev::DragEvent),
    impl Fn(leptos::ev::DragEvent),
    impl Fn(leptos::ev::DragEvent),
) {
    let component_type_start = component_type.clone();
    let component_type_drag = component_type.clone();
    let _component_type_end = component_type.clone();

    let on_drag_start = move |ev: leptos::ev::DragEvent| {
        let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();

        if let Some(dt) = drag_ev.data_transfer() {
            _ = dt.set_data("component", &component_type_start);
            dt.set_effect_allowed("copy");

            // Set custom drag image (invisible)
            if config.enable_ghost
                && let Some(document) = web_sys::window().and_then(|w| w.document())
                && let Ok(img) = document.create_element("div")
            {
                let img = img.unchecked_into::<web_sys::HtmlElement>();
                _ = img.style().set_property("width", "1px");
                _ = img.style().set_property("height", "1px");
                _ = img.style().set_property("background", "transparent");
                dt.set_drag_image(&img, 0, 0);
            }
        }

        let client_x = drag_ev.client_x() as f64;
        let client_y = drag_ev.client_y() as f64;

        drag_state.set(DragState::Dragging {
            component_type: component_type_start.clone(),
            ghost_x: client_x,
            ghost_y: client_y,
        });
    };

    let on_drag = move |ev: leptos::ev::DragEvent| {
        if config.enable_ghost {
            let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
            let client_x = drag_ev.client_x() as f64;
            let client_y = drag_ev.client_y() as f64;

            if client_x > 0.0 && client_y > 0.0 {
                if let DragState::Dragging { .. } = drag_state.get() {
                    drag_state.set(DragState::Dragging {
                        component_type: component_type_drag.clone(),
                        ghost_x: client_x,
                        ghost_y: client_y,
                    });
                } else if let DragState::DraggingOver { drop_zone, .. } = drag_state.get() {
                    drag_state.set(DragState::DraggingOver {
                        component_type: component_type_drag.clone(),
                        drop_zone,
                        ghost_x: client_x,
                        ghost_y: client_y,
                    });
                }
            }

            // Auto-scroll
            if config.enable_auto_scroll
                && let Some(window) = web_sys::window()
            {
                let _scroll_y = window.scroll_y().unwrap_or(0.0);
                let inner_height = window
                    .inner_height()
                    .unwrap_or(wasm_bindgen::JsValue::from(600.0))
                    .as_f64()
                    .unwrap_or(600.0);

                if client_y < config.scroll_threshold {
                    window.scroll_by_with_x_and_y(0.0, -config.scroll_speed);
                } else if client_y > inner_height - config.scroll_threshold {
                    window.scroll_by_with_x_and_y(0.0, config.scroll_speed);
                }
            }
        }
    };

    let on_drag_end = move |_ev: leptos::ev::DragEvent| {
        drag_state.set(DragState::NotDragging);
    };

    (on_drag_start, on_drag, on_drag_end)
}

pub fn create_drop_zone_handlers(
    zone_name: String,
    drag_state: RwSignal<DragState>,
    config: DragDropConfig,
) -> (
    impl Fn(leptos::ev::DragEvent),
    impl Fn(leptos::ev::DragEvent),
    impl Fn(leptos::ev::DragEvent),
) {
    let zone_enter = zone_name.clone();
    let _zone_leave = zone_name.clone();

    let on_drag_enter = move |ev: leptos::ev::DragEvent| {
        let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
        drag_ev.prevent_default();

        if let DragState::Dragging {
            component_type,
            ghost_x,
            ghost_y,
        } = drag_state.get()
            && config.enable_drop_zones
        {
            drag_state.set(DragState::DraggingOver {
                component_type,
                drop_zone: zone_enter.clone(),
                ghost_x,
                ghost_y,
            });
        }
    };

    let on_drag_over = move |ev: leptos::ev::DragEvent| {
        let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();
        drag_ev.prevent_default();
        _ = drag_ev.data_transfer().map(|dt| dt.set_drop_effect("copy"));
    };

    let on_drag_leave = move |ev: leptos::ev::DragEvent| {
        let drag_ev = ev.clone().unchecked_into::<web_sys::DragEvent>();

        // Only change state if we're actually leaving the drop zone
        if let Some(related_target) = drag_ev.related_target()
            && let Some(current_target) = drag_ev.current_target()
            && let (Ok(related_element), Ok(current_element)) = (
                related_target.dyn_into::<web_sys::Element>(),
                current_target.dyn_into::<web_sys::Element>(),
            )
            && !current_element.contains(Some(&related_element))
            && let DragState::DraggingOver {
                component_type,
                ghost_x,
                ghost_y,
                ..
            } = drag_state.get()
        {
            drag_state.set(DragState::Dragging {
                component_type,
                ghost_x,
                ghost_y,
            });
        }
    };

    (on_drag_enter, on_drag_over, on_drag_leave)
}

#[component]
pub fn DropZone<F>(
    zone_name: String,
    drag_state: RwSignal<DragState>,
    on_drop: F,
    config: Option<DragDropConfig>,
    children: Children,
) -> impl IntoView
where
    F: Fn(leptos::ev::DragEvent) + 'static,
{
    let config = config.unwrap_or_default();
    let (on_drag_enter, on_drag_over, on_drag_leave) =
        create_drop_zone_handlers(zone_name.clone(), drag_state, config);

    let is_drag_over = Memo::new(move |_| {
        matches!(
            drag_state.get(),
            DragState::DraggingOver { drop_zone, .. } if drop_zone == zone_name
        )
    });

    view! {
        <div
            class="drop-zone"
            class:drop-zone-active=is_drag_over
            style=move || {
                let base_style = "
                    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
                    position: relative;
                ";

                if is_drag_over.get() {
                    format!("{}
                        background: rgba(59, 130, 246, 0.1);
                        border: 2px dashed #3b82f6;
                        border-radius: 8px;
                        box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
                    ", base_style)
                } else {
                    base_style.to_string()
                }
            }
            on:dragenter=on_drag_enter
            on:dragover=on_drag_over
            on:dragleave=on_drag_leave
            on:drop=on_drop
        >
            {children()}

            <Show when=move || is_drag_over.get()>
                <div style="
                    position: absolute;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    background: rgba(9, 105, 218, 0.9);
                    color: white;
                    padding: 8px 16px;
                    border-radius: 20px;
                    font-size: 12px;
                    font-weight: 500;
                    pointer-events: none;
                    z-index: 100;
                    box-shadow: 0 2px 8px rgba(9, 105, 218, 0.3);
                ">
                    "Drop here"
                </div>
            </Show>
        </div>
    }
}
