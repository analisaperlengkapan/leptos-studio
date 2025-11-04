# Tasks Document - Leptos Studio Development Plan

## 1. Documentation & Critical Fixes

### 1.1 Fix Misleading Documentation
**Requirements:** [Req 6.4.1, 6.4.2, 7.1.1]  
**Design:** [Design 9.2]

- [ ] 1.1.1 Update README.md - Remove or clarify "hot reload" claim (change to "reactive editing")
- [ ] 1.1.2 Update README.md - Remove Git integration claims OR add "Requires Backend" warning
- [ ] 1.1.3 Add "Browser Limitations" section to README explaining no filesystem/shell access
- [ ] 1.1.4 Update .github/copilot-instructions.md checklist to reflect actual implementation status
- [ ] 1.1.5 Add performance optimization notes to documentation

**Priority:** HIGH | **Est:** 2 hours  
**Dependencies:** None  
**Acceptance Criteria:**
  - README accurately reflects current capabilities
  - No misleading claims about unimplemented features
  - Browser limitations clearly documented
  - Checklist shows correct status (17 implemented features)

### 1.2 Remove/Refactor Git Panel
**Requirements:** [Req 2.4.1, 4.7.1]  
**Design:** [Design 1.2.3]

- [ ] 1.2.1 Remove `git_panel.rs` module from codebase
- [ ] 1.2.2 Remove GitPanel import/usage from `app.rs`
- [ ] 1.2.3 Remove git_panel from mod.rs exports
- [ ] 1.2.4 Update UI to remove Git panel UI elements
- [ ] 1.2.5 Update tests that reference git_panel

**Priority:** HIGH | **Est:** 1.5 hours  
**Dependencies:** 1.1  
**Acceptance Criteria:**
  - No compilation errors after removal
  - All tests pass
  - UI renders without git panel
  - No dead code related to git integration

**Technical Notes:**
  - Check for any context/signals shared with git_panel
  - Ensure no localStorage keys left behind

## 2. Performance Optimization - Virtual Scrolling

### 2.1 Implement VirtualScrollState
**Requirements:** [Req 4.1.1]  
**Design:** [Design 2.2, 3.1]

- [ ] 2.1.1 Create `src/builder/virtual_scroll.rs` file
- [ ] 2.1.2 Implement `VirtualScrollState` struct with fields (items, visible_start, visible_count, etc.)
- [ ] 2.1.3 Implement `VirtualScrollState::new()` constructor
- [ ] 2.1.4 Implement `visible_items()` method with buffer calculation
- [ ] 2.1.5 Implement `on_scroll()` method for scroll position updates
- [ ] 2.1.6 Implement `offset_top()` for absolute positioning

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 1.2  
**Acceptance Criteria:**
  - VirtualScrollState calculates visible range correctly
  - Buffer items included before/after visible range
  - Scroll position updates visible_start accurately
  - Offset calculation matches rendered position

**Technical Notes:**
  - Use `saturating_sub()` to prevent underflow
  - Item height default: 60px
  - Buffer size default: 5 items

### 2.2 Create VirtualScroll Component
**Requirements:** [Req 4.1.1]  
**Design:** [Design 3.1.2]

- [ ] 2.2.1 Implement generic `VirtualScroll<T, F>` component
- [ ] 2.2.2 Add props: items signal, render_item closure, item_height, buffer_size
- [ ] 2.2.3 Create scroll_state RwSignal for state management
- [ ] 2.2.4 Create container_ref for DOM access
- [ ] 2.2.5 Implement on_scroll event handler
- [ ] 2.2.6 Render absolute positioned container with calculated height
- [ ] 2.2.7 Render only visible items with proper offset

**Priority:** HIGH | **Est:** 4 hours  
**Dependencies:** 2.1  
**Acceptance Criteria:**
  - Component renders only visible + buffer items
  - Smooth scrolling without jumps
  - Scroll position maintained during updates
  - Works with generic item types

### 2.3 Integrate VirtualScroll with Sidebar
**Requirements:** [Req 4.1.1]  
**Design:** [Design 3.1.3]

- [ ] 2.3.1 Update `sidebar.rs` to import VirtualScroll
- [ ] 2.3.2 Wrap component library list with VirtualScroll component
- [ ] 2.3.3 Pass component_library signal to VirtualScroll
- [ ] 2.3.4 Implement render_item closure for LibraryComponent
- [ ] 2.3.5 Test with 100+ components in library
- [ ] 2.3.6 Test with 1000+ components for performance validation

**Priority:** HIGH | **Est:** 2 hours  
**Dependencies:** 2.2  
**Acceptance Criteria:**
  - Sidebar component library uses virtual scrolling
  - Performance improved for large component lists
  - All existing drag-drop functionality preserved
  - No visual glitches during scroll

### 2.4 Unit Tests for Virtual Scrolling
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.1]

