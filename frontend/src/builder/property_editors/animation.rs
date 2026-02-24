use crate::builder::property_inputs::{BoolCheckbox, EnumSelect, NumberInput};
use crate::domain::{Animation, AnimationType, ComponentId};
use leptos::prelude::*;

#[component]
pub fn AnimationPropertyEditor(
    _id: ComponentId,
    #[prop(into)] animation: Option<Animation>,
    #[prop(into)] on_change: Callback<Option<Animation>>,
) -> impl IntoView {
    // Initialize signal from prop to avoid stale captures
    let current_anim = RwSignal::new(animation.unwrap_or_default());

    view! {
        <div class="property-group">
            <div class="group-title">"Animation"</div>

            {move || {
                let anim = current_anim.get();
                let anim_type_str = match anim.animation_type {
                    AnimationType::None => "None",
                    AnimationType::FadeIn => "Fade In",
                    AnimationType::SlideInUp => "Slide Up",
                    AnimationType::SlideInDown => "Slide Down",
                    AnimationType::SlideInLeft => "Slide Left",
                    AnimationType::SlideInRight => "Slide Right",
                    AnimationType::Bounce => "Bounce",
                    AnimationType::ZoomIn => "Zoom In",
                    AnimationType::Pulse => "Pulse",
                }.to_string();

                view! {
                    <EnumSelect
                        label="Type".to_string()
                        value=anim_type_str
                        options=vec![
                            "None".to_string(), "Fade In".to_string(), "Slide Up".to_string(),
                            "Slide Down".to_string(), "Slide Left".to_string(), "Slide Right".to_string(),
                            "Bounce".to_string(), "Zoom In".to_string(), "Pulse".to_string()
                        ]
                        on_change=move |val| {
                            let new_type = match val.as_str() {
                                "None" => AnimationType::None,
                                "Fade In" => AnimationType::FadeIn,
                                "Slide Up" => AnimationType::SlideInUp,
                                "Slide Down" => AnimationType::SlideInDown,
                                "Slide Left" => AnimationType::SlideInLeft,
                                "Slide Right" => AnimationType::SlideInRight,
                                "Bounce" => AnimationType::Bounce,
                                "Zoom In" => AnimationType::ZoomIn,
                                "Pulse" => AnimationType::Pulse,
                                _ => AnimationType::None,
                            };

                            let mut anim = current_anim.get();
                            // Preserve other values if we're switching between active animations
                            // If switching from None, we might want defaults, but current_anim holds state.

                            anim.animation_type = new_type.clone();
                            current_anim.set(anim.clone());

                            if new_type == AnimationType::None {
                                on_change.run(None);
                            } else {
                                on_change.run(Some(anim));
                            }
                        }
                    />
                }
            }}

            {move || {
                let anim = current_anim.get();

                if anim.animation_type != AnimationType::None {
                    view! {
                        <div style="margin-top: 8px;">
                            <NumberInput
                                label="Duration (s)".to_string()
                                value=anim.duration as f64
                                min_value=0.1
                                max_value=10.0
                                step_value=0.1
                                on_change=move |val| {
                                    let mut anim = current_anim.get();
                                    anim.duration = val as f32;
                                    current_anim.set(anim.clone());
                                    on_change.run(Some(anim));
                                }
                            />

                            <NumberInput
                                label="Delay (s)".to_string()
                                value=anim.delay as f64
                                min_value=0.0
                                max_value=10.0
                                step_value=0.1
                                on_change=move |val| {
                                    let mut anim = current_anim.get();
                                    anim.delay = val as f32;
                                    current_anim.set(anim.clone());
                                    on_change.run(Some(anim));
                                }
                            />

                            <BoolCheckbox
                                label="Infinite Loop".to_string()
                                checked=anim.infinite
                                on_change=move |val| {
                                    let mut anim = current_anim.get();
                                    anim.infinite = val;
                                    current_anim.set(anim.clone());
                                    on_change.run(Some(anim));
                                }
                            />
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}
