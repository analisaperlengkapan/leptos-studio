# Design Document - Leptos Studio Development Plan

## 1. Architecture Overview

### 1.1 High-Level Architecture
**Requirements:** [Req 1.1, 1.3.1]

Leptos Studio menggunakan browser-based WASM architecture dengan Leptos framework untuk reactive UI. Application adalah pure client-side tanpa backend dependency.

```
┌─────────────────────────────────────────────────────────┐
│                   Browser Environment                    │
│  ┌───────────────────────────────────────────────────┐  │
│  │              Leptos Application (WASM)             │  │
│  │                                                     │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ │  │
│  │  │ Builder  │  │  Canvas  │  │  Property Editor │ │  │
│  │  │ Sidebar  │  │  View    │  │                  │ │  │
│  │  └──────────┘  └──────────┘  └──────────────────┘ │  │
│  │                                                     │  │
│  │  ┌──────────────────────────────────────────────┐ │  │
│  │  │         State Management (Signals)           │ │  │
│  │  │  • Components   • Selected  • Undo/Redo     │ │  │
│  │  └──────────────────────────────────────────────┘ │  │
│  │                                                     │  │
│  │  ┌──────────────────────────────────────────────┐ │  │
│  │  │      NEW: Performance Layer                  │ │  │
│  │  │  • Virtual Scroll • Lazy Render • Memory Mgmt│ │  │
│  │  └──────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────┘  │
│                                                           │
│  ┌───────────────────────────────────────────────────┐  │
│  │          Browser APIs                              │  │
│  │  • localStorage • File API • Performance API       │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 1.2 System Components
**Requirements:** [Req 2.1, 4.1, 4.2, 4.3]

#### 1.2.1 Existing Modules (Production Ready)
- `app.rs` - Main application coordinator
- `canvas.rs` - Canvas rendering & drag-drop
- `sidebar.rs` - Component library & controls
- `property_editor.rs` - Property editing UI
- `export.rs` - Code generation engine
- `preview.rs` - Live preview panel
- `design_tokens.rs` - Design system
- `keyboard.rs` - Keyboard shortcuts
- `command_palette.rs` - Command search

#### 1.2.2 New Modules (To Be Implemented)
- `virtual_scroll.rs` - Virtual scrolling (Req 4.1.1)
- `lazy_renderer.rs` - Lazy canvas rendering (Req 4.1.2)
- `memory_manager.rs` - Memory optimization (Req 4.1.3)
- `breakpoint_manager.rs` - Responsive properties (Req 4.2.1)
- `media_query_gen.rs` - CSS generation (Req 4.2.2)
- `project_scaffold.rs` - Full project generation (Req 4.3.1)
- `code_formatter.rs` - Basic code formatting (Req 4.3.2)
- `onboarding.rs` - Tutorial system (Req 4.4.1)
- `accessibility.rs` - A11y utilities (Req 4.4.2)

#### 1.2.3 Modules to Remove/Refactor
- `git_panel.rs` - **REMOVE** (Req 2.4.1, 4.7.1 - not feasible)

### 1.3 Design Principles
**Requirements:** [Req 3.2, 6.1]

1. **Performance First**: Optimize for large projects (1000+ components)
2. **Browser Constraints**: Work within browser limitations (no filesystem, no shell)
3. **Backward Compatibility**: Existing saved layouts must work
4. **Progressive Enhancement**: Add features without breaking existing functionality
5. **Type Safety**: Leverage Rust's type system for correctness
6. **Reactive Architecture**: Use Leptos signals for state management

## 2. Data Models

### 2.1 Enhanced Canvas Component
**Requirements:** [Req 4.2.1]

#### 2.1.1 Current Data Model
```rust
// Existing (canvas.rs)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CanvasComponent {
    Button { label: String },
    Text { content: String },
    Input { placeholder: String },
    Container { children: Vec<CanvasComponent> },
    Custom { name: String },
}
```

#### 2.1.2 Extended for Responsive Styles
```rust
// NEW: Responsive style management
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ComponentStyle {
    pub padding: Option<String>,
    pub margin: Option<String>,
    pub font_size: Option<String>,
    pub display: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub color: Option<String>,
    pub background: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponsiveStyles {
    /// Base styles (mobile-first approach)
    pub base: ComponentStyle,
    /// Tablet overrides (min-width: 768px)
    pub tablet: Option<ComponentStyle>,
    /// Desktop overrides (min-width: 1024px)
    pub desktop: Option<ComponentStyle>,
}

impl Default for ResponsiveStyles {
    fn default() -> Self {
        Self {
            base: ComponentStyle::default(),
            tablet: None,
            desktop: None,
        }
    }
}

// Extended CanvasComponent (backward compatible)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CanvasComponent {
    Button { 
        label: String,
        #[serde(default)]
        styles: Option<ResponsiveStyles>,
    },
    Text { 
        content: String,
        #[serde(default)]
        styles: Option<ResponsiveStyles>,
    },
    Input { 
        placeholder: String,
        #[serde(default)]
        styles: Option<ResponsiveStyles>,
    },
    Container { 
        children: Vec<CanvasComponent>,
        #[serde(default)]
        styles: Option<ResponsiveStyles>,
    },
    Custom { 
        name: String,
        #[serde(default)]
        styles: Option<ResponsiveStyles>,
    },
}
```

**Migration Strategy:**
- Use `#[serde(default)]` untuk backward compatibility
- Old layouts tanpa styles field akan deserialize dengan `None`
- Gradual adoption: components tanpa styles menggunakan default styling

### 2.2 Virtual Scroll State
**Requirements:** [Req 4.1.1]

