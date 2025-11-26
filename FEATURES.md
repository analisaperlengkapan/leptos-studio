# Leptos Studio – Advanced Features Guide

This guide covers the advanced features and recent improvements to Leptos Studio, a powerful visual UI builder for the Leptos framework.

## Table of Contents

1. [Responsive Design Preview](#responsive-design-preview)
2. [Enhanced Component Library](#enhanced-component-library)
3. [Advanced Styling System](#advanced-styling-system)
4. [Component Constraints & Guidelines](#component-constraints--guidelines)
5. [Hot Reload for Custom Components](#hot-reload-for-custom-components)
6. [Error Handling & Validation](#error-handling--validation)

---

## Responsive Design Preview

### Overview

The Responsive Design Preview feature allows you to view and design your UI for different device sizes without leaving the editor. Supported presets include:

- **Mobile**: 375px × 667px - Standard smartphone viewport
- **Tablet**: 768px × 1024px - Standard tablet viewport
- **Desktop**: 1920px × 1080px - Full desktop viewport

### Usage

1. Click any of the responsive buttons in the top navigation bar (`📱 Mobile`, `📱 Tablet`, `🖥️ Desktop`)
2. The canvas will resize to match the selected device viewport
3. Components automatically adapt to the new viewport size
4. Use the responsive indicator (bottom-right corner) to confirm current viewport

### Code Location

- **Component**: `src/builder/responsive_preview.rs`
- **Exports**: `ResponsivePreviewControls`, `CanvasViewport`, `ResponsiveIndicator`
- **Integration**: Added to main navigation in `src/app.rs`

### CSS Classes

```css
.responsive-controls       /* Viewport control container */
.responsive-btn           /* Individual viewport button */
.responsive-btn.active    /* Active button state */
.canvas-viewport-wrapper  /* Canvas container with size constraints */
.responsive-indicator     /* Floating viewport info display */
```

---

## Enhanced Component Library

### Overview

The Enhanced Component Library provides better organization and discovery of components through:

- **Search functionality** - Fuzzy search across component names, kinds, and categories
- **Category filtering** - Organize components by type (Basic, Input, Container, Custom)
- **Component cards** - Improved visual representation with icons and metadata
- **Favorites system** - Mark frequently used components for quick access

### Features

#### Search Components

```rust
let filtered = search_components(&components, "button", &ComponentCategory::All);
```

#### Get Categories

```rust
let categories = get_categories(&components);
```

#### Component Categories

- **Basic**: Button, Text components
- **Input**: Form inputs, textareas
- **Container**: Layout containers, grids
- **Custom**: User-defined components
- **All**: Show all components

### Code Location

- **Module**: `src/builder/component_library_enhanced.rs`
- **Exports**: `ComponentCategory`, `search_components`, `get_categories`, `ComponentCard`, `LibrarySearchBar`, `CategoryFilter`

### CSS Classes

```css
.library-search               /* Search container */
.library-search-input        /* Search input field */
.category-filter             /* Category buttons container */
.category-btn               /* Individual category button */
.category-btn.active        /* Active category */
.component-card             /* Component card container */
.component-card-icon        /* Component icon */
.component-card-title       /* Component name */
.component-card-badge       /* Component type badge */
.favorite-btn               /* Favorite button */
```

---

## Advanced Styling System

### Overview

The Advanced Styling System provides powerful styling capabilities:

- **CSS property editor** - Edit common CSS properties visually
- **Theme presets** - Pre-configured themes for quick styling
- **Inline CSS generation** - Auto-generate CSS strings from properties
- **Style persistence** - Styles are saved with components

### Supported Style Properties

```rust
pub struct ComponentStyle {
    pub padding: Option<String>,           // e.g., "8px" or "8px 16px"
    pub margin: Option<String>,
    pub width: Option<String>,             // e.g., "100px" or "50%"
    pub height: Option<String>,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<u32>,         // in pixels
    pub border_radius: Option<u32>,        // in pixels
    pub font_size: Option<u32>,            // in pixels
    pub font_weight: Option<String>,       // "normal", "bold", "600"
    pub text_align: Option<String>,        // "left", "center", "right"
    pub display: Option<String>,           // "flex", "grid", "block"
    pub flex_direction: Option<String>,    // "row", "column"
    pub gap: Option<String>,               // e.g., "8px"
    pub custom_css: Option<String>,        // Custom CSS class
}
```

### Theme Presets

1. **Light** - Clean, professional light theme
2. **Dark** - Dark mode with high contrast
3. **High Contrast** - Maximum accessibility
4. **Colorful** - Vibrant, eye-catching colors
5. **Minimal** - Subtle, minimal styling

Each theme has preset styles for buttons, containers, and other components.

### Usage Example

```rust
// Get button style for Dark theme
let style = ThemePreset::Dark.get_button_style();

// Generate CSS string
let css = style.to_css_string();
// Output: "background-color: #1e293b; border-color: #475569; ..."
```

### Code Location

- **Module**: `src/builder/styling_system.rs`
- **Exports**: `ComponentStyle`, `ThemePreset`, `StyleEditor`, `ThemeSelector`

### CSS Classes

```css
.style-editor              /* Style editor container */
.style-control            /* Individual style property */
.color-picker            /* Color input */
.text-input              /* Text input field */
.number-input            /* Number input field */
.theme-selector          /* Theme selector container */
.theme-buttons           /* Theme button group */
.theme-btn               /* Individual theme button */
```

---

## Component Constraints & Guidelines

### Overview

Component constraints and guidelines ensure design consistency and prevent invalid component configurations:

- **Size constraints** - Min/max width and height
- **Aspect ratio** - Enforce specific width/height ratios
- **Alignment options** - Flexbox alignment presets
- **Design guidelines** - Best practices for component sizing and spacing
- **Responsive breakpoints** - Common device breakpoints
- **Grid systems** - Bootstrap 12-column and Material 8-column grids

### Size Constraints

```rust
pub struct SizeConstraints {
    pub min_width: Option<u32>,
    pub max_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_height: Option<u32>,
    pub aspect_ratio: Option<f32>,
}
```

### Predefined Constraints

```rust
SizeConstraints::for_button()     // 80-400px width, 32px+ height
SizeConstraints::for_input()      // 120-500px width, 32px+ height
SizeConstraints::for_container()  // 100px+ width and height
SizeConstraints::for_text()       // 50px+ width, 20px+ height
```

### Alignment Options

- `FlexStart` - Align to start
- `Center` - Center align
- `FlexEnd` - Align to end
- `SpaceBetween` - Distribute with space between
- `SpaceAround` - Distribute with equal space around
- `SpaceEvenly` - Distribute with equal space

### Design Guidelines

Pre-configured guidelines for:

- **Buttons** - Primary, Secondary, Icon buttons with recommended sizes
- **Inputs** - Text input and large textarea sizes
- **Spacing** - Compact, Default, Comfortable, Loose spacing presets

### Responsive Breakpoints

Standard device breakpoints:

| Name | Width |
|------|-------|
| Mobile | 320px |
| Small | 480px |
| Tablet | 768px |
| Desktop | 1024px |
| Large | 1280px |
| Extra Large | 1920px |

### Grid Systems

```rust
GridSystem::bootstrap_12()  // 12 columns, 16px gap
GridSystem::material_8()    // 8 columns, 8px gap
```

### Code Location

- **Module**: `src/builder/component_constraints.rs`
- **Exports**: `SizeConstraints`, `AlignmentOption`, `DesignGuideline`, `ResponsiveBreakpoint`, `GridSystem`

---

## Hot Reload for Custom Components

### Overview

When you modify custom component templates, changes are automatically reflected in the editor without requiring a full page reload. This enables rapid iteration and testing.

### Implementation Details

The hot reload system monitors changes to custom component definitions and:

1. Validates the new template
2. Updates the component library in real-time
3. Re-renders canvas components using the new template
4. Shows confirmation notifications

### How It Works

1. Edit a custom component's HTML template in the sidebar
2. Changes are immediately validated using `HtmlTemplateValidator`
3. If valid, the component updates automatically on the canvas
4. If invalid, an error message explains the issue

### Notifications

- ✅ "Component successfully updated" - Template change applied
- ❌ "Template validation failed" - Template contains invalid HTML
- ⚠️ "Component name already exists" - Duplicate name detected

### Future Enhancements

- Support for component property schema updates
- Undo/redo for component modifications
- Component version history
- Live preview of template changes

---

## Error Handling & Validation

### Overview

Comprehensive error handling ensures a smooth user experience with clear, actionable feedback for all operations.

### Error Types

#### Validation Errors

- **EmptyName** - Component name is required
- **InvalidName** - Name contains invalid characters (must be `[a-zA-Z0-9_]`)
- **EmptyTemplate** - Template cannot be empty
- **InvalidTemplate** - Template contains invalid HTML

#### Application Errors

- **NotFound** - Component or resource not found
- **InvalidOperation** - Operation cannot be performed
- **SerializationError** - Failed to serialize/deserialize data
- **ClipboardError** - Clipboard operation failed

### Error Display

Errors are displayed through:

1. **Snackbar notifications** - Toast notifications at bottom of screen
2. **Form validation messages** - Inline errors in forms
3. **Dialog alerts** - For critical errors
4. **Console logs** - For debugging

### Notification Types

```rust
NotificationType::Success    // ✅ Green background
NotificationType::Error      // ❌ Red background
NotificationType::Warning    // ⚠️ Yellow background
NotificationType::Info       // ℹ️ Blue background
```

### Message Format

All error messages follow a consistent format:

```
[Icon] [Action] [Details]

Examples:
❌ Template validation failed: Expected closing tag
✅ Component added successfully
⚠️ Layout change will reset history
ℹ️ Drag components to canvas to get started
```

### Validation Flow

```
User Input
    ↓
Validate (domain validator)
    ├─ Valid → Apply change → Show success
    └─ Invalid → Show error message → Allow correction
```

### Code Location

- **Validators**: `src/domain/validation.rs`
- **Error types**: `src/domain/error.rs`
- **Notifications**: `src/state/app_state.rs`
- **UI display**: `src/builder/snackbar.rs`

### Best Practices

1. Always validate user input before applying
2. Provide specific error messages
3. Suggest solutions when possible
4. Use appropriate notification types
5. Auto-clear success messages after a few seconds
6. Keep error messages concise and friendly

---

## Development Guidelines

### Adding New Features

1. **Create a new module** in `src/builder/` or appropriate layer
2. **Add exports** to `src/builder/mod.rs`
3. **Add CSS classes** for styling
4. **Include error handling** with user-friendly messages
5. **Write documentation** in comments
6. **Add tests** if applicable

### Code Organization

```
src/
├── domain/          # Business logic, types, validators
├── state/           # Application state management
├── services/        # Use case functions
├── builder/         # UI components
│   ├── responsive_preview.rs      # ← New features
│   ├── component_library_enhanced.rs
│   ├── styling_system.rs
│   └── component_constraints.rs
└── utils/           # Shared utilities
```

### Testing

Run tests with:

```bash
cargo test                                    # All tests
cargo test --target wasm32-unknown-unknown    # WASM tests
cargo test --lib domain                       # Domain tests only
```

### Building

```bash
trunk serve          # Development server
trunk build --release # Production build
```

---

## Summary

Leptos Studio now includes:

- ✅ **Responsive Design Preview** - Mobile, Tablet, Desktop viewports
- ✅ **Enhanced Component Library** - Search, categorization, favorites
- ✅ **Advanced Styling System** - Visual CSS editor with theme presets
- ✅ **Component Constraints** - Size, alignment, and design guidelines
- ✅ **Hot Reload** - Real-time updates for custom components
- ✅ **Robust Error Handling** - Clear, actionable error messages

These features combine to create a professional, user-friendly UI builder experience.