- [ ] 2.4.1 Test VirtualScrollState::visible_items() with various ranges
- [ ] 2.4.2 Test on_scroll() updates visible_start correctly
- [ ] 2.4.3 Test buffer calculation at list boundaries (start/end)
- [ ] 2.4.4 Test offset_top() calculation
- [ ] 2.4.5 Test with empty list edge case
- [ ] 2.4.6 Test with single item edge case

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** 2.1  
**Acceptance Criteria:**
  - All unit tests pass
  - Edge cases covered (empty, single item, boundaries)
  - Test coverage >80% for virtual_scroll module

## 3. Performance Optimization - Lazy Rendering

### 3.1 Implement LazyRenderer Core
**Requirements:** [Req 4.1.2]  
**Design:** [Design 2.3, 3.2]

- [ ] 3.1.1 Create `src/builder/lazy_renderer.rs` file
- [ ] 3.1.2 Implement `RenderPriority` enum (Immediate, High, Medium, Low)
- [ ] 3.1.3 Implement `RenderTask` struct with component_idx, priority, estimated_cost
- [ ] 3.1.4 Implement `LazyRenderer` struct with pending_queue, budget_ms, rendered
- [ ] 3.1.5 Implement `LazyRenderer::new()` with 5ms frame budget
- [ ] 3.1.6 Implement `schedule()` method to build render queue
- [ ] 3.1.7 Implement `calculate_priority()` based on viewport proximity
- [ ] 3.1.8 Implement `estimate_cost()` for rendering complexity

**Priority:** HIGH | **Est:** 4 hours  
**Dependencies:** 2.3  
**Acceptance Criteria:**
  - LazyRenderer queues tasks by priority
  - Priority calculation considers viewport
  - Cost estimation accounts for nested components
  - Queue sorted correctly (Immediate first)

**Technical Notes:**
  - Use VecDeque for efficient queue operations
  - Immediate priority for visible viewport
  - Custom components cost more (complexity factor 3)

### 3.2 Implement Frame Processing
**Requirements:** [Req 4.1.2]  
**Design:** [Design 2.3, 3.2.2]

- [ ] 3.2.1 Implement `process_frame()` method
- [ ] 3.2.2 Use Performance API to measure elapsed time
- [ ] 3.2.3 Process tasks until budget exhausted (5ms)
- [ ] 3.2.4 Return list of newly rendered component indices
- [ ] 3.2.5 Track rendered components to avoid re-rendering
- [ ] 3.2.6 Handle empty queue gracefully

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 3.1  
**Acceptance Criteria:**
  - Frame processing respects time budget
  - Yields to browser after 5ms
  - Returns correct indices of rendered components
  - No duplicate rendering

### 3.3 Implement LazyRenderContext
**Requirements:** [Req 4.1.2]  
**Design:** [Design 3.2.2]

- [ ] 3.3.1 Implement `LazyRenderContext` struct with renderer Rc<RefCell>
- [ ] 3.3.2 Implement `start_rendering()` method
- [ ] 3.3.3 Implement `request_animation_frame()` loop
- [ ] 3.3.4 Implement `get_viewport()` from canvas DOM element
- [ ] 3.3.5 Add proper Closure handling for RAF callback
- [ ] 3.3.6 Implement cancellation via raf_handle

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 3.2  
**Acceptance Criteria:**
  - RAF loop runs until queue empty
  - Viewport detected from canvas bounds
  - Memory-safe closure handling
  - Can cancel rendering if needed

### 3.4 Integrate LazyRenderer with Canvas
**Requirements:** [Req 4.1.2]  
**Design:** [Design 3.2.3]

- [ ] 3.4.1 Update `canvas.rs` to import LazyRenderContext
- [ ] 3.4.2 Create lazy_render_ctx signal in Canvas component
- [ ] 3.4.3 Create rendered_indices signal
- [ ] 3.4.4 Add create_effect to trigger lazy rendering on component change
- [ ] 3.4.5 Add threshold check (>100 components = lazy, else immediate)
- [ ] 3.4.6 Render only components in rendered_indices
- [ ] 3.4.7 Add loading overlay UI during progressive rendering

**Priority:** HIGH | **Est:** 4 hours  
**Dependencies:** 3.3  
**Acceptance Criteria:**
  - Canvas uses lazy rendering for >100 components
  - Loading indicator shown during rendering
  - All components eventually rendered
  - Maintains 60 FPS during rendering

### 3.5 Unit Tests for Lazy Rendering
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.1]

- [ ] 3.5.1 Test RenderTask priority ordering
- [ ] 3.5.2 Test LazyRenderer::schedule() builds correct queue
- [ ] 3.5.3 Test process_frame() respects time budget
- [ ] 3.5.4 Test calculate_priority() for different viewport positions
- [ ] 3.5.5 Test estimate_cost() for nested components
- [ ] 3.5.6 Test with 1000+ component canvas

**Priority:** MEDIUM | **Est:** 3 hours  
**Dependencies:** 3.2  
**Acceptance Criteria:**
  - All unit tests pass
  - Frame budget enforcement verified
  - Priority calculation tested with mock viewport
  - Performance validated with large datasets

## 4. Performance Optimization - Memory Management

### 4.1 Implement MemoryManager
**Requirements:** [Req 4.1.3, 4.1.4]  
**Design:** [Design 2.4]