```rust
// virtual_scroll.rs
pub struct VirtualScrollState {
    /// All items in the list
    pub items: Vec<LibraryComponent>,
    /// Index of first visible item
    pub visible_start: usize,
    /// Number of visible items
    pub visible_count: usize,
    /// Estimated item height (pixels)
    pub item_height: f64,
    /// Buffer before/after visible range
    pub buffer_size: usize,
    /// Total scrollable height
    pub total_height: f64,
}

impl VirtualScrollState {
    pub fn new(items: Vec<LibraryComponent>) -> Self {
        let total_height = items.len() as f64 * 60.0; // 60px per item
        Self {
            items,
            visible_start: 0,
            visible_count: 10,
            item_height: 60.0,
            buffer_size: 5,
            total_height,
        }
    }
    
    /// Get items that should be rendered
    pub fn visible_items(&self) -> &[LibraryComponent] {
        let start = self.visible_start.saturating_sub(self.buffer_size);
        let end = (self.visible_start + self.visible_count + self.buffer_size)
            .min(self.items.len());
        &self.items[start..end]
    }
    
    /// Update visible range based on scroll position
    pub fn on_scroll(&mut self, scroll_top: f64, viewport_height: f64) {
        self.visible_start = (scroll_top / self.item_height).floor() as usize;
        self.visible_count = (viewport_height / self.item_height).ceil() as usize + 1;
    }
    
    /// Get offset for absolute positioning
    pub fn offset_top(&self) -> f64 {
        let start = self.visible_start.saturating_sub(self.buffer_size);
        start as f64 * self.item_height
    }
}
```

### 2.3 Lazy Render Queue
**Requirements:** [Req 4.1.2]

```rust
// lazy_renderer.rs
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderPriority {
    Immediate = 0, // Visible in viewport
    High = 1,      // Near viewport (within 200px)
    Medium = 2,    // Off-screen but top-level
    Low = 3,       // Nested or far from viewport
}

pub struct RenderTask {
    pub component_idx: usize,
    pub priority: RenderPriority,
    pub estimated_cost: u32, // Rendering complexity estimate
}

pub struct LazyRenderer {
    /// Queue of pending render tasks
    pending_queue: VecDeque<RenderTask>,
    /// Rendering budget per frame (milliseconds)
    budget_ms: f64,
    /// Currently rendered component indices
    rendered: Vec<usize>,
}

impl LazyRenderer {
    pub fn new() -> Self {
        Self {
            pending_queue: VecDeque::new(),
            budget_ms: 5.0, // 5ms per frame = 200 FPS budget
            rendered: Vec::new(),
        }
    }
    
    /// Schedule components for rendering
    pub fn schedule(&mut self, components: &[CanvasComponent], viewport: &Viewport) {
        self.pending_queue.clear();
        
        for (idx, comp) in components.iter().enumerate() {
            let priority = self.calculate_priority(comp, viewport, idx);
            let cost = self.estimate_cost(comp);
            self.pending_queue.push_back(RenderTask {
                component_idx: idx,
                priority,
                estimated_cost: cost,
            });
        }
        
        // Sort by priority (immediate first)
        let mut queue = self.pending_queue.make_contiguous();
        queue.sort_by_key(|t| t.priority);
    }
    
    /// Process render queue for one frame
    pub fn process_frame(&mut self) -> Vec<usize> {
        let start_time = web_sys::window()
            .unwrap()
            .performance()
            .unwrap()
            .now();
        
        let mut newly_rendered = Vec::new();
        
        while let Some(task) = self.pending_queue.pop_front() {
            newly_rendered.push(task.component_idx);
            
            let elapsed = web_sys::window()
                .unwrap()
                .performance()
                .unwrap()
                .now() - start_time;
            
            // Check budget
            if elapsed > self.budget_ms {
                break; // Yield to browser
            }
        }
        
        self.rendered.extend(&newly_rendered);
        newly_rendered
    }
    
    fn calculate_priority(&self, comp: &CanvasComponent, viewport: &Viewport, idx: usize) -> RenderPriority {
        // Simplified: top components are immediate, rest are low
        // Real implementation would check bounding boxes
        if idx < 10 {
            RenderPriority::Immediate
        } else if idx < 50 {
            RenderPriority::High
        } else {
            RenderPriority::Low
        }
    }
    
    fn estimate_cost(&self, comp: &CanvasComponent) -> u32 {
        match comp {
            CanvasComponent::Container { children, .. } => {
                1 + children.iter().map(|c| self.estimate_cost(c)).sum::<u32>()
            },
            CanvasComponent::Custom { .. } => 3, // Custom components more expensive
            _ => 1,
        }
    }
}

pub struct Viewport {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}
```

### 2.4 Memory Management
**Requirements:** [Req 4.1.3]

