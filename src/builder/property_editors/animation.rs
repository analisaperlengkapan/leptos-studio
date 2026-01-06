use crate::builder::property_inputs::{BoolCheckbox, EnumSelect, NumberInput};
use crate::domain::{Animation, AnimationType, ComponentId};
use leptos::prelude::*;

#[component]
pub fn AnimationPropertyEditor(
    _id: ComponentId,
    #[prop(into)] animation: Option<Animation>,
    #[prop(into)] on_change: Callback<Option<Animation>>,
) -> impl IntoView {
    let current_anim = animation.unwrap_or_default();

    // We need to clone it for the closures
    let anim_type_val = current_anim.animation_type.clone();
    let duration_val = current_anim.duration;
    let delay_val = current_anim.delay;
    let infinite_val = current_anim.infinite;

    let update_anim = move |new_anim: Animation| {
        if new_anim.animation_type == AnimationType::None {
            on_change.run(None);
        } else {
            on_change.run(Some(new_anim));
        }
    };

    // Clones for the reactive closure below
    let anim_type_for_block = anim_type_val.clone();
    let anim_type_for_dur = anim_type_val.clone();
    let anim_type_for_delay = anim_type_val.clone();
    let anim_type_for_infinite = anim_type_val.clone();

    let update_anim_dur = update_anim.clone();
    let update_anim_delay = update_anim.clone();
    let update_anim_inf = update_anim.clone();

    view! {
        <div class="property-group">
            <div class="group-title">"Animation"</div>

            <EnumSelect
                label="Type".to_string()
                value=match anim_type_val {
                    AnimationType::None => "None",
                    AnimationType::FadeIn => "Fade In",
                    AnimationType::SlideInUp => "Slide Up",
                    AnimationType::SlideInDown => "Slide Down",
                    AnimationType::SlideInLeft => "Slide Left",
                    AnimationType::SlideInRight => "Slide Right",
                    AnimationType::Bounce => "Bounce",
                    AnimationType::ZoomIn => "Zoom In",
                    AnimationType::Pulse => "Pulse",
                }.to_string()
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

                    let mut new_anim = Animation::default();
                    new_anim.animation_type = new_type;
                    // Preserve other values if we're switching between active animations
                    if anim_type_val != AnimationType::None {
                         new_anim.duration = duration_val;
                         new_anim.delay = delay_val;
                         new_anim.infinite = infinite_val;
                    }

                    update_anim(new_anim);
                }
            />

            {move || {
                let anim_type = anim_type_for_block.clone();
                let anim_type_dur = anim_type_for_dur.clone();
                let anim_type_delay = anim_type_for_delay.clone();
                let anim_type_inf = anim_type_for_infinite.clone();

                let update_dur = update_anim_dur.clone();
                let update_delay = update_anim_delay.clone();
                let update_inf = update_anim_inf.clone();

                if anim_type != AnimationType::None {
                    view! {
                        <div style="margin-top: 8px;">
                            <NumberInput
                                label="Duration (s)".to_string()
                                value=duration_val as f64
                                min=0.1
                                max=10.0
                                step=0.1
                                on_change=move |val| {
                                    let new_anim = Animation {
                                        animation_type: anim_type_dur.clone(),
                                        duration: val as f32,
                                        delay: delay_val,
                                        infinite: infinite_val,
                                    };
                                    update_dur(new_anim);
                                }
                            />

                            <NumberInput
                                label="Delay (s)".to_string()
                                value=delay_val as f64
                                min=0.0
                                max=10.0
                                step=0.1
                                on_change=move |val| {
                                    let new_anim = Animation {
                                        animation_type: anim_type_delay.clone(),
                                        duration: duration_val,
                                        delay: val as f32,
                                        infinite: infinite_val,
                                    };
                                    update_delay(new_anim);
                                }
                            />

                            <BoolCheckbox
                                label="Infinite Loop".to_string()
                                checked=infinite_val
                                on_change=move |val| {
                                    let new_anim = Animation {
                                        animation_type: anim_type_inf.clone(),
                                        duration: duration_val,
                                        delay: delay_val,
                                        infinite: val,
                                    };
                                    update_inf(new_anim);
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