- [ ] 4.1.1 Create `src/builder/memory_manager.rs` file
- [ ] 4.1.2 Implement `MemoryManager` struct with config fields
- [ ] 4.1.3 Implement `new()` with defaults (50 undo stack, 200MB threshold)
- [ ] 4.1.4 Implement `trim_undo_stack()` for LRU eviction
- [ ] 4.1.5 Implement `check_memory_usage()` via Performance API
- [ ] 4.1.6 Implement `check_storage_usage()` for localStorage
- [ ] 4.1.7 Implement `should_warn()` and `should_warn_storage()`

**Priority:** MEDIUM | **Est:** 3 hours  
**Dependencies:** None  
**Acceptance Criteria:**
  - Undo stack limited to 50 entries
  - Memory usage detected (Chrome only, graceful fallback)
  - localStorage usage calculated correctly
  - Warning thresholds enforced (200MB, 80% storage)

**Technical Notes:**
  - Performance.memory is Chrome-specific (use feature detection)
  - Fallback to localStorage size calculation for other browsers
  - LRU eviction removes oldest undo states

### 4.2 Integrate MemoryManager with Undo System
**Requirements:** [Req 4.1.3]  
**Design:** [Design 2.4]

- [ ] 4.2.1 Update `app.rs` to create MemoryManager instance
- [ ] 4.2.2 Call `trim_undo_stack()` after each undo state push
- [ ] 4.2.3 Add memory warning UI when `should_warn()` returns true
- [ ] 4.2.4 Add localStorage warning when `should_warn_storage()` returns true
- [ ] 4.2.5 Display memory usage in debug panel

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** 4.1  
**Acceptance Criteria:**
  - Undo stack never exceeds 50 entries
  - Memory warnings shown when threshold exceeded
  - User notified before localStorage full
  - Debug panel shows current memory usage

### 4.3 Unit Tests for Memory Management
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.1]

- [ ] 4.3.1 Test trim_undo_stack() removes oldest entries
- [ ] 4.3.2 Test trim_undo_stack() preserves newest entries
- [ ] 4.3.3 Test check_storage_usage() calculates size correctly
- [ ] 4.3.4 Test should_warn() threshold logic
- [ ] 4.3.5 Test should_warn_storage() at 80% threshold
- [ ] 4.3.6 Mock localStorage for deterministic testing

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** 4.1  
**Acceptance Criteria:**
  - All unit tests pass
  - LRU eviction verified
  - Threshold warnings tested
  - localStorage mocking works

## 5. Responsive Design System

### 5.1 Implement Data Models
**Requirements:** [Req 4.2.1]  
**Design:** [Design 2.1]

- [ ] 5.1.1 Add `ComponentStyle` struct to `canvas.rs`
- [ ] 5.1.2 Add `ResponsiveStyles` struct with base/tablet/desktop
- [ ] 5.1.3 Update `CanvasComponent` enum with optional styles field
- [ ] 5.1.4 Add `#[serde(default)]` for backward compatibility
- [ ] 5.1.5 Implement `Default` for ComponentStyle and ResponsiveStyles
- [ ] 5.1.6 Implement `ComponentStyle::merge()` method

**Priority:** HIGH | **Est:** 2 hours  
**Dependencies:** None  
**Acceptance Criteria:**
  - Data models compile without errors
  - Old layouts deserialize successfully (backward compatible)
  - Default implementation correct
  - Merge logic handles None values properly

**Technical Notes:**
  - Use Option<String> for all style properties
  - Mobile-first approach: base = mobile, tablet/desktop = overrides
  - Test deserialization with old format

### 5.2 Implement Breakpoint Manager
**Requirements:** [Req 4.2.1]  
**Design:** [Design 3.3]

- [ ] 5.2.1 Create `src/builder/breakpoint_manager.rs` file
- [ ] 5.2.2 Implement `Breakpoint` enum (Mobile, Tablet, Desktop)
- [ ] 5.2.3 Implement `Breakpoint::min_width()` and `max_width()` methods
- [ ] 5.2.4 Implement `Breakpoint::css_name()` method
- [ ] 5.2.5 Implement `BreakpointManager` struct with active_breakpoint signal
- [ ] 5.2.6 Implement `get_effective_styles()` with cascade logic

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 5.1  
**Acceptance Criteria:**
  - Breakpoint enum with correct pixel values (768px, 1024px)
  - BreakpointManager cascades styles correctly (base → tablet → desktop)
  - Active breakpoint tracked via signal
  - Style merging preserves specificity

### 5.3 Update Property Editor UI
**Requirements:** [Req 4.2.1]  
**Design:** [Design 3.3.3]

- [ ] 5.3.1 Add breakpoint tabs to `property_editor.rs`
- [ ] 5.3.2 Create active_breakpoint signal
- [ ] 5.3.3 Add buttons for Desktop/Tablet/Mobile breakpoints
- [ ] 5.3.4 Update property fields to edit current breakpoint styles
- [ ] 5.3.5 Add visual indicator for inherited vs overridden properties
- [ ] 5.3.6 Add "Reset to inherit" button for tablet/desktop overrides