```rust
// memory_manager.rs
use std::collections::VecDeque;

pub struct MemoryManager {
    /// Maximum undo stack size
    max_undo_stack: usize,
    /// Maximum redo stack size
    max_redo_stack: usize,
    /// Memory usage threshold for warnings (bytes)
    warning_threshold: usize,
    /// localStorage size limit (bytes)
    storage_limit: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            max_undo_stack: 50,
            max_redo_stack: 50,
            warning_threshold: 200 * 1024 * 1024, // 200MB
            storage_limit: 5 * 1024 * 1024, // 5MB
        }
    }
    
    /// Enforce undo stack limit (LRU eviction)
    pub fn trim_undo_stack(&self, stack: &mut Vec<Vec<CanvasComponent>>) {
        if stack.len() > self.max_undo_stack {
            // Remove oldest entries
            stack.drain(0..(stack.len() - self.max_undo_stack));
        }
    }
    
    /// Check memory usage via Performance API
    pub fn check_memory_usage(&self) -> Option<usize> {
        // Use performance.memory API (Chrome only)
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let window = web_sys::window()?;
            let performance = window.performance()?;
            
            // Try to access memory property (non-standard)
            let memory = js_sys::Reflect::get(&performance, &"memory".into()).ok()?;
            let used_js_heap = js_sys::Reflect::get(&memory, &"usedJSHeapSize".into()).ok()?;
            
            used_js_heap.as_f64().map(|v| v as usize)
        }
        #[cfg(not(target_arch = "wasm32"))]
        None
    }
    
    /// Check localStorage usage
    pub fn check_storage_usage(&self) -> Result<usize, String> {
        let storage = web_sys::window()
            .ok_or("No window")?
            .local_storage()
            .map_err(|_| "No localStorage")?
            .ok_or("localStorage not available")?;
        
        let mut total_size = 0;
        let length = storage.length().unwrap_or(0);
        
        for i in 0..length {
            if let Ok(Some(key)) = storage.key(i) {
                if let Ok(Some(value)) = storage.get_item(&key) {
                    total_size += key.len() + value.len();
                }
            }
        }
        
        Ok(total_size)
    }
    
    /// Should show memory warning?
    pub fn should_warn(&self) -> bool {
        if let Some(usage) = self.check_memory_usage() {
            usage > self.warning_threshold
        } else {
            false
        }
    }
    
    /// Should warn about localStorage?
    pub fn should_warn_storage(&self) -> bool {
        if let Ok(usage) = self.check_storage_usage() {
            usage > self.storage_limit * 80 / 100 // Warn at 80%
        } else {
            false
        }
    }
}
```

## 3. Module Design

### 3.1 Virtual Scrolling Implementation
**Requirements:** [Req 4.1.1]  
**Design:** [Design 2.2]

#### 3.1.1 Module Responsibilities
- Manage visible item range based on scroll position
- Render only visible + buffer items
- Maintain scroll position during updates
- Handle dynamic item heights

#### 3.1.2 Component Interface
```rust
// virtual_scroll.rs
#[component]
pub fn VirtualScroll<T, F>(
    /// All items to virtualize
    items: RwSignal<Vec<T>>,
    /// Render function for each item
    render_item: F,
    /// Item height estimate (pixels)
    #[prop(default = 60.0)]
    item_height: f64,
    /// Buffer size (items before/after viewport)
    #[prop(default = 5)]
    buffer_size: usize,
) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T, usize) -> View + 'static,
{
    let scroll_state = create_rw_signal(VirtualScrollState::new(items.get()));
    let container_ref = create_node_ref::<html::Div>();
    
    // Update on scroll
    let on_scroll = move |_| {
        if let Some(container) = container_ref.get() {
            let scroll_top = container.scroll_top() as f64;
            let viewport_height = container.client_height() as f64;
            scroll_state.update(|state| state.on_scroll(scroll_top, viewport_height));
        }
    };
    
    view! {
        <div 
            node_ref=container_ref
            on:scroll=on_scroll
            style="overflow-y: auto; height: 100%;"
        >
            <div style=move || format!("height: {}px; position: relative;", scroll_state.get().total_height)>
                <div style=move || format!("position: absolute; top: {}px; width: 100%;", scroll_state.get().offset_top())>
                    {move || {
                        scroll_state.get().visible_items().iter().enumerate()
                            .map(|(idx, item)| render_item(item.clone(), idx))
                            .collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}
```

#### 3.1.3 Integration with Sidebar
Modify `sidebar.rs` to wrap component library list:
```rust
// In sidebar.rs
use crate::builder::virtual_scroll::VirtualScroll;

// Replace direct component rendering with:
view! {
    <VirtualScroll
        items=component_library
        item_height=60.0
        buffer_size=5
        render_item=move |comp: LibraryComponent, idx| {
            // Existing component rendering logic
            render_library_component(comp, idx)
        }
    />
}
```

### 3.2 Lazy Canvas Rendering
**Requirements:** [Req 4.1.2]  
**Design:** [Design 2.3]

#### 3.2.1 Module Responsibilities
- Progressive rendering for large layouts
- Prioritize visible viewport
- Maintain 60 FPS during rendering
- Show loading indicators

#### 3.2.2 Implementation Strategy
```rust
// lazy_renderer.rs
use leptos::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct LazyRenderContext {
    renderer: Rc<RefCell<LazyRenderer>>,
    raf_handle: Option<i32>,
}

impl LazyRenderContext {
    pub fn start_rendering(&mut self, components: Vec<CanvasComponent>) {
        // Schedule render tasks
        let viewport = self.get_viewport();
        self.renderer.borrow_mut().schedule(&components, &viewport);
        
        // Start RAF loop
        self.request_animation_frame();
    }
    
    fn request_animation_frame(&mut self) {
        let renderer = self.renderer.clone();
        
        let closure = Closure::wrap(Box::new(move || {
            let newly_rendered = renderer.borrow_mut().process_frame();
            
            if !newly_rendered.is_empty() {
                // Trigger Leptos reactivity to re-render
                // Queue next frame
                // request_animation_frame recursively
            }
        }) as Box<dyn FnMut()>);
        
        let handle = web_sys::window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
        
        closure.forget();
        self.raf_handle = Some(handle);
    }
    
    fn get_viewport(&self) -> Viewport {
        // Get canvas viewport from DOM
        Viewport {
            top: 0.0,
            bottom: 800.0,
            left: 0.0,
            right: 1200.0,
        }
    }
}
```

