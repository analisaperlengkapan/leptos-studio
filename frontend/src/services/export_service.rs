use crate::domain::{Animation, AppError, AppResult, CanvasComponent, Variable, VariableType};
use crate::state::ExportPreset;
use std::cell::RefCell;

/// Helper to generate animation styles
fn get_animation_css(animation: &Option<Animation>) -> String {
    animation
        .as_ref()
        .map(|a| a.to_css_string())
        .unwrap_or_default()
}

/// Code generator trait
pub trait CodeGenerator {
    fn generate(&self, components: &[CanvasComponent], variables: &[Variable])
    -> AppResult<String>;
    fn file_extension(&self) -> &str;
}

/// Leptos code generator
pub struct LeptosCodeGenerator {
    preset: ExportPreset,
    /// Tracks signals that need to be injected at the top of the component
    /// Format: (signal_name, default_value)
    required_signals: RefCell<Vec<(String, String)>>,
}

impl LeptosCodeGenerator {
    pub fn new(preset: ExportPreset) -> Self {
        Self {
            preset,
            required_signals: RefCell::new(Vec::new()),
        }
    }

    fn generate_imports(&self) -> String {
        match self.preset {
            ExportPreset::Plain => "use leptos::*;\n".to_string(),
            ExportPreset::ThawUi => "use leptos::*;\nuse thaw::*;\n".to_string(),
            ExportPreset::LeptosMaterial => "use leptos::*;\nuse leptos_material::*;\n".to_string(),
            ExportPreset::LeptosUse => "use leptos::*;\nuse leptos_use::*;\n".to_string(),
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn generate_component(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "    ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                let id_attr = if let Some(bind) = btn.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = btn.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
                } else {
                    String::new()
                };

                let variant_class = match btn.variant {
                    crate::domain::ButtonVariant::Primary => "btn-primary",
                    crate::domain::ButtonVariant::Secondary => "btn-secondary",
                    crate::domain::ButtonVariant::Outline => "btn-outline",
                    crate::domain::ButtonVariant::Ghost => "btn-ghost",
                };

                let size_class = match btn.size {
                    crate::domain::ButtonSize::Small => "btn-sm",
                    crate::domain::ButtonSize::Medium => "btn-md",
                    crate::domain::ButtonSize::Large => "btn-lg",
                };

                let anim_style = get_animation_css(&btn.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let click_handler = if let Some(handler) = &btn.on_click {
                    format!("on:click={}", handler)
                } else {
                    "on:click=move |_| { web_sys::console::log_1(&\"Clicked\".into()); }"
                        .to_string()
                };

                let label_expr = if let Some(bind) = btn.bindings.get("label") {
                    format!("move || {}.get()", bind)
                } else {
                    format!("\"{}\"", btn.label)
                };

                let disabled_attr = if let Some(bind) = btn.bindings.get("disabled") {
                    format!("prop:disabled=move || {}.get()", bind)
                } else {
                    format!("disabled={}", btn.disabled)
                };

                // Add interactive scaffolding
                output.push_str(&format!(
                    "{}        <button{}{} class=\"{} {}\" {} {}{}>{}</button>\n",
                    indent,
                    id_attr,
                    class_attr,
                    variant_class,
                    size_class,
                    disabled_attr,
                    click_handler,
                    style_attr,
                    label_expr
                ));
            }
            CanvasComponent::Text(txt) => {
                let id_attr = if let Some(bind) = txt.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = txt.bindings.get("custom_css_classes") {
                    format!(
                        " class=move || format!(\"text-{{}} {{}}\", \"{}\", {}.get())",
                        match txt.style {
                            crate::domain::TextStyle::Heading1 => "heading1",
                            crate::domain::TextStyle::Heading2 => "heading2",
                            crate::domain::TextStyle::Heading3 => "heading3",
                            crate::domain::TextStyle::Body => "body",
                            crate::domain::TextStyle::Caption => "caption",
                        },
                        bind
                    )
                } else {
                    format!(
                        " class=\"text-{}\"",
                        match txt.style {
                            crate::domain::TextStyle::Heading1 => "heading1",
                            crate::domain::TextStyle::Heading2 => "heading2",
                            crate::domain::TextStyle::Heading3 => "heading3",
                            crate::domain::TextStyle::Body => "body",
                            crate::domain::TextStyle::Caption => "caption",
                        }
                    )
                };

                let tag = match txt.tag {
                    crate::domain::TextTag::H1 => "h1",
                    crate::domain::TextTag::H2 => "h2",
                    crate::domain::TextTag::H3 => "h3",
                    crate::domain::TextTag::P => "p",
                    crate::domain::TextTag::Span => "span",
                };

                let anim_style = get_animation_css(&txt.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let content_expr = if let Some(bind) = txt.bindings.get("content") {
                    format!("move || {}.get()", bind)
                } else {
                    format!("\"{}\"", txt.content)
                };

                output.push_str(&format!(
                    "{}        <{}{}{}{}>{}</{}>\n",
                    indent, tag, id_attr, class_attr, style_attr, content_expr, tag
                ));
            }
            CanvasComponent::Input(inp) => {
                let id_attr = if let Some(bind) = inp.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = inp.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
                } else {
                    String::new()
                };

                let input_type = match inp.input_type {
                    crate::domain::InputType::Text => "text",
                    crate::domain::InputType::Password => "password",
                    crate::domain::InputType::Email => "email",
                    crate::domain::InputType::Number => "number",
                    crate::domain::InputType::Tel => "tel",
                };

                let anim_style = get_animation_css(&inp.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let input_handler = if let Some(handler) = &inp.on_input {
                    format!("on:input={}", handler)
                } else {
                    // Track signal for this input
                    let signal_name = format!("input_{}", self.required_signals.borrow().len());
                    self.required_signals
                        .borrow_mut()
                        .push((signal_name.clone(), "String::new()".to_string()));
                    format!(
                        "prop:value={} on:input=move |ev| set_{}(event_target_value(&ev))",
                        signal_name, signal_name
                    )
                };

                let change_handler = if let Some(handler) = &inp.on_change {
                    format!(" on:change={}", handler)
                } else {
                    String::new()
                };

                let placeholder_attr = if let Some(bind) = inp.bindings.get("placeholder") {
                    format!("prop:placeholder=move || {}.get()", bind)
                } else {
                    format!("placeholder=\"{}\"", inp.placeholder)
                };

                let disabled_attr = if let Some(bind) = inp.bindings.get("disabled") {
                    format!("prop:disabled=move || {}.get()", bind)
                } else {
                    format!("disabled={}", inp.disabled)
                };

                // Add binding
                output.push_str(&format!(
                    "{}        <input{}{} type=\"{}\" {} {} {} required={} {}{} />\n",
                    indent,
                    id_attr,
                    class_attr,
                    input_type,
                    placeholder_attr,
                    input_handler,
                    change_handler,
                    inp.required,
                    disabled_attr,
                    style_attr
                ));
            }
            CanvasComponent::Select(sel) => {
                let id_attr = if let Some(bind) = sel.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = sel.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
                } else {
                    String::new()
                };

                let anim_style = get_animation_css(&sel.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let change_handler = if let Some(handler) = &sel.on_change {
                    format!(" on:change={}", handler)
                } else {
                    String::new()
                };

                let options_expr = if let Some(bind) = sel.bindings.get("options") {
                    format!(
                        "move || {}.get().split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>().into_iter().map(|opt| view! {{ <option value=opt.clone()>{{opt}}</option> }}).collect_view()",
                        bind
                    )
                } else {
                    let opts = sel
                        .options
                        .split(',')
                        .map(|o| {
                            format!(
                                "view! {{ <option value=\"{}\">\"{}\"</option> }}",
                                o.trim(),
                                o.trim()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("vec![{}].collect_view()", opts)
                };

                let disabled_attr = if let Some(bind) = sel.bindings.get("disabled") {
                    format!("prop:disabled=move || {}.get()", bind)
                } else {
                    format!("disabled={}", sel.disabled)
                };

                let placeholder_attr = if let Some(bind) = sel.bindings.get("placeholder") {
                    format!("prop:placeholder=move || {}.get()", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "{}        <select{}{} {} {} {}{}>\n",
                    indent,
                    id_attr,
                    class_attr,
                    disabled_attr,
                    placeholder_attr,
                    change_handler,
                    style_attr
                ));

                if !sel.placeholder.is_empty() {
                    output.push_str(&format!(
                        "{}            <option value=\"\" disabled selected>\"{}\"</option>\n",
                        indent, sel.placeholder
                    ));
                }

                output.push_str(&format!("{}            {{{}}}\n", indent, options_expr));

                output.push_str(&format!("{}        </select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                let (layout_class, align_style) = match &container.layout {
                    crate::domain::LayoutType::Flex {
                        direction,
                        wrap,
                        align_items,
                        justify_content,
                    } => {
                        let dir = match direction {
                            crate::domain::FlexDirection::Row => "flex-row",
                            crate::domain::FlexDirection::Column => "flex-col",
                        };
                        let mut classes = dir.to_string();
                        if *wrap {
                            classes.push_str(" flex-wrap");
                        }

                        let align_css = match align_items {
                            crate::domain::FlexAlign::Start => "flex-start",
                            crate::domain::FlexAlign::Center => "center",
                            crate::domain::FlexAlign::End => "flex-end",
                            crate::domain::FlexAlign::Stretch => "stretch",
                            crate::domain::FlexAlign::Baseline => "baseline",
                        };

                        let justify_css = match justify_content {
                            crate::domain::FlexJustify::Start => "flex-start",
                            crate::domain::FlexJustify::Center => "center",
                            crate::domain::FlexJustify::End => "flex-end",
                            crate::domain::FlexJustify::Between => "space-between",
                            crate::domain::FlexJustify::Around => "space-around",
                            crate::domain::FlexJustify::Evenly => "space-evenly",
                        };

                        (
                            classes,
                            format!(
                                "align-items: {}; justify-content: {};",
                                align_css, justify_css
                            ),
                        )
                    }
                    crate::domain::LayoutType::Grid { columns, rows } => (
                        format!("grid grid-cols-{} grid-rows-{}", columns, rows),
                        String::new(),
                    ),
                    crate::domain::LayoutType::Stack => ("stack".to_string(), String::new()),
                };

                let anim_style = get_animation_css(&container.animation);

                let click_handler = if let Some(handler) = &container.on_click {
                    format!(" on:click={}", handler)
                } else {
                    String::new()
                };

                let id_attr = if let Some(bind) = container.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = container.bindings.get("custom_css_classes") {
                    format!(
                        " class=move || format!(\"container {} {{}}\", {}.get())",
                        layout_class, bind
                    )
                } else {
                    format!(" class=\"container {}\"", layout_class)
                };

                output.push_str(&format!(
                    "{}        <div{}{} style=\"gap: {}px; padding: {}px {}px {}px {}px; {} {}\"{}>\n",
                    indent,
                    id_attr,
                    class_attr,
                    container.gap,
                    container.padding.top,
                    container.padding.right,
                    container.padding.bottom,
                    container.padding.left,
                    align_style,
                    anim_style,
                    click_handler
                ));

                // Recursively generate children
                for child in &container.children {
                    self.generate_component(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}        </div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                let id_attr = if let Some(bind) = img.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = img.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
                } else {
                    String::new()
                };

                let width_attr = img
                    .width
                    .as_ref()
                    .map_or(String::new(), |w| format!(" width=\"{}\"", w));
                let height_attr = img
                    .height
                    .as_ref()
                    .map_or(String::new(), |h| format!(" height=\"{}\"", h));

                let anim_style = get_animation_css(&img.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let click_handler = if let Some(handler) = &img.on_click {
                    format!(" on:click={}", handler)
                } else {
                    String::new()
                };

                let src_attr = if let Some(bind) = img.bindings.get("src") {
                    format!("prop:src=move || {}.get()", bind)
                } else {
                    format!("src=\"{}\"", img.src)
                };

                let alt_attr = if let Some(bind) = img.bindings.get("alt") {
                    format!("prop:alt=move || {}.get()", bind)
                } else {
                    format!("alt=\"{}\"", img.alt)
                };

                output.push_str(&format!(
                    "{}        <img{}{} {} {} {}{}{}{} />\n",
                    indent,
                    id_attr,
                    class_attr,
                    src_attr,
                    alt_attr,
                    width_attr,
                    height_attr,
                    style_attr,
                    click_handler
                ));
            }
            CanvasComponent::Card(card) => {
                let shadow_style = if card.shadow {
                    "box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);"
                } else {
                    ""
                };
                let border_class = if card.border {
                    "border border-gray-200"
                } else {
                    ""
                };
                let anim_style = get_animation_css(&card.animation);

                let click_handler = if let Some(handler) = &card.on_click {
                    format!(" on:click={}", handler)
                } else {
                    String::new()
                };

                let id_attr = if let Some(bind) = card.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = card.bindings.get("custom_css_classes") {
                    format!(
                        " class=move || format!(\"card {} {{}}\", {}.get())",
                        border_class, bind
                    )
                } else {
                    format!(" class=\"card {}\"", border_class)
                };

                output.push_str(&format!(
                    "{}        <div{}{} style=\"padding: {}px; border-radius: {}px; {} {}\"{}>\n",
                    indent,
                    id_attr,
                    class_attr,
                    card.padding,
                    card.border_radius,
                    shadow_style,
                    anim_style,
                    click_handler
                ));
                for child in &card.children {
                    self.generate_component(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}        </div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                let id_attr = if let Some(bind) = custom.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = custom.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "{}        // Custom component: {}\n",
                    indent, custom.name
                ));
                output.push_str(&format!(
                    "{}        <div{}{}>{}        </div>\n",
                    indent, id_attr, class_attr, custom.template
                ));
            }
        }

        Ok(())
    }
}

impl CodeGenerator for LeptosCodeGenerator {
    fn generate(
        &self,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        let mut output = String::new();
        // Reset required signals for each generation
        self.required_signals.borrow_mut().clear();

        // Add imports
        output.push_str(&self.generate_imports());
        output.push('\n');

        // Generate component function
        output.push_str("#[component]\n");
        output.push_str("pub fn App() -> impl IntoView {\n");

        // We need to generate the body first to populate signals, then inject them
        // This requires buffering the view body
        let mut view_body = String::new();
        view_body.push_str("    view! {\n");
        for component in components {
            self.generate_component(component, &mut view_body, 0)?;
        }
        view_body.push_str("    }\n");

        // Now inject signals
        // First global variables
        if !variables.is_empty() {
            output.push_str("    // Global signals\n");
            for var in variables {
                let default_val = match var.data_type {
                    VariableType::String => format!("\"{}\".to_string()", var.default_value),
                    VariableType::Number => var.default_value.clone(),
                    VariableType::Boolean => var.default_value.clone(),
                };
                output.push_str(&format!(
                    "    let ({}, set_{}) = signal({});\n",
                    var.name, var.name, default_val
                ));
            }
            output.push('\n');
        }

        // Then local required signals
        let signals = self.required_signals.borrow();
        if !signals.is_empty() {
            output.push_str("    // Local signals\n");
            for (name, default) in signals.iter() {
                output.push_str(&format!(
                    "    let ({}, set_{}) = signal({});\n",
                    name, name, default
                ));
            }
            output.push('\n');
        }

        output.push_str(&view_body);
        output.push_str("}\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "rs"
    }
}

/// HTML code generator
pub struct HtmlCodeGenerator;

impl CodeGenerator for HtmlCodeGenerator {
    fn generate(
        &self,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        let mut output = String::from(
            "<!DOCTYPE html>\n<html>\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Generated Layout</title>\n</head>\n<body>\n",
        );

        for component in components {
            self.generate_html(component, &mut output, 1)?;
        }

        output.push_str("</body>\n</html>");
        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "html"
    }
}

impl HtmlCodeGenerator {
    #[allow(clippy::only_used_in_recursion)]
    fn generate_html(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "    ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                output.push_str(&format!(
                    "{}<button{}>{}</button>\n",
                    indent,
                    if btn.disabled { " disabled" } else { "" },
                    btn.label
                ));
            }
            CanvasComponent::Text(txt) => {
                let tag = match txt.tag {
                    crate::domain::TextTag::H1 => "h1",
                    crate::domain::TextTag::H2 => "h2",
                    crate::domain::TextTag::H3 => "h3",
                    crate::domain::TextTag::P => "p",
                    crate::domain::TextTag::Span => "span",
                };
                output.push_str(&format!("{}<{}>{}</{}>\n", indent, tag, txt.content, tag));
            }
            CanvasComponent::Input(inp) => {
                let input_type = match inp.input_type {
                    crate::domain::InputType::Text => "text",
                    crate::domain::InputType::Password => "password",
                    crate::domain::InputType::Email => "email",
                    crate::domain::InputType::Number => "number",
                    crate::domain::InputType::Tel => "tel",
                };
                output.push_str(&format!(
                    "{}<input type=\"{}\" placeholder=\"{}\"{}{}>\n",
                    indent,
                    input_type,
                    inp.placeholder,
                    if inp.required { " required" } else { "" },
                    if inp.disabled { " disabled" } else { "" }
                ));
            }
            CanvasComponent::Select(sel) => {
                output.push_str(&format!(
                    "{}<select{}>\n",
                    indent,
                    if sel.disabled { " disabled" } else { "" }
                ));
                if !sel.placeholder.is_empty() {
                    output.push_str(&format!(
                        "{}    <option value=\"\" disabled selected>{}</option>\n",
                        indent, sel.placeholder
                    ));
                }
                for option in sel.options.split(',') {
                    let opt = option.trim();
                    output.push_str(&format!(
                        "{}    <option value=\"{}\">{}</option>\n",
                        indent, opt, opt
                    ));
                }
                output.push_str(&format!("{}</select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                output.push_str(&format!("{}<div>\n", indent));
                for child in &container.children {
                    self.generate_html(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                let width_attr = img
                    .width
                    .as_ref()
                    .map_or(String::new(), |w| format!(" width=\"{}\"", w));
                let height_attr = img
                    .height
                    .as_ref()
                    .map_or(String::new(), |h| format!(" height=\"{}\"", h));
                output.push_str(&format!(
                    "{}<img src=\"{}\" alt=\"{}\"{}{} />\n",
                    indent, img.src, img.alt, width_attr, height_attr
                ));
            }
            CanvasComponent::Card(card) => {
                let shadow_style = if card.shadow {
                    "box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);"
                } else {
                    ""
                };
                let border_style = if card.border {
                    "border: 1px solid #e5e7eb;"
                } else {
                    ""
                };
                output.push_str(&format!(
                    "{}<div style=\"padding: {}px; border-radius: {}px; {} {}\">\n",
                    indent, card.padding, card.border_radius, shadow_style, border_style
                ));
                for child in &card.children {
                    self.generate_html(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!("{}<!-- {} -->\n", indent, custom.name));
                output.push_str(&format!("{}{}\n", indent, custom.template));
            }
        }

        Ok(())
    }
}

/// JSON code generator
pub struct JsonCodeGenerator;

impl CodeGenerator for JsonCodeGenerator {
    fn generate(
        &self,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        serde_json::to_string_pretty(components)
            .map_err(|e| AppError::Export(format!("Failed to serialize to JSON: {}", e)))
    }

    fn file_extension(&self) -> &str {
        "json"
    }
}

/// Markdown code generator (for documentation)
pub struct MarkdownCodeGenerator;

impl CodeGenerator for MarkdownCodeGenerator {
    fn generate(
        &self,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        let mut output = String::from("# Generated Layout Documentation\n\n");

        for (i, component) in components.iter().enumerate() {
            output.push_str(&format!("## Component {}\n\n", i + 1));
            self.generate_markdown(component, &mut output, 0)?;
            output.push('\n');
        }

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "md"
    }
}

impl MarkdownCodeGenerator {
    #[allow(clippy::only_used_in_recursion)]
    fn generate_markdown(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        depth: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(depth);

        match component {
            CanvasComponent::Button(btn) => {
                output.push_str(&format!("{}- **Button**: {}\n", indent, btn.label));
                output.push_str(&format!("{}  - Variant: {:?}\n", indent, btn.variant));
                output.push_str(&format!("{}  - Size: {:?}\n", indent, btn.size));
                output.push_str(&format!("{}  - Disabled: {}\n", indent, btn.disabled));
            }
            CanvasComponent::Text(txt) => {
                output.push_str(&format!("{}- **Text**: {}\n", indent, txt.content));
                output.push_str(&format!("{}  - Style: {:?}\n", indent, txt.style));
                output.push_str(&format!("{}  - Tag: {:?}\n", indent, txt.tag));
            }
            CanvasComponent::Input(inp) => {
                output.push_str(&format!("{}- **Input**\n", indent));
                output.push_str(&format!("{}  - Type: {:?}\n", indent, inp.input_type));
                output.push_str(&format!("{}  - Placeholder: {}\n", indent, inp.placeholder));
                output.push_str(&format!("{}  - Required: {}\n", indent, inp.required));
            }
            CanvasComponent::Select(sel) => {
                output.push_str(&format!("{}- **Select**\n", indent));
                output.push_str(&format!("{}  - Placeholder: {}\n", indent, sel.placeholder));
                output.push_str(&format!("{}  - Options: {}\n", indent, sel.options));
                output.push_str(&format!("{}  - Disabled: {}\n", indent, sel.disabled));
            }
            CanvasComponent::Container(container) => {
                output.push_str(&format!("{}- **Container**\n", indent));
                output.push_str(&format!("{}  - Layout: {:?}\n", indent, container.layout));
                output.push_str(&format!(
                    "{}  - Children: {}\n",
                    indent,
                    container.children.len()
                ));
                for child in &container.children {
                    self.generate_markdown(child, output, depth + 1)?;
                }
            }
            CanvasComponent::Image(img) => {
                output.push_str(&format!("{}- **Image**\n", indent));
                output.push_str(&format!("{}  - Src: {}\n", indent, img.src));
                output.push_str(&format!("{}  - Alt: {}\n", indent, img.alt));
            }
            CanvasComponent::Card(card) => {
                output.push_str(&format!("{}- **Card**\n", indent));
                output.push_str(&format!("{}  - Padding: {}\n", indent, card.padding));
                for child in &card.children {
                    self.generate_markdown(child, output, depth + 1)?;
                }
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!(
                    "{}- **Custom Component**: {}\n",
                    indent, custom.name
                ));
                output.push_str(&format!("{}  - Template: {}\n", indent, custom.template));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        ButtonComponent, CanvasComponent, InputComponent, InputType, TextComponent,
    };

    #[test]
    fn test_leptos_generator() {
        let generator = LeptosCodeGenerator::new(ExportPreset::Plain);
        let button = CanvasComponent::Button(ButtonComponent::new("Click me".to_string()));
        let code = generator.generate(&[button], &[]).unwrap();

        assert!(code.contains("use leptos::*;"));
        assert!(code.contains("#[component]"));
        assert!(code.contains("pub fn App()"));
        assert!(code.contains("Click me"));
        assert!(code.contains("on:click=")); // Check for new interactive code
    }

    #[test]
    fn test_leptos_input_generator() {
        let generator = LeptosCodeGenerator::new(ExportPreset::Plain);
        let variables = vec![];
        let input = CanvasComponent::Input(InputComponent {
            id: Default::default(),
            input_type: InputType::Text,
            placeholder: "Type here".to_string(),
            required: false,
            disabled: false,
            on_input: None,
            on_change: None,
            animation: None,
            bindings: Default::default(),
            style: Default::default(),
        });
        let code = generator.generate(&[input], &variables).unwrap();

        // Should generate a signal
        assert!(code.contains("let (input_0, set_input_0) = signal(String::new());"));
        // Should use the signal
        assert!(code.contains("prop:value=input_0"));
        assert!(code.contains("on:input=move |ev| set_input_0(event_target_value(&ev))"));
    }

    #[test]
    fn test_html_generator() {
        let generator = HtmlCodeGenerator;
        let text = CanvasComponent::Text(TextComponent::new("Hello World".to_string()));
        let code = generator.generate(&[text], &[]).unwrap();

        assert!(code.contains("<!DOCTYPE html>"));
        assert!(code.contains("<body>"));
        assert!(code.contains("Hello World"));
    }

    #[test]
    fn test_json_generator() {
        let generator = JsonCodeGenerator;
        let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
        let code = generator.generate(&[button], &[]).unwrap();

        assert!(code.contains("Button"));
        assert!(code.contains("Test"));
    }

    #[test]
    fn test_markdown_generator() {
        let generator = MarkdownCodeGenerator;
        let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
        let code = generator.generate(&[button], &[]).unwrap();

        assert!(code.contains("# Generated Layout Documentation"));
        assert!(code.contains("**Button**"));
        assert!(code.contains("Test"));
    }

    #[test]
    fn test_leptos_select_generator() {
        use crate::domain::SelectComponent;
        let generator = LeptosCodeGenerator::new(ExportPreset::Plain);
        let variables = vec![];
        let select = CanvasComponent::Select(SelectComponent {
            id: Default::default(),
            options: "A, B, C".to_string(),
            placeholder: "Choose".to_string(),
            disabled: false,
            on_change: None,
            animation: None,
            bindings: Default::default(),
            style: Default::default(),
        });
        let code = generator.generate(&[select], &variables).unwrap();

        assert!(code.contains("<select disabled=false  >"));
        assert!(code.contains("<option value=\"\" disabled selected>\"Choose\"</option>"));
        assert!(code.contains("<option value=\"A\">\"A\"</option>"));
        assert!(code.contains("<option value=\"B\">\"B\"</option>"));
    }

    #[test]
    fn test_html_select_generator() {
        use crate::domain::SelectComponent;
        let generator = HtmlCodeGenerator;
        let variables = vec![];
        let select = CanvasComponent::Select(SelectComponent {
            id: Default::default(),
            options: "X, Y".to_string(),
            placeholder: "Pick".to_string(),
            disabled: true,
            on_change: None,
            animation: None,
            bindings: Default::default(),
            style: Default::default(),
        });
        let code = generator.generate(&[select], &variables).unwrap();

        assert!(code.contains("<select disabled>"));
        assert!(code.contains("<option value=\"\" disabled selected>Pick</option>"));
        assert!(code.contains("<option value=\"X\">X</option>"));
    }

    #[test]
    fn test_markdown_select_generator() {
        use crate::domain::SelectComponent;
        let generator = MarkdownCodeGenerator;
        let variables = vec![];
        let select = CanvasComponent::Select(SelectComponent {
            id: Default::default(),
            options: "One, Two".to_string(),
            placeholder: "Select One".to_string(),
            disabled: false,
            on_change: None,
            animation: None,
            bindings: Default::default(),
            style: Default::default(),
        });
        let code = generator.generate(&[select], &variables).unwrap();

        assert!(code.contains("- **Select**"));
        assert!(code.contains("- Placeholder: Select One"));
        assert!(code.contains("- Options: One, Two"));
    }
}