**Priority:** HIGH | **Est:** 4 hours  
**Dependencies:** 5.2  
**Acceptance Criteria:**
  - Three breakpoint tabs render correctly
  - Switching tabs shows correct property values
  - Editing in tablet/desktop creates override
  - Visual distinction between inherited and overridden
  - Reset button removes override (reverts to inherit)

**Technical Notes:**
  - Use different styling for inherited properties (lighter text)
  - Show base value as placeholder for tablet/desktop
  - Validate CSS values on input

### 5.4 Update Canvas for Responsive Preview
**Requirements:** [Req 4.2.1]  
**Design:** [Design 3.3]

- [ ] 5.4.1 Update `canvas.rs` to use BreakpointManager
- [ ] 5.4.2 Apply effective styles based on active breakpoint
- [ ] 5.4.3 Add viewport size controls (Desktop/Tablet/Mobile buttons)
- [ ] 5.4.4 Update canvas width when viewport changes
- [ ] 5.4.5 Re-render components when breakpoint changes
- [ ] 5.4.6 Add viewport size label to canvas

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 5.2  
**Acceptance Criteria:**
  - Canvas applies correct styles per breakpoint
  - Viewport size buttons work (Desktop: 1024px+, Tablet: 768px, Mobile: 375px)
  - Components re-render with correct styles
  - Visual feedback for active viewport

### 5.5 Integration Tests for Responsive System
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.2]

- [ ] 5.5.1 Test style cascade (base + tablet override)
- [ ] 5.5.2 Test style cascade (base + tablet + desktop override)
- [ ] 5.5.3 Test backward compatibility (old layouts without styles)
- [ ] 5.5.4 Test property editor writes correct breakpoint
- [ ] 5.5.5 Test canvas renders correct styles per viewport
- [ ] 5.5.6 Test serialization/deserialization with responsive styles

**Priority:** MEDIUM | **Est:** 3 hours  
**Dependencies:** 5.4  
**Acceptance Criteria:**
  - All integration tests pass
  - Style cascade verified
  - Backward compatibility confirmed
  - Round-trip serialization works

## 6. Code Generation - Media Queries

### 6.1 Implement Media Query Generator
**Requirements:** [Req 4.2.2, 4.2.3]  
**Design:** [Design 3.4]

- [ ] 6.1.1 Create `src/builder/media_query_gen.rs` file
- [ ] 6.1.2 Implement `generate_media_queries()` function
- [ ] 6.1.3 Generate base styles (mobile-first)
- [ ] 6.1.4 Generate tablet media query (@media min-width: 768px)
- [ ] 6.1.5 Generate desktop media query (@media min-width: 1024px)
- [ ] 6.1.6 Implement `format_component_style()` helper
- [ ] 6.1.7 Add `CanvasComponent::get_responsive_styles()` method

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 5.1  
**Acceptance Criteria:**
  - CSS output is valid and well-formatted
  - Mobile-first approach (base + min-width queries)
  - Only generates media queries for components with styles
  - Class names unique per component

**Technical Notes:**
  - Use component index for class names (comp-0, comp-1, ...)
  - Skip media queries if tablet/desktop overrides are None
  - Indent CSS for readability (2 spaces)

### 6.2 Integrate with Export System
**Requirements:** [Req 4.2.2]  
**Design:** [Design 3.4.3]

- [ ] 6.2.1 Update `export.rs` to import media_query_gen
- [ ] 6.2.2 Create `generate_leptos_code_with_responsive()` function
- [ ] 6.2.3 Include media queries in generated code
- [ ] 6.2.4 Wrap media queries in <style> tag
- [ ] 6.2.5 Add class names to generated component view! code
- [ ] 6.2.6 Test exported code compiles and renders correctly

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 6.1  
**Acceptance Criteria:**
  - Exported code includes media queries
  - Class names applied to components
  - Generated project runs without errors
  - Responsive behavior works in exported code

### 6.3 Unit Tests for Media Query Generation
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.1]

- [ ] 6.3.1 Test generate_media_queries() with single component
- [ ] 6.3.2 Test with multiple components
- [ ] 6.3.3 Test with only base styles (no media queries)
- [ ] 6.3.4 Test with tablet override only
- [ ] 6.3.5 Test with full responsive styles (base + tablet + desktop)
- [ ] 6.3.6 Validate CSS syntax of generated output

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** 6.1  
**Acceptance Criteria:**
  - All unit tests pass
  - CSS output validated
  - Edge cases covered (no styles, partial styles)

## 7. Code Generation - Project Scaffolding

### 7.1 Implement Project Template Engine
**Requirements:** [Req 4.3.1]  
**Design:** [Design 3.5.2]

- [ ] 7.1.1 Create `src/builder/project_scaffold.rs` file
- [ ] 7.1.2 Implement `ProjectScaffold` struct with name, components, preset
- [ ] 7.1.3 Implement `ProjectFile` struct with path and content
- [ ] 7.1.4 Implement `generate_files()` method
- [ ] 7.1.5 Implement `generate_cargo_toml()` with dependency detection
- [ ] 7.1.6 Implement `generate_main_rs()` with component code
- [ ] 7.1.7 Implement `generate_trunk_toml()`
- [ ] 7.1.8 Implement `generate_readme()`
- [ ] 7.1.9 Implement `generate_gitignore()`
- [ ] 7.1.10 Implement `generate_index_html()`