#### 3.2.3 Integration with Canvas
Modify `canvas.rs` untuk support lazy rendering:
```rust
// In canvas.rs
#[component]
pub fn Canvas(/* ...existing props... */) -> impl IntoView {
    let lazy_render_ctx = create_rw_signal(LazyRenderContext::new());
    let rendered_indices = create_rw_signal(Vec::<usize>::new());
    
    // Trigger lazy render on component change
    create_effect(move |_| {
        let comps = components.get();
        if comps.len() > 100 {
            // Use lazy rendering
            lazy_render_ctx.update(|ctx| ctx.start_rendering(comps));
        } else {
            // Render immediately
            rendered_indices.set((0..comps.len()).collect());
        }
    });
    
    view! {
        <div class="canvas">
            {move || {
                let indices = rendered_indices.get();
                let comps = components.get();
                indices.iter().map(|&idx| {
                    if let Some(comp) = comps.get(idx) {
                        render_component(comp.clone(), /* ... */)
                    } else {
                        view! { <div></div> }
                    }
                }).collect_view()
            }}
            
            // Loading indicator
            {move || {
                let ctx = lazy_render_ctx.get();
                if ctx.renderer.borrow().pending_queue.is_empty() {
                    view! { <div></div> }
                } else {
                    view! {
                        <div class="loading-overlay">
                            <span>"Rendering components..."</span>
                        </div>
                    }
                }
            }}
        </div>
    }
}
```

### 3.3 Breakpoint Management System
**Requirements:** [Req 4.2.1]  
**Design:** [Design 2.1.2]

#### 3.3.1 Module Responsibilities
- Manage breakpoint definitions
- Store/retrieve per-breakpoint properties
- Apply style cascade (desktop → tablet → mobile)
- Serialize/deserialize with backward compatibility

#### 3.3.2 Breakpoint Manager Implementation
```rust
// breakpoint_manager.rs
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    Mobile,
    Tablet,
    Desktop,
}

impl Breakpoint {
    pub fn min_width(&self) -> Option<u32> {
        match self {
            Breakpoint::Mobile => None,
            Breakpoint::Tablet => Some(768),
            Breakpoint::Desktop => Some(1024),
        }
    }
    
    pub fn max_width(&self) -> Option<u32> {
        match self {
            Breakpoint::Mobile => Some(767),
            Breakpoint::Tablet => Some(1023),
            Breakpoint::Desktop => None,
        }
    }
    
    pub fn css_name(&self) -> &'static str {
        match self {
            Breakpoint::Mobile => "mobile",
            Breakpoint::Tablet => "tablet",
            Breakpoint::Desktop => "desktop",
        }
    }
}

pub struct BreakpointManager {
    active_breakpoint: RwSignal<Breakpoint>,
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self {
            active_breakpoint: create_rw_signal(Breakpoint::Desktop),
        }
    }
    
    /// Get effective styles for current breakpoint
    pub fn get_effective_styles(&self, responsive: &ResponsiveStyles) -> ComponentStyle {
        let breakpoint = self.active_breakpoint.get();
        
        match breakpoint {
            Breakpoint::Mobile => {
                // Just base styles
                responsive.base.clone()
            },
            Breakpoint::Tablet => {
                // Base + tablet overrides
                let mut styles = responsive.base.clone();
                if let Some(tablet) = &responsive.tablet {
                    styles.merge(tablet);
                }
                styles
            },
            Breakpoint::Desktop => {
                // Base + tablet + desktop overrides
                let mut styles = responsive.base.clone();
                if let Some(tablet) = &responsive.tablet {
                    styles.merge(tablet);
                }
                if let Some(desktop) = &responsive.desktop {
                    styles.merge(desktop);
                }
                styles
            },
        }
    }
}

impl ComponentStyle {
    /// Merge another style, overriding with non-None values
    pub fn merge(&mut self, other: &ComponentStyle) {
        if other.padding.is_some() {
            self.padding = other.padding.clone();
        }
        if other.margin.is_some() {
            self.margin = other.margin.clone();
        }
        if other.font_size.is_some() {
            self.font_size = other.font_size.clone();
        }
        // ... for all fields
    }
}
```

#### 3.3.3 Property Editor Integration
Modify `property_editor.rs` untuk support breakpoint tabs:
```rust
// In property_editor.rs
#[component]
pub fn PropertyEditor(
    selected: RwSignal<SelectedComponent>,
    components: RwSignal<Vec<CanvasComponent>>,
    // ... existing props
) -> impl IntoView {
    let active_breakpoint = create_rw_signal(Breakpoint::Desktop);
    
    view! {
        <div class="property-editor">
            // Breakpoint tabs
            <div class="breakpoint-tabs">
                <button 
                    class:active=move || active_breakpoint.get() == Breakpoint::Desktop
                    on:click=move |_| active_breakpoint.set(Breakpoint::Desktop)
                >
                    "Desktop"
                </button>
                <button 
                    class:active=move || active_breakpoint.get() == Breakpoint::Tablet
                    on:click=move |_| active_breakpoint.set(Breakpoint::Tablet)
                >
                    "Tablet"
                </button>
                <button 
                    class:active=move || active_breakpoint.get() == Breakpoint::Mobile
                    on:click=move |_| active_breakpoint.set(Breakpoint::Mobile)
                >
                    "Mobile"
                </button>
            </div>
            
            // Property fields based on active breakpoint
            {move || {
                if let Some(idx) = selected.get().idx {
                    let comp = components.get()[idx].clone();
                    render_properties_for_breakpoint(comp, active_breakpoint.get())
                } else {
                    view! { <div>"No component selected"</div> }
                }
            }}
        </div>
    }
}
```

