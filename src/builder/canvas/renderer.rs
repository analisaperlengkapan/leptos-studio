use crate::builder::canvas::handle_drop;
use crate::builder::drag_drop::DropZone;
use crate::domain::{
    ButtonComponent, CanvasComponent, ContainerComponent, CustomComponent, InputComponent,
    TextComponent,
};
use crate::state::{AppState, CanvasState};
use leptos::prelude::*;

/// Component renderer for displaying canvas components
#[component]
pub fn ComponentRenderer(
    /// The component to render
    component: CanvasComponent,
    /// Canvas state for selection tracking
    canvas_state: CanvasState,
) -> impl IntoView {
    let app_state = AppState::expect_context();
    let component_id = *component.id();
    let preview_mode = app_state.ui.preview_mode;

    let is_selected = Memo::new(move |_| {
        !preview_mode.get()
            && canvas_state
                .selected
                .get()
                .as_ref()
                .map(|id| id == &component_id)
                .unwrap_or(false)
    });

    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        if !preview_mode.get() {
            canvas_state.selected.set(Some(component_id));
        }
    };

    let class = move || {
        if is_selected.get() {
            "canvas-component selected"
        } else {
            "canvas-component"
        }
    };

    let component_type_label = match component.component_type() {
        crate::domain::ComponentType::Button => "Button",
        crate::domain::ComponentType::Text => "Text",
        crate::domain::ComponentType::Input => "Input",
        crate::domain::ComponentType::Container => "Container",
        crate::domain::ComponentType::Custom => "Custom",
    };

    view! {
        <div
            class=class
            on:click=on_click
            data-component-id=component_id.to_string()
        >
            {move || if is_selected.get() {
                view! { <div class="selected-label">{component_type_label}</div> }.into_any()
            } else {
                view! { }.into_any()
            }}
            {match component {
                CanvasComponent::Button(btn) => render_button(btn).into_any(),
                CanvasComponent::Text(txt) => render_text(txt).into_any(),
                CanvasComponent::Input(inp) => render_input(inp).into_any(),
                CanvasComponent::Container(container) => render_container(container, canvas_state).into_any(),
                CanvasComponent::Custom(custom) => render_custom(custom).into_any(),
            }}
        </div>
    }
}

fn render_button(button: ButtonComponent) -> impl IntoView {
    let variant_class = match button.variant {
        crate::domain::ButtonVariant::Primary => "btn-primary",
        crate::domain::ButtonVariant::Secondary => "btn-secondary",
        crate::domain::ButtonVariant::Outline => "btn-outline",
        crate::domain::ButtonVariant::Ghost => "btn-ghost",
    };

    let size_class = match button.size {
        crate::domain::ButtonSize::Small => "btn-sm",
        crate::domain::ButtonSize::Medium => "btn-md",
        crate::domain::ButtonSize::Large => "btn-lg",
    };

    view! {
        <button
            class=format!("canvas-button {} {}", variant_class, size_class)
            disabled=button.disabled
        >
            {button.label}
        </button>
    }
}

fn render_text(text: TextComponent) -> impl IntoView {
    let tag_class = match text.tag {
        crate::domain::TextTag::H1 => "text-h1",
        crate::domain::TextTag::H2 => "text-h2",
        crate::domain::TextTag::H3 => "text-h3",
        crate::domain::TextTag::P => "text-p",
        crate::domain::TextTag::Span => "text-span",
    };

    let style_class = match text.style {
        crate::domain::TextStyle::Heading1 => "style-heading1",
        crate::domain::TextStyle::Heading2 => "style-heading2",
        crate::domain::TextStyle::Heading3 => "style-heading3",
        crate::domain::TextStyle::Body => "style-body",
        crate::domain::TextStyle::Caption => "style-caption",
    };

    view! {
        <span class=format!("canvas-text {} {}", tag_class, style_class)>
            {text.content}
        </span>
    }
}