**Priority:** HIGH | **Est:** 5 hours  
**Dependencies:** 6.2  
**Acceptance Criteria:**
  - All 6 project files generated correctly
  - Cargo.toml includes correct dependencies per preset
  - main.rs contains valid Leptos code
  - Trunk.toml configured properly
  - README has setup instructions
  - Generated project structure is valid

**Technical Notes:**
  - Detect dependencies from ExportPreset (ThawUi, LeptosMaterial, etc.)
  - Use raw strings (r#"..."#) for template literals
  - Validate project name (valid Rust crate name)

### 7.2 Implement ZIP Export
**Requirements:** [Req 4.3.1]  
**Design:** [Design 3.5.3]

- [ ] 7.2.1 Add JSZip to project (via npm or WASM binding)
- [ ] 7.2.2 Implement `export_project_as_zip()` async function
- [ ] 7.2.3 Create JSZip instance via js_sys::eval
- [ ] 7.2.4 Add all project files to ZIP
- [ ] 7.2.5 Generate ZIP blob asynchronously
- [ ] 7.2.6 Trigger browser download via anchor element
- [ ] 7.2.7 Cleanup blob URL after download

**Priority:** HIGH | **Est:** 4 hours  
**Dependencies:** 7.1  
**Acceptance Criteria:**
  - ZIP file downloads successfully
  - ZIP contains all project files with correct paths
  - File contents match generated templates
  - Download works in all major browsers
  - Proper error handling for ZIP generation failures

**Technical Notes:**
  - Use wasm_bindgen_futures for async/await
  - Proper Closure handling to avoid memory leaks
  - Blob URL must be revoked after download

### 7.3 Add Project Export UI
**Requirements:** [Req 4.3.1]  
**Design:** [Design 3.5]

- [ ] 7.3.1 Add "Export Full Project" button to export panel
- [ ] 7.3.2 Add project name input field
- [ ] 7.3.3 Add preset selector (Plain, ThawUi, LeptosMaterial, LeptosUse)
- [ ] 7.3.4 Show loading spinner during ZIP generation
- [ ] 7.3.5 Show success message after download
- [ ] 7.3.6 Handle errors gracefully with user-friendly messages

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** 7.2  
**Acceptance Criteria:**
  - UI elements render correctly
  - Project name validated (no special characters)
  - Preset selection works
  - Loading state shown during async operation
  - Success/error messages clear

### 7.4 Unit Tests for Project Scaffolding
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.1]

- [ ] 7.4.1 Test generate_cargo_toml() for each preset
- [ ] 7.4.2 Test generate_main_rs() includes all components
- [ ] 7.4.3 Test detect_dependencies() returns correct crates
- [ ] 7.4.4 Test generated Trunk.toml is valid
- [ ] 7.4.5 Test project name validation
- [ ] 7.4.6 Test generated files are valid (syntax check)

**Priority:** MEDIUM | **Est:** 3 hours  
**Dependencies:** 7.1  
**Acceptance Criteria:**
  - All unit tests pass
  - Generated files validated
  - Dependency detection correct per preset
  - Edge cases handled (empty project, invalid names)

## 8. Code Generation - Formatter

### 8.1 Implement Basic Code Formatter
**Requirements:** [Req 4.3.2, 4.3.3]  
**Design:** [Design 3.6]

- [ ] 8.1.1 Create `src/builder/code_formatter.rs` file
- [ ] 8.1.2 Implement `CodeFormatter` struct with config (indent_size, use_tabs)
- [ ] 8.1.3 Implement `format_rust_code()` with basic indentation
- [ ] 8.1.4 Handle opening braces (increase indent)
- [ ] 8.1.5 Handle closing braces (decrease indent)
- [ ] 8.1.6 Preserve string literals
- [ ] 8.1.7 Implement `clean_imports()` to sort and deduplicate
- [ ] 8.1.8 Implement `make_indent()` helper

**Priority:** MEDIUM | **Est:** 3 hours  
**Dependencies:** None  
**Acceptance Criteria:**
  - Basic indentation works for braces
  - Imports sorted alphabetically
  - Duplicate imports removed
  - String literals preserved
  - Configurable indent (spaces vs tabs)

**Technical Notes:**
  - This is basic formatting only (not full rustfmt)
  - Focus on readability, not perfect Rust style
  - Handle edge cases (nested blocks, macros)

### 8.2 Integrate Formatter with Export
**Requirements:** [Req 4.3.2]  
**Design:** [Design 3.6]

- [ ] 8.2.1 Update `export.rs` to import CodeFormatter
- [ ] 8.2.2 Apply formatter to generated Leptos code
- [ ] 8.2.3 Apply formatter to main.rs in project scaffold
- [ ] 8.2.4 Add "Format Code" checkbox to export UI
- [ ] 8.2.5 Make formatting optional (default: enabled)