### 3.4 Media Query Generation
**Requirements:** [Req 4.2.2]  
**Design:** [Design 2.1.2]

#### 3.4.1 Module Responsibilities
- Generate CSS media queries from responsive styles
- Mobile-first approach (base styles + min-width queries)
- Clean, readable CSS output
- Integrate with existing export system

#### 3.4.2 Implementation
```rust
// media_query_gen.rs
pub fn generate_media_queries(components: &[CanvasComponent]) -> String {
    let mut css = String::new();
    
    // Helper to generate unique class names
    let mut class_counter = 0;
    let mut component_classes = std::collections::HashMap::new();
    
    // Generate base styles (mobile-first)
    css.push_str("/* Base Styles (Mobile-First) */\n");
    for (idx, comp) in components.iter().enumerate() {
        let class_name = format!("comp-{}", idx);
        component_classes.insert(idx, class_name.clone());
        
        if let Some(styles) = comp.get_responsive_styles() {
            css.push_str(&format!(".{} {{\n", class_name));
            css.push_str(&format_component_style(&styles.base, "  "));
            css.push_str("}\n\n");
        }
    }
    
    // Tablet overrides
    css.push_str("/* Tablet Styles (min-width: 768px) */\n");
    css.push_str("@media (min-width: 768px) {\n");
    
    for (idx, comp) in components.iter().enumerate() {
        if let Some(styles) = comp.get_responsive_styles() {
            if let Some(tablet) = &styles.tablet {
                let class_name = &component_classes[&idx];
                css.push_str(&format!("  .{} {{\n", class_name));
                css.push_str(&format_component_style(tablet, "    "));
                css.push_str("  }\n");
            }
        }
    }
    
    css.push_str("}\n\n");
    
    // Desktop overrides
    css.push_str("/* Desktop Styles (min-width: 1024px) */\n");
    css.push_str("@media (min-width: 1024px) {\n");
    
    for (idx, comp) in components.iter().enumerate() {
        if let Some(styles) = comp.get_responsive_styles() {
            if let Some(desktop) = &styles.desktop {
                let class_name = &component_classes[&idx];
                css.push_str(&format!("  .{} {{\n", class_name));
                css.push_str(&format_component_style(desktop, "    "));
                css.push_str("  }\n");
            }
        }
    }
    
    css.push_str("}\n");
    
    css
}

fn format_component_style(style: &ComponentStyle, indent: &str) -> String {
    let mut css = String::new();
    
    if let Some(padding) = &style.padding {
        css.push_str(&format!("{}padding: {};\n", indent, padding));
    }
    if let Some(margin) = &style.margin {
        css.push_str(&format!("{}margin: {};\n", indent, margin));
    }
    if let Some(font_size) = &style.font_size {
        css.push_str(&format!("{}font-size: {};\n", indent, font_size));
    }
    if let Some(display) = &style.display {
        css.push_str(&format!("{}display: {};\n", indent, display));
    }
    if let Some(width) = &style.width {
        css.push_str(&format!("{}width: {};\n", indent, width));
    }
    if let Some(height) = &style.height {
        css.push_str(&format!("{}height: {};\n", indent, height));
    }
    
    css
}

impl CanvasComponent {
    pub fn get_responsive_styles(&self) -> Option<&ResponsiveStyles> {
        match self {
            CanvasComponent::Button { styles, .. } => styles.as_ref(),
            CanvasComponent::Text { styles, .. } => styles.as_ref(),
            CanvasComponent::Input { styles, .. } => styles.as_ref(),
            CanvasComponent::Container { styles, .. } => styles.as_ref(),
            CanvasComponent::Custom { styles, .. } => styles.as_ref(),
        }
    }
}
```

#### 3.4.3 Integration with Export System
Modify `export.rs` untuk include media queries:
```rust
// In export.rs
use crate::builder::media_query_gen::generate_media_queries;

pub fn generate_leptos_code_with_responsive(
    components: &[CanvasComponent],
    custom_components: &[LibraryComponent],
    preset: ExportPreset,
) -> String {
    let mut code = String::new();
    
    // Generate component code
    code.push_str(&generate_leptos_code(components, custom_components, preset));
    
    // Add responsive CSS
    code.push_str("\n\n// Responsive Styles\n");
    code.push_str("<style>\n");
    code.push_str(&generate_media_queries(components));
    code.push_str("</style>\n");
    
    code
}
```

### 3.5 Project Scaffolding System
**Requirements:** [Req 4.3.1]  
**Design:** [Design 1.2.2]

#### 3.5.1 Module Responsibilities
- Generate complete Leptos project structure
- Create Cargo.toml with dependencies
- Generate main.rs entry point
- Create Trunk.toml build config
- Package as downloadable ZIP