fn render_input(input: InputComponent) -> impl IntoView {
    let input_type = match input.input_type {
        crate::domain::InputType::Text => "text",
        crate::domain::InputType::Password => "password",
        crate::domain::InputType::Email => "email",
        crate::domain::InputType::Number => "number",
        crate::domain::InputType::Tel => "tel",
    };

    view! {
        <input
            class="canvas-input"
            type=input_type
            placeholder=input.placeholder
            required=input.required
            disabled=input.disabled
        />
    }
}

fn render_container(container: ContainerComponent, canvas_state: CanvasState) -> impl IntoView {
    let layout_class = match &container.layout {
        crate::domain::LayoutType::Flex { direction, wrap } => {
            let dir_class = match direction {
                crate::domain::FlexDirection::Row => "flex-row",
                crate::domain::FlexDirection::Column => "flex-col",
            };
            if *wrap {
                format!("{} flex-wrap", dir_class)
            } else {
                dir_class.to_string()
            }
        }
        crate::domain::LayoutType::Grid { columns, rows } => {
            format!("grid grid-cols-{} grid-rows-{}", columns, rows)
        }
        crate::domain::LayoutType::Stack => "stack".to_string(),
    };

    let style = format!(
        "gap: {}px; padding: {}px {}px {}px {}px;",
        container.gap,
        container.padding.top,
        container.padding.right,
        container.padding.bottom,
        container.padding.left
    );

    let container_id = container.id;

    // Handle dropping items into this container
    let on_drop = move |ev: leptos::ev::DragEvent| {
        handle_drop(ev, canvas_state, Some(container_id));
    };

    let has_children = !container.children.is_empty();
    let preview_mode = AppState::expect_context().ui.preview_mode;

    // We need to clone these for the closures to use
    let children = container.children.clone();
    let container_id = container.id;

    view! {
        {move || {
            let style = style.clone();
            let layout_class = layout_class.clone();
            let children = children.clone();

            if !preview_mode.get() {
                view! {
                    <DropZone
                        zone_name=format!("container-{}", container_id)
                        drag_state=canvas_state.drag_state
                        on_drop=on_drop
                        config=None
                    >
                        <div
                            class=format!("canvas-container {}", layout_class)
                            class:hovered=move || {
                                 // Check if this container is being dragged over
                                 if let crate::builder::drag_drop::DragState::DraggingOver { drop_zone, .. } = canvas_state.drag_state.get() {
                                     drop_zone == format!("container-{}", container_id)
                                 } else {
                                     false
                                 }
                            }
                            style=style
                        >
                            {if has_children {
                                 let children = children.clone();
                                view! {
                                    <For
                                        each=move || children.clone()
                                        key=|comp| *comp.id()
                                        children=move |comp| {
                                            view! {
                                                <ComponentRenderer
                                                    component=comp
                                                    canvas_state=canvas_state
                                                />
                                            }
                                        }
                                    />
                                }.into_any()
                            } else {
                                view! {
                                    <div class="empty-container-placeholder">
                                        "Drop items here"
                                    </div>
                                }.into_any()
                            }}
                        </div>
                    </DropZone>
                }.into_any()
            } else {
                view! {
                    <div class=format!("canvas-container {}", layout_class) style=style>
                         <For
                            each=move || children.clone()
                            key=|comp| *comp.id()
                            children=move |comp| {
                                view! {
                                    <ComponentRenderer
                                        component=comp
                                        canvas_state=canvas_state
                                    />
                                }
                            }
                        />
                    </div>
                }.into_any()
            }
        }}
    }
}

fn render_custom(custom: CustomComponent) -> impl IntoView {
    view! {
        <div class="canvas-custom">
            <div class="custom-header">
                <span class="custom-name">{custom.name.clone()}</span>
            </div>
            <div class="custom-template" inner_html=custom.template></div>
        </div>
    }
}