**Priority:** MEDIUM | **Est:** 1.5 hours  
**Dependencies:** 8.1  
**Acceptance Criteria:**
  - Exported code is formatted by default
  - User can disable formatting if desired
  - Formatted code is more readable
  - No syntax errors introduced by formatting

### 8.3 Unit Tests for Code Formatter
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.1]

- [ ] 8.3.1 Test format_rust_code() with nested blocks
- [ ] 8.3.2 Test indentation with braces
- [ ] 8.3.3 Test clean_imports() sorts correctly
- [ ] 8.3.4 Test duplicate import removal
- [ ] 8.3.5 Test string literal preservation
- [ ] 8.3.6 Test edge cases (empty code, only imports)

**Priority:** LOW | **Est:** 2 hours  
**Dependencies:** 8.1  
**Acceptance Criteria:**
  - All unit tests pass
  - Formatting preserves code semantics
  - Edge cases handled

## 9. UX Enhancement - Onboarding

### 9.1 Implement Onboarding State
**Requirements:** [Req 4.4.1]  
**Design:** [Design 3.7]

- [ ] 9.1.1 Create `src/builder/onboarding.rs` file
- [ ] 9.1.2 Implement `TutorialStep` enum (Welcome, DragComponent, EditProperties, ExportCode, Complete)
- [ ] 9.1.3 Implement `OnboardingState` struct with signals
- [ ] 9.1.4 Implement `new()` to load completion status from localStorage
- [ ] 9.1.5 Implement `next_step()` to progress tutorial
- [ ] 9.1.6 Implement `skip_tutorial()` to dismiss
- [ ] 9.1.7 Implement `mark_completed()` to save to localStorage
- [ ] 9.1.8 Implement localStorage save/load helpers

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** None  
**Acceptance Criteria:**
  - Tutorial state persists across sessions
  - next_step() progresses correctly
  - skip_tutorial() dismisses and marks complete
  - localStorage key: "leptos_studio_tutorial_completed"

### 9.2 Implement OnboardingOverlay Component
**Requirements:** [Req 4.4.1]  
**Design:** [Design 3.7.1]

- [ ] 9.2.1 Implement `OnboardingOverlay` component
- [ ] 9.2.2 Show modal for Welcome step with Start/Skip buttons
- [ ] 9.2.3 Show highlight overlay for DragComponent step (sidebar)
- [ ] 9.2.4 Show highlight overlay for EditProperties step (property editor)
- [ ] 9.2.5 Show highlight overlay for ExportCode step (export button)
- [ ] 9.2.6 Show completion message for Complete step
- [ ] 9.2.7 Add proper CSS for overlay and modal

**Priority:** MEDIUM | **Est:** 4 hours  
**Dependencies:** 9.1  
**Acceptance Criteria:**
  - Overlay renders only for new users
  - Each step highlights correct UI element
  - Navigation buttons work correctly
  - Modal dismisses after completion
  - Visual design polished (not intrusive)

### 9.3 Integrate Onboarding with App
**Requirements:** [Req 4.4.1]  
**Design:** [Design 3.7]

- [ ] 9.3.1 Create OnboardingState in `app.rs`
- [ ] 9.3.2 Render OnboardingOverlay component in App
- [ ] 9.3.3 Add "Restart Tutorial" button to settings/help menu
- [ ] 9.3.4 Test tutorial flow from start to finish
- [ ] 9.3.5 Test skip functionality
- [ ] 9.3.6 Test restart functionality

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** 9.2  
**Acceptance Criteria:**
  - Tutorial shows for first-time users
  - Tutorial can be restarted from menu
  - No impact on existing user experience
  - Completed users don't see tutorial

### 9.4 Integration Tests for Onboarding
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.2]

- [ ] 9.4.1 Test tutorial flow (Welcome → Complete)
- [ ] 9.4.2 Test skip functionality
- [ ] 9.4.3 Test localStorage persistence
- [ ] 9.4.4 Test restart clears completed flag
- [ ] 9.4.5 Test overlay visibility logic
- [ ] 9.4.6 Mock localStorage for deterministic tests

**Priority:** LOW | **Est:** 2 hours  
**Dependencies:** 9.1  
**Acceptance Criteria:**
  - All integration tests pass
  - Tutorial flow verified
  - Persistence tested

## 10. UX Enhancement - Accessibility

### 10.1 Implement Accessibility Module
**Requirements:** [Req 4.4.2]  
**Design:** [Design 1.2.2]

- [ ] 10.1.1 Create `src/builder/accessibility.rs` file
- [ ] 10.1.2 Implement keyboard navigation helpers
- [ ] 10.1.3 Implement focus management utilities
- [ ] 10.1.4 Implement ARIA label generation for components
- [ ] 10.1.5 Implement screen reader announcements helper
- [ ] 10.1.6 Add skip links for canvas navigation

**Priority:** LOW | **Est:** 3 hours  
**Dependencies:** None  
**Acceptance Criteria:**
  - Keyboard navigation works (Tab, Arrow keys)
  - Focus visible and logical
  - ARIA labels present on interactive elements
  - Screen reader friendly announcements
  - Skip links allow bypassing navigation