#### 3.5.2 Project Template Engine
```rust
// project_scaffold.rs
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub struct ProjectScaffold {
    pub name: String,
    pub components: Vec<CanvasComponent>,
    pub custom_components: Vec<LibraryComponent>,
    pub preset: ExportPreset,
}

impl ProjectScaffold {
    pub fn generate_files(&self) -> Vec<ProjectFile> {
        vec![
            self.generate_cargo_toml(),
            self.generate_main_rs(),
            self.generate_trunk_toml(),
            self.generate_readme(),
            self.generate_gitignore(),
            self.generate_index_html(),
        ]
    }
    
    fn generate_cargo_toml(&self) -> ProjectFile {
        let dependencies = self.detect_dependencies();
        let content = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.6"
{}

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
"#,
            self.name,
            dependencies.join("\n")
        );
        
        ProjectFile {
            path: "Cargo.toml".to_string(),
            content,
        }
    }
    
    fn detect_dependencies(&self) -> Vec<String> {
        let mut deps = Vec::new();
        
        match self.preset {
            ExportPreset::ThawUi => {
                deps.push("thaw = \"0.2\"".to_string());
            },
            ExportPreset::LeptosMaterial => {
                deps.push("leptos-material = \"0.1\"".to_string());
            },
            ExportPreset::LeptosUse => {
                deps.push("leptos-use = \"0.10\"".to_string());
            },
            _ => {},
        }
        
        deps
    }
    
    fn generate_main_rs(&self) -> ProjectFile {
        let component_code = generate_leptos_code_with_responsive(
            &self.components,
            &self.custom_components,
            self.preset,
        );
        
        let content = format!(
            r#"use leptos::*;

#[component]
fn App() -> impl IntoView {{
    {}
}}

fn main() {{
    leptos::mount_to_body(|| view! {{ <App /> }});
}}
"#,
            component_code
        );
        
        ProjectFile {
            path: "src/main.rs".to_string(),
            content,
        }
    }
    
    fn generate_trunk_toml(&self) -> ProjectFile {
        let content = r#"[build]
target = "index.html"

[watch]
ignore = ["dist"]

[serve]
address = "127.0.0.1"
port = 8080
"#.to_string();
        
        ProjectFile {
            path: "Trunk.toml".to_string(),
            content,
        }
    }
    
    fn generate_readme(&self) -> ProjectFile {
        let content = format!(
            r#"# {}

Generated by Leptos Studio.

## Development

```bash
trunk serve
```

Open http://localhost:8080

## Build

```bash
trunk build --release
```

Output in `dist/` directory.
"#,
            self.name
        );
        
        ProjectFile {
            path: "README.md".to_string(),
            content,
        }
    }
    
    fn generate_gitignore(&self) -> ProjectFile {
        let content = r#"/target
/dist
Cargo.lock
.DS_Store
"#.to_string();
        
        ProjectFile {
            path: ".gitignore".to_string(),
            content,
        }
    }
    
    fn generate_index_html(&self) -> ProjectFile {
        let content = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link data-trunk rel="rust" />
</head>
<body>
</body>
</html>
"#,
            self.name
        );
        
        ProjectFile {
            path: "index.html".to_string(),
            content,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProjectFile {
    pub path: String,
    pub content: String,
}
```

#### 3.5.3 ZIP Export Implementation
```rust
// project_scaffold.rs (continued)
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, Url};

pub async fn export_project_as_zip(scaffold: ProjectScaffold) -> Result<(), JsValue> {
    let files = scaffold.generate_files();
    
    // Use JSZip wasm binding (via npm package)
    let zip = js_sys::eval("new JSZip()")?;
    
    for file in files {
        // Add file to ZIP
        let js_zip = zip.dyn_ref::<js_sys::Object>().unwrap();
        js_sys::Reflect::get(js_zip, &"file".into())?
            .dyn_into::<js_sys::Function>()?
            .call2(&zip, &file.path.into(), &file.content.into())?;
    }
    
    // Generate ZIP blob
    let generate_async = js_sys::Reflect::get(&zip, &"generateAsync".into())?
        .dyn_into::<js_sys::Function>()?;
    
    let options = js_sys::Object::new();
    js_sys::Reflect::set(&options, &"type".into(), &"blob".into())?;
    
    let promise = generate_async.call1(&zip, &options)?
        .dyn_into::<js_sys::Promise>()?;
    
    let blob_value = wasm_bindgen_futures::JsFuture::from(promise).await?;
    let blob = blob_value.dyn_into::<Blob>()?;
    
    // Trigger download
    let url = Url::create_object_url_with_blob(&blob)?;
    let document = web_sys::window().unwrap().document().unwrap();
    let anchor = document.create_element("a")?.dyn_into::<web_sys::HtmlAnchorElement>()?;
    
    anchor.set_href(&url);
    anchor.set_download(&format!("{}.zip", scaffold.name));
    anchor.click();
    
    Url::revoke_object_url(&url)?;
    
    Ok(())
}
```

### 3.6 Code Formatter
**Requirements:** [Req 4.3.2]

#### 3.6.1 Basic Formatting Rules
```rust
// code_formatter.rs
pub struct CodeFormatter {
    indent_size: usize,
    use_tabs: bool,
}

impl CodeFormatter {
    pub fn new() -> Self {
        Self {
            indent_size: 4,
            use_tabs: false,
        }
    }
    
    pub fn format_rust_code(&self, code: &str) -> String {
        let mut formatted = String::new();
        let mut indent_level = 0;
        let mut in_string = false;
        
        for line in code.lines() {
            let trimmed = line.trim();
            
            // Skip empty lines
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }
            
            // Decrease indent for closing braces
            if trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')') {
                indent_level = indent_level.saturating_sub(1);
            }
            
            // Add indentation
            formatted.push_str(&self.make_indent(indent_level));
            formatted.push_str(trimmed);
            formatted.push('\n');
            
            // Increase indent for opening braces
            if trimmed.ends_with('{') || trimmed.ends_with('[') || trimmed.ends_with('(') {
                indent_level += 1;
            }
        }
        
        formatted
    }
    
    fn make_indent(&self, level: usize) -> String {
        if self.use_tabs {
            "\t".repeat(level)
        } else {
            " ".repeat(level * self.indent_size)
        }
    }
    
    pub fn clean_imports(&self, code: &str) -> String {
        // Remove duplicate imports
        // Sort imports alphabetically
        // Remove unused imports (basic heuristic)
        
        let mut lines: Vec<&str> = code.lines().collect();
        let mut import_lines = Vec::new();
        let mut other_lines = Vec::new();
        
        for line in lines {
            if line.trim().starts_with("use ") {
                import_lines.push(line);
            } else {
                other_lines.push(line);
            }
        }
        
        // Deduplicate and sort
        import_lines.sort();
        import_lines.dedup();
        
        let mut result = String::new();
        for line in import_lines {
            result.push_str(line);
            result.push('\n');
        }
        
        if !import_lines.is_empty() && !other_lines.is_empty() {
            result.push('\n'); // Blank line after imports
        }
        
        for line in other_lines {
            result.push_str(line);
            result.push('\n');
        }
        
        result
    }
}
```

