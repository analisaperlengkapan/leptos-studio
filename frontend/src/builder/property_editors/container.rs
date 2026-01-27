use super::AnimationPropertyEditor;
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
            // Overwrite using new signature (closure that assigns the value)
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

            <AnimationPropertyEditor
                _id=comp_id
                animation=container.animation.clone()
                on_change=move |new_anim| {
                    let mut updated = container.clone();
                    updated.animation = new_anim;
                    apply_update(comp_id, CanvasComponent::Container(updated), "animation".to_string());
                }
            />

        </div>
    }
}