### 10.2 Audit and Fix Existing Accessibility Issues
**Requirements:** [Req 4.4.2]  
**Design:** [Design 1.2.2]

- [ ] 10.2.1 Add ARIA labels to sidebar components
- [ ] 10.2.2 Add ARIA labels to canvas components
- [ ] 10.2.3 Add ARIA labels to property editor fields
- [ ] 10.2.4 Ensure all buttons have accessible names
- [ ] 10.2.5 Add keyboard shortcuts documentation
- [ ] 10.2.6 Test with screen reader (NVDA/JAWS)

**Priority:** LOW | **Est:** 4 hours  
**Dependencies:** 10.1  
**Acceptance Criteria:**
  - All interactive elements have ARIA labels
  - Keyboard navigation works throughout app
  - Screen reader can navigate and operate app
  - Meets WCAG 2.1 AA standards (basic compliance)

### 10.3 Integration Tests for Accessibility
**Requirements:** [Req 5.6.1, 7.6.1]  
**Design:** [Design 7.2]

- [ ] 10.3.1 Test keyboard navigation (Tab order)
- [ ] 10.3.2 Test focus management (modal open/close)
- [ ] 10.3.3 Test ARIA labels present
- [ ] 10.3.4 Test keyboard shortcuts work
- [ ] 10.3.5 Automated a11y audit (axe-core or similar)

**Priority:** LOW | **Est:** 2 hours  
**Dependencies:** 10.2  
**Acceptance Criteria:**
  - Keyboard navigation tests pass
  - No critical a11y violations
  - Focus management verified

## 11. Performance Testing & Validation

### 11.1 Benchmark Virtual Scrolling
**Requirements:** [Req 5.1, 7.8.1]  
**Design:** [Design 7.3]

- [ ] 11.1.1 Create benchmark for sidebar with 100 components
- [ ] 11.1.2 Create benchmark for sidebar with 1000 components
- [ ] 11.1.3 Create benchmark for sidebar with 5000 components
- [ ] 11.1.4 Measure FPS during scroll
- [ ] 11.1.5 Measure memory usage
- [ ] 11.1.6 Compare with non-virtualized baseline

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 2.3  
**Acceptance Criteria:**
  - 60 FPS maintained with 1000+ components
  - Memory usage reasonable (<300MB for 1000 components)
  - Performance improvement over baseline documented
  - Benchmarks automated and repeatable

**Technical Notes:**
  - Use Performance API for measurements
  - Run benchmarks multiple times for average
  - Document system specs for reproducibility

### 11.2 Benchmark Lazy Rendering
**Requirements:** [Req 5.1, 7.8.1]  
**Design:** [Design 7.3]

- [ ] 11.2.1 Create benchmark for canvas with 100 components
- [ ] 11.2.2 Create benchmark for canvas with 1000 components
- [ ] 11.2.3 Create benchmark for canvas with 5000 components
- [ ] 11.2.4 Measure time to first paint
- [ ] 11.2.5 Measure time to interactive
- [ ] 11.2.6 Measure frame drops during rendering

**Priority:** HIGH | **Est:** 3 hours  
**Dependencies:** 3.4  
**Acceptance Criteria:**
  - First paint <1s for 1000 components
  - Interactive <3s for 1000 components
  - No frame drops below 60 FPS
  - Progressive rendering visible to user

### 11.3 Memory Profiling
**Requirements:** [Req 5.1, 7.8.1]  
**Design:** [Design 7.3]

- [ ] 11.3.1 Profile memory usage with 100 components
- [ ] 11.3.2 Profile memory usage with 1000 components
- [ ] 11.3.3 Profile undo stack memory
- [ ] 11.3.4 Profile localStorage usage
- [ ] 11.3.5 Identify memory leaks (if any)
- [ ] 11.3.6 Document memory optimization recommendations

**Priority:** MEDIUM | **Est:** 3 hours  
**Dependencies:** 4.2  
**Acceptance Criteria:**
  - Memory usage within targets (<300MB for 1000 components)
  - No memory leaks detected
  - Undo stack properly trimmed
  - localStorage under 5MB limit

### 11.4 Bundle Size Analysis
**Requirements:** [Req 5.1.4, 7.8.1]  
**Design:** [Design 7.3]

- [ ] 11.4.1 Measure WASM bundle size (production build)
- [ ] 11.4.2 Analyze bundle composition (wasm-opt --print-wasm-stats)
- [ ] 11.4.3 Identify largest modules
- [ ] 11.4.4 Apply wasm-opt with aggressive settings
- [ ] 11.4.5 Test code splitting opportunities
- [ ] 11.4.6 Document optimization recommendations

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** None  
**Acceptance Criteria:**
  - Bundle size <2MB (target: maintain current ~1.8MB)
  - wasm-opt applied in release builds
  - Optimization recommendations documented
  - No functional regressions after optimization

## 12. Documentation & Migration

### 12.1 Update API Documentation
**Requirements:** [Req 7.1.1, 7.6.2]  
**Design:** [Design 9.1]