### 3.7 Onboarding System
**Requirements:** [Req 4.4.1]

#### 3.7.1 Tutorial Steps
```rust
// onboarding.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TutorialStep {
    Welcome,
    DragComponent,
    EditProperties,
    ExportCode,
    Complete,
}

pub struct OnboardingState {
    current_step: RwSignal<TutorialStep>,
    completed: RwSignal<bool>,
    skipped: RwSignal<bool>,
}

impl OnboardingState {
    pub fn new() -> Self {
        // Check if tutorial already completed
        let completed = Self::load_completion_status();
        
        Self {
            current_step: create_rw_signal(TutorialStep::Welcome),
            completed: create_rw_signal(completed),
            skipped: create_rw_signal(false),
        }
    }
    
    pub fn next_step(&self) {
        self.current_step.update(|step| {
            *step = match step {
                TutorialStep::Welcome => TutorialStep::DragComponent,
                TutorialStep::DragComponent => TutorialStep::EditProperties,
                TutorialStep::EditProperties => TutorialStep::ExportCode,
                TutorialStep::ExportCode => TutorialStep::Complete,
                TutorialStep::Complete => TutorialStep::Complete,
            };
        });
        
        if self.current_step.get() == TutorialStep::Complete {
            self.mark_completed();
        }
    }
    
    pub fn skip_tutorial(&self) {
        self.skipped.set(true);
        self.mark_completed();
    }
    
    fn mark_completed(&self) {
        self.completed.set(true);
        Self::save_completion_status();
    }
    
    fn load_completion_status() -> bool {
        web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .and_then(|storage| storage.get_item("leptos_studio_tutorial_completed").ok())
            .flatten()
            .map(|v| v == "true")
            .unwrap_or(false)
    }
    
    fn save_completion_status() {
        if let Some(storage) = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            let _ = storage.set_item("leptos_studio_tutorial_completed", "true");
        }
    }
}

#[component]
pub fn OnboardingOverlay(state: OnboardingState) -> impl IntoView {
    let should_show = create_memo(move |_| {
        !state.completed.get() && !state.skipped.get()
    });
    
    view! {
        {move || if should_show.get() {
            let step = state.current_step.get();
            view! {
                <div class="onboarding-overlay">
                    <div class="onboarding-modal">
                        {match step {
                            TutorialStep::Welcome => view! {
                                <div>
                                    <h2>"Welcome to Leptos Studio!"</h2>
                                    <p>"Build Leptos UIs visually with drag & drop."</p>
                                    <button on:click=move |_| state.next_step()>"Start Tutorial"</button>
                                    <button on:click=move |_| state.skip_tutorial()>"Skip"</button>
                                </div>
                            }.into_any(),
                            TutorialStep::DragComponent => view! {
                                <div class="tutorial-highlight sidebar">
                                    <h3>"Step 1: Drag a Component"</h3>
                                    <p>"Try dragging a Button from the sidebar to the canvas."</p>
                                    <button on:click=move |_| state.next_step()>"Next"</button>
                                </div>
                            }.into_any(),
                            TutorialStep::EditProperties => view! {
                                <div class="tutorial-highlight property-editor">
                                    <h3>"Step 2: Edit Properties"</h3>
                                    <p>"Click a component and edit its properties here."</p>
                                    <button on:click=move |_| state.next_step()>"Next"</button>
                                </div>
                            }.into_any(),
                            TutorialStep::ExportCode => view! {
                                <div class="tutorial-highlight export-button">
                                    <h3>"Step 3: Export Code"</h3>
                                    <p>"Click Export to generate Leptos code!"</p>
                                    <button on:click=move |_| state.next_step()>"Finish"</button>
                                </div>
                            }.into_any(),
                            TutorialStep::Complete => view! {
                                <div>
                                    <h2>"🎉 Tutorial Complete!"</h2>
                                    <p>"You're ready to build amazing UIs."</p>
                                    <button on:click=move |_| state.mark_completed()>"Let's Go!"</button>
                                </div>
                            }.into_any(),
                        }}
                    </div>
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }}
    }
}
```

## 4. Technology Stack & Rationale

### 4.1 Core Technologies
**Requirements:** [Req 5.7.2, 6.1.4]

| Technology | Version | Purpose | Rationale |
|------------|---------|---------|-----------|
| Leptos | 0.6+ | UI Framework | Reactive, type-safe, excellent WASM support |
| Rust | 1.70+ | Language | Memory safety, performance, type system |
| WASM | wasm32-unknown-unknown | Runtime | Browser execution, near-native performance |
| Trunk | 0.18+ | Build Tool | Leptos-specific bundler, hot reload support |
| serde/serde_json | 1.0+ | Serialization | Layout persistence, import/export |
| web-sys | 0.3+ | Web APIs | Browser API bindings |

