use crate::builder::property_inputs::NumberInput;
use crate::domain::{CanvasComponent, ComponentId, PropValue, LayoutType, FlexDirection, FlexAlign, FlexJustify};
use crate::services::update_container_prop;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn ContainerPropertyEditor(
    id: ComponentId,
    #[prop(into)] container: crate::domain::ContainerComponent,
) -> impl IntoView {
    let app_state = AppState::expect_context();
    let ui_state = app_state.ui;
    let canvas_state = app_state.canvas;

    let comp_id = id;

    // Helper to apply updates
    let apply_update = move |id: ComponentId, updated: CanvasComponent, prop_name: String| {
        if let Err(e) = updated.validate() {
            ui_state.notify(crate::state::Notification::error(e.user_message()));
        } else {
            canvas_state.update_component_with_snapshot(
                &id,
                updated,
                &format!("Update Container {}", prop_name),
            );
        }
    };

    // Helper to switch layout type
    let switch_layout = {
        let container = container.clone();
        let apply_update = apply_update.clone();
        let comp_id = comp_id;
        move |new_type: &str| {
            let new_layout = match new_type {
                "Flex" => LayoutType::Flex {
                    direction: FlexDirection::Column,
                    wrap: false,
                    align_items: FlexAlign::Start,
                    justify_content: FlexJustify::Start,
                },
                "Grid" => LayoutType::Grid { columns: 2, rows: 2 }, // Default 2x2 grid
                "Stack" => LayoutType::Stack,
                _ => return,
            };

            let mut new_container = container.clone();
            new_container.layout = new_layout;
            apply_update(comp_id, CanvasComponent::Container(new_container), "layout".to_string());
        }
    };

    let is_flex = matches!(container.layout, LayoutType::Flex { .. });

    // Extract current flex values if applicable
    let (direction, align, justify) = if let LayoutType::Flex { direction, align_items, justify_content, .. } = &container.layout {
        (Some(direction.clone()), Some(align_items.clone()), Some(justify_content.clone()))
    } else {
        (None, None, None)
    };

    view! {
        <div class="property-group-container">
            // 1. Layout Mode Selection
            <div class="property-section">
                <div class="section-header">"Layout"</div>
                <div class="layout-selector">
                    <button
                        class={
                            let c = container.clone();
                            move || if matches!(c.layout, LayoutType::Flex { .. }) { "layout-btn active" } else { "layout-btn" }
                        }
                        on:click={
                            let switch_layout = switch_layout.clone();
                            move |_| switch_layout("Flex")
                        }
                    >
                        <span class="icon">"⊞"</span>
                        "Flex"
                    </button>
                    <button
                        class={
                            let c = container.clone();
                            move || if matches!(c.layout, LayoutType::Grid { .. }) { "layout-btn active" } else { "layout-btn" }
                        }
                        on:click={
                            let switch_layout = switch_layout.clone();
                            move |_| switch_layout("Grid")
                        }
                    >
                        <span class="icon">"▦"</span>
                        "Grid"
                    </button>
                    <button
                         class={
                            let c = container.clone();
                            move || if matches!(c.layout, LayoutType::Stack { .. }) { "layout-btn active" } else { "layout-btn" }
                        }
                        on:click={
                            let switch_layout = switch_layout.clone();
                            move |_| switch_layout("Stack")
                        }
                    >
                        <span class="icon">"☰"</span>
                        "Stack"
                    </button>
                </div>
            </div>

            // 2. Flex Controls (Conditional)
            {if is_flex {
                let container = container.clone();
                let apply_update = apply_update.clone();
                let comp_id = comp_id;

                let current_dir = direction.unwrap_or(FlexDirection::Column);
                let current_align = align.unwrap_or(FlexAlign::Start);
                let current_justify = justify.unwrap_or(FlexJustify::Start);

                view! {
                    <div class="property-section fade-in">
                        <div class="section-header">"Flex Options"</div>

                        // Direction
                        <div class="control-row">
                            <span class="label">"Direction"</span>
                            <div class="toggle-group">
                                <button
                                    class=if matches!(current_dir, FlexDirection::Row) { "toggle-btn active" } else { "toggle-btn" }
                                    title="Row"
                                    on:click={
                                        let c = container.clone();
                                        let a = apply_update.clone();
                                        move |_| {
                                            let updated = update_container_prop(c.clone(), "layout", PropValue::String("FlexRow".to_string()));
                                            a(comp_id, CanvasComponent::Container(updated), "direction".to_string());
                                        }
                                    }
                                >"→"</button>
                                <button
                                    class=if matches!(current_dir, FlexDirection::Column) { "toggle-btn active" } else { "toggle-btn" }
                                    title="Column"
                                    on:click={
                                        let c = container.clone();
                                        let a = apply_update.clone();
                                        move |_| {
                                            let updated = update_container_prop(c.clone(), "layout", PropValue::String("FlexColumn".to_string()));
                                            a(comp_id, CanvasComponent::Container(updated), "direction".to_string());
                                        }
                                    }
                                >"↓"</button>
                            </div>
                        </div>

                        // Align Items
                        <div class="control-row">
                            <span class="label">"Align"</span>
                            <div class="toggle-group compact">
                                <button class=if matches!(current_align, FlexAlign::Start) { "toggle-btn active" } else { "toggle-btn" }
                                    title="Start"
                                    on:click={
                                        let c = container.clone(); let a = apply_update.clone();
                                        move |_| { let u = update_container_prop(c.clone(), "align_items", PropValue::String("Start".to_string())); a(comp_id, CanvasComponent::Container(u), "align".to_string()); }
                                    }>"├"</button>
                                <button class=if matches!(current_align, FlexAlign::Center) { "toggle-btn active" } else { "toggle-btn" }
                                    title="Center"
                                    on:click={
                                        let c = container.clone(); let a = apply_update.clone();
                                        move |_| { let u = update_container_prop(c.clone(), "align_items", PropValue::String("Center".to_string())); a(comp_id, CanvasComponent::Container(u), "align".to_string()); }
                                    }>"┼"</button>
                                <button class=if matches!(current_align, FlexAlign::End) { "toggle-btn active" } else { "toggle-btn" }
                                    title="End"
                                    on:click={
                                        let c = container.clone(); let a = apply_update.clone();
                                        move |_| { let u = update_container_prop(c.clone(), "align_items", PropValue::String("End".to_string())); a(comp_id, CanvasComponent::Container(u), "align".to_string()); }
                                    }>"┤"</button>
                            </div>
                        </div>

                        // Justify Content
                        <div class="control-row">
                            <span class="label">"Justify"</span>
                            <div class="toggle-group compact">
                                <button class=if matches!(current_justify, FlexJustify::Start) { "toggle-btn active" } else { "toggle-btn" }
                                    title="Start"
                                    on:click={
                                        let c = container.clone(); let a = apply_update.clone();
                                        move |_| { let u = update_container_prop(c.clone(), "justify_content", PropValue::String("Start".to_string())); a(comp_id, CanvasComponent::Container(u), "justify".to_string()); }
                                    }>"├"</button>
                                <button class=if matches!(current_justify, FlexJustify::Center) { "toggle-btn active" } else { "toggle-btn" }
                                    title="Center"
                                    on:click={
                                        let c = container.clone(); let a = apply_update.clone();
                                        move |_| { let u = update_container_prop(c.clone(), "justify_content", PropValue::String("Center".to_string())); a(comp_id, CanvasComponent::Container(u), "justify".to_string()); }
                                    }>"┼"</button>
                                <button class=if matches!(current_justify, FlexJustify::End) { "toggle-btn active" } else { "toggle-btn" }
                                    title="End"
                                    on:click={
                                        let c = container.clone(); let a = apply_update.clone();
                                        move |_| { let u = update_container_prop(c.clone(), "justify_content", PropValue::String("End".to_string())); a(comp_id, CanvasComponent::Container(u), "justify".to_string()); }
                                    }>"┤"</button>
                                <button class=if matches!(current_justify, FlexJustify::Between) { "toggle-btn active" } else { "toggle-btn" }
                                    title="Between"
                                    on:click={
                                        let c = container.clone(); let a = apply_update.clone();
                                        move |_| { let u = update_container_prop(c.clone(), "justify_content", PropValue::String("Between".to_string())); a(comp_id, CanvasComponent::Container(u), "justify".to_string()); }
                                    }>"↔"</button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}

            // 3. Spacing (Visual Box Model)
            <div class="property-section">
                <div class="section-header">"Spacing"</div>
                <div class="box-model-editor">
                    <div class="box-label">"PADDING"</div>
                    <div class="box-top">
                         <NumberInput
                            value={container.padding.top as f64}
                            label="".to_string()
                            on_change={
                                let c = container.clone(); let a = apply_update.clone();
                                move |v| { let u = update_container_prop(c.clone(), "padding_top", PropValue::Number(v)); a(comp_id, CanvasComponent::Container(u), "padding-top".to_string()); }
                            }
                        />
                    </div>
                    <div class="box-middle">
                        <div class="box-left">
                             <NumberInput
                                value={container.padding.left as f64}
                                label="".to_string()
                                on_change={
                                    let c = container.clone(); let a = apply_update.clone();
                                    move |v| { let u = update_container_prop(c.clone(), "padding_left", PropValue::Number(v)); a(comp_id, CanvasComponent::Container(u), "padding-left".to_string()); }
                                }
                            />
                        </div>
                        <div class="box-content"></div>
                        <div class="box-right">
                             <NumberInput
                                value={container.padding.right as f64}
                                label="".to_string()
                                on_change={
                                    let c = container.clone(); let a = apply_update.clone();
                                    move |v| { let u = update_container_prop(c.clone(), "padding_right", PropValue::Number(v)); a(comp_id, CanvasComponent::Container(u), "padding-right".to_string()); }
                                }
                            />
                        </div>
                    </div>
                    <div class="box-bottom">
                         <NumberInput
                            value={container.padding.bottom as f64}
                            label="".to_string()
                            on_change={
                                let c = container.clone(); let a = apply_update.clone();
                                move |v| { let u = update_container_prop(c.clone(), "padding_bottom", PropValue::Number(v)); a(comp_id, CanvasComponent::Container(u), "padding-bottom".to_string()); }
                            }
                        />
                    </div>
                </div>

                <div class="control-row" style="margin-top: 12px;">
                    <NumberInput
                        value={container.gap as f64}
                        label="Gap (px)".to_string()
                        on_change={
                            let c = container.clone(); let a = apply_update.clone();
                            move |v| { let u = update_container_prop(c.clone(), "gap", PropValue::Number(v)); a(comp_id, CanvasComponent::Container(u), "gap".to_string()); }
                        }
                    />
                </div>
            </div>

            <style>
            "
            .property-group-container {
                display: flex;
                flex-direction: column;
                gap: 16px;
            }
            .property-section {
                background: rgba(255, 255, 255, 0.05);
                border-radius: 8px;
                padding: 12px;
                border: 1px solid rgba(255, 255, 255, 0.1);
            }
            .section-header {
                font-size: 0.75rem;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.05em;
                color: rgba(255, 255, 255, 0.6);
                margin-bottom: 8px;
            }
            .layout-selector {
                display: grid;
                grid-template-columns: 1fr 1fr 1fr;
                gap: 4px;
                background: rgba(0, 0, 0, 0.2);
                padding: 4px;
                border-radius: 6px;
            }
            .layout-btn {
                background: transparent;
                border: none;
                color: #aaa;
                padding: 8px;
                border-radius: 4px;
                cursor: pointer;
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 4px;
                font-size: 0.7rem;
                transition: all 0.2s;
            }
            .layout-btn:hover {
                background: rgba(255, 255, 255, 0.05);
                color: #fff;
            }
            .layout-btn.active {
                background: #3b82f6;
                color: white;
            }
            .layout-btn .icon {
                font-size: 1.2rem;
            }
            .control-row {
                display: flex;
                align-items: center;
                justify-content: space-between;
                margin-bottom: 8px;
            }
            .control-row .label {
                font-size: 0.8rem;
                color: #ccc;
            }
            .toggle-group {
                display: flex;
                background: rgba(0, 0, 0, 0.2);
                border-radius: 4px;
                padding: 2px;
            }
            .toggle-btn {
                background: transparent;
                border: none;
                color: #888;
                width: 32px;
                height: 32px;
                border-radius: 4px;
                cursor: pointer;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 1.1rem;
                transition: all 0.2s;
            }
            .toggle-btn:hover {
                color: #fff;
                background: rgba(255,255,255,0.05);
            }
            .toggle-btn.active {
                background: #3b82f6;
                color: white;
            }

            /* Visual Box Model */
            .box-model-editor {
                position: relative;
                background: rgba(0,0,0,0.3);
                border: 1px dashed #555;
                padding: 24px 34px;
                border-radius: 4px;
                display: flex;
                flex-direction: column;
                align-items: center;
                margin-top: 8px;
            }
            .box-label {
                position: absolute;
                top: 2px;
                left: 4px;
                font-size: 0.6rem;
                color: #555;
            }
            .box-top, .box-bottom {
                width: 50px;
                text-align: center;
            }
            .box-middle {
                display: flex;
                align-items: center;
                width: 100%;
                justify-content: center;
                gap: 8px;
                margin: 4px 0;
            }
            .box-left, .box-right {
                width: 50px;
            }
            .box-content {
                width: 40px;
                height: 24px;
                background: rgba(59, 130, 246, 0.2);
                border: 1px solid rgba(59, 130, 246, 0.4);
                border-radius: 2px;
            }
            /* Override NumberInput styling for the box model to make it compact/transparent */
            .box-model-editor input {
                text-align: center;
                background: transparent;
                border: none;
                color: white;
                width: 100%;
                font-size: 0.85rem;
                padding: 0;
            }
            .box-model-editor .property-field label {
                display: none; /* Hide standard labels inside the box model */
            }
            .box-model-editor .property-field {
                margin: 0;
            }
            "
            </style>
        </div>
    }
}