- [ ] 12.1.1 Add rustdoc comments to virtual_scroll module
- [ ] 12.1.2 Add rustdoc comments to lazy_renderer module
- [ ] 12.1.3 Add rustdoc comments to memory_manager module
- [ ] 12.1.4 Add rustdoc comments to breakpoint_manager module
- [ ] 12.1.5 Add rustdoc comments to media_query_gen module
- [ ] 12.1.6 Add rustdoc comments to project_scaffold module
- [ ] 12.1.7 Add rustdoc comments to code_formatter module
- [ ] 12.1.8 Update API.md with new modules

**Priority:** MEDIUM | **Est:** 4 hours  
**Dependencies:** All implementation tasks  
**Acceptance Criteria:**
  - All public APIs have rustdoc comments
  - Examples provided for complex APIs
  - API.md includes all new modules
  - `cargo doc` generates complete documentation

### 12.2 Update User Documentation
**Requirements:** [Req 7.1.1, 6.4.2]  
**Design:** [Design 9.2]

- [ ] 12.2.1 Add "Performance" section to README (virtual scrolling, lazy rendering)
- [ ] 12.2.2 Add "Responsive Design" section to README (breakpoint system)
- [ ] 12.2.3 Add "Project Export" section to README (full project scaffolding)
- [ ] 12.2.4 Update "Features" list with new capabilities
- [ ] 12.2.5 Add screenshots/GIFs for new features
- [ ] 12.2.6 Update CHANGELOG.md with version bump (0.2.0)

**Priority:** MEDIUM | **Est:** 3 hours  
**Dependencies:** All implementation tasks  
**Acceptance Criteria:**
  - README reflects all new features
  - Usage examples provided
  - CHANGELOG follows Keep a Changelog format
  - Version bumped to 0.2.0

### 12.3 Create Migration Guide
**Requirements:** [Req 7.6.2]  
**Design:** [Design 9.3]

- [ ] 12.3.1 Create MIGRATION.md file
- [ ] 12.3.2 Document backward compatibility guarantees
- [ ] 12.3.3 Explain new responsive styles feature
- [ ] 12.3.4 Provide example of migrating to responsive styles
- [ ] 12.3.5 Document breaking changes (if any)
- [ ] 12.3.6 Add troubleshooting section for common migration issues

**Priority:** MEDIUM | **Est:** 2 hours  
**Dependencies:** All implementation tasks  
**Acceptance Criteria:**
  - MIGRATION.md clearly explains upgrade process
  - Examples provided for new features
  - Troubleshooting covers common issues
  - No breaking changes for existing layouts

### 12.4 Update Contributing Guidelines
**Requirements:** [Req 7.1.1]  
**Design:** [Design 9.1]

- [ ] 12.4.1 Document new module structure
- [ ] 12.4.2 Add guidelines for performance optimization
- [ ] 12.4.3 Add guidelines for accessibility
- [ ] 12.4.4 Update testing requirements
- [ ] 12.4.5 Document benchmark process

**Priority:** LOW | **Est:** 1.5 hours  
**Dependencies:** All implementation tasks  
**Acceptance Criteria:**
  - CONTRIBUTING.md up to date
  - Guidelines for new features clear
  - Testing requirements documented

---

## Progress Summary

**Total Tasks:** 134  
**Completed:** 0/134  
**In Progress:** 0/134  
**Pending:** 134/134

### By Priority:
- **HIGH Priority:** 50 tasks (Documentation fixes, Performance, Responsive Design, Code Generation core)
- **MEDIUM Priority:** 58 tasks (Memory management, UX enhancements, Testing, Documentation)
- **LOW Priority:** 26 tasks (Accessibility, Advanced testing, Polish)

### By Phase:
1. **Phase 1 - Critical Fixes** (2 task groups, ~3.5 hours)
2. **Phase 2 - Performance** (12 task groups, ~38 hours)
3. **Phase 3 - Responsive Design** (7 task groups, ~19 hours)
4. **Phase 4 - Code Generation** (11 task groups, ~28 hours)
5. **Phase 5 - UX & A11y** (7 task groups, ~19 hours)
6. **Phase 6 - Testing & Validation** (4 task groups, ~11 hours)
7. **Phase 7 - Documentation** (4 task groups, ~10.5 hours)

**Total Estimated Time:** ~129 hours

### Critical Path:
1. Documentation fixes (1.1, 1.2)
2. Virtual scrolling (2.1 → 2.2 → 2.3)
3. Lazy rendering (3.1 → 3.2 → 3.3 → 3.4)
4. Responsive data models (5.1 → 5.2 → 5.3 → 5.4)
5. Media query generation (6.1 → 6.2)
6. Project scaffolding (7.1 → 7.2 → 7.3)

### Next Immediate Tasks:
- [ ] Task 1.1.1 - Fix README documentation
- [ ] Task 1.1.2 - Clarify hot reload terminology
- [ ] Task 1.1.3 - Add browser limitations section
- [ ] Task 1.2.1 - Remove git_panel.rs module

---

**Document Version:** 1.0  
**Last Updated:** 2025-11-04  
**Status:** Ready for Implementation  
**References:** requirements.md v2.0, design.md v1.0