### 4.2 New Dependencies
**Requirements:** [Req 4.1.4, 4.3.1]

| Dependency | Purpose | Justification |
|------------|---------|---------------|
| js-sys | JavaScript interop | Access browser APIs not in web-sys |
| wasm-bindgen-futures | Async support | Promise integration for async operations |
| JSZip (via npm) | ZIP generation | Project scaffolding export |
| wasm-opt (toolchain) | Bundle optimization | Reduce WASM size |

### 4.3 Browser APIs Used
**Requirements:** [Req 6.1.1]

- **Performance API**: Memory monitoring, timing measurements
- **IntersectionObserver API**: Virtual scrolling, lazy rendering
- **RequestAnimationFrame API**: Smooth rendering, 60 FPS target
- **File API**: Project import/export
- **localStorage API**: State persistence
- **Blob/URL APIs**: File download

## 5. Performance Targets & Benchmarks

### 5.1 Performance Budgets
**Requirements:** [Req 5.1.1, 5.1.2, 5.1.3]

| Metric | Current | Target | Priority |
|--------|---------|--------|----------|
| Initial Load | ~2.5s | <3s | Maintain |
| Interaction Latency | <100ms | <100ms | Maintain |
| 60 FPS (100 comp) | ✅ | ✅ | Maintain |
| 60 FPS (1000 comp) | ❌ | ✅ | HIGH |
| Memory (100 comp) | ~100MB | <150MB | Maintain |
| Memory (1000 comp) | Unknown | <300MB | HIGH |
| WASM Bundle | ~1.8MB | <2MB | Maintain |
| localStorage Usage | ~50KB | <5MB | Monitor |

### 5.2 Optimization Strategies
**Requirements:** [Req 4.1]

1. **Virtual Scrolling**: Reduce DOM nodes for large lists
2. **Lazy Rendering**: Progressive canvas rendering
3. **Memory Management**: LRU cache, weak references
4. **Code Splitting**: Lazy load optional features
5. **Bundle Optimization**: wasm-opt, tree-shaking

## 6. Migration & Backward Compatibility

### 6.1 Data Migration Strategy
**Requirements:** [Req 5.5.2, 6.3]

#### 6.1.1 Backward Compatible Changes
- New fields with `#[serde(default)]` attribute
- Optional responsive styles: `Option<ResponsiveStyles>`
- Existing layouts deserialize successfully with default values

#### 6.1.2 Layout Version Detection
```rust
#[derive(Serialize, Deserialize)]
pub struct SavedLayout {
    #[serde(default)]
    pub version: u32, // 0 = old format, 1 = with responsive styles
    pub components: Vec<CanvasComponent>,
    #[serde(default)]
    pub custom_components: Vec<LibraryComponent>,
}

impl SavedLayout {
    pub fn load_from_storage() -> Result<Self, String> {
        // Load from localStorage
        // Detect version
        // Migrate if needed
    }
    
    fn migrate_v0_to_v1(mut self) -> Self {
        // Add default responsive styles if missing
        self.version = 1;
        self
    }
}
```

## 7. Testing Strategy

### 7.1 Unit Tests
**Requirements:** [Req 5.6.1, 7.6.1]

- **VirtualScrollState**: Test visible range calculations
- **LazyRenderer**: Test priority calculation, frame budget
- **BreakpointManager**: Test style cascade logic
- **MediaQueryGen**: Test CSS generation
- **ProjectScaffold**: Test file generation

### 7.2 Integration Tests
**Requirements:** [Req 5.6.1, 7.6.1]

- **Virtual Scroll + Sidebar**: Test with 1000+ components
- **Lazy Render + Canvas**: Test progressive rendering
- **Responsive + Export**: Test media query in generated code
- **Project Scaffold + ZIP**: Test full project export

### 7.3 Performance Tests
**Requirements:** [Req 5.1, 7.8.1]

- Benchmark virtual scrolling performance
- Measure lazy rendering latency
- Profile memory usage for large projects
- Bundle size regression tests

## 8. Security Considerations

### 8.1 Input Validation
**Requirements:** [Req 5.2.1]

- **Component Names**: Already validated (Rust identifier rules)
- **Templates**: HTML validation present
- **User Styles**: CSS injection prevention via escaping
- **localStorage**: Validate on load, handle corrupt data

### 8.2 Content Security Policy
**Requirements:** [Req 5.2.1]

Recommended CSP headers for deployment:
```
Content-Security-Policy: 
  default-src 'self';
  script-src 'self' 'wasm-unsafe-eval';
  style-src 'self' 'unsafe-inline';
  img-src 'self' data: blob:;
```

## 9. Documentation Updates Required

### 9.1 Code Documentation
**Requirements:** [Req 7.1.1, 7.6.2]

- Add rustdoc comments untuk all new public APIs
- Update API.md with new modules
- Document performance characteristics
- Add usage examples

### 9.2 User Documentation
**Requirements:** [Req 7.1.1, 6.4.2]

**CRITICAL FIXES:**
1. Remove "hot reload" terminology or clarify as "reactive editing"
2. Remove Git integration claims or document backend requirement
3. Add "Browser Limitations" section to README
4. Update feature checklist to reflect actual status
5. Document breakpoint-specific properties usage
6. Add project scaffolding guide

### 9.3 Migration Guide
**Requirements:** [Req 7.6.2]

Create migration guide untuk users dengan existing layouts:
- Explain backward compatibility
- Document new features (responsive styles)
- Provide upgrade path
- Include troubleshooting section

---

**Document Version:** 1.0  
**Last Updated:** 2025-11-04  
**Status:** Draft - Ready for Implementation  
**References**: requirements.md v2.0
