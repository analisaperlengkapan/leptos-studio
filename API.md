# Leptos Studio API Documentation

## Core Types

### CanvasComponent
- `Button { label: String }`
- `Text { content: String }`
- `Input { placeholder: String }`
- `Container { children: Vec<CanvasComponent> }`
- `Custom { name: String }`

### LibraryComponent
- name: `String`
- kind: `String` (e.g. "Button", "Text", "Custom")
- template: `Option<String>` (for custom)
- category: `String`
- props_schema: `Option<Vec<PropSchema>>`
- description: `Option<String>`

### PropSchema
- name: `String`
- prop_type: `String` (e.g. "string", "number", "bool")
- required: `bool`
- description: `Option<String>`

### ResponsiveMode
- `Desktop`, `Tablet`, `Mobile`

### Theme
- `Light`, `Dark`, `Custom`

## Signals & State
- `components: RwSignal<Vec<CanvasComponent>>` — current layout state
- `undo_stack: RwSignal<Vec<Vec<CanvasComponent>>>`
- `redo_stack: RwSignal<Vec<Vec<CanvasComponent>>>`
- `custom_components: RwSignal<Vec<LibraryComponent>>`
- `theme: RwSignal<Theme>`
- `responsive_mode: RwSignal<ResponsiveMode>`

## Main Functions
- `canvas(...)` — main canvas UI, handles drag/drop, render, responsive
- `sidebar(...)` — sidebar UI, component library, theme, responsive, version control
- `GitPanel()` — version control panel (commit, log, status)

## Custom Component Lifecycle
- Add/Edit/Delete via sidebar
- Live preview via signals (hot reload)

## Export
- Export layout as Leptos code (with preset: thaw-ui, leptos-material, leptos-use)

## Testing
- Unit & integration tests in `tests/`

---
Lihat README dan source code untuk detail lebih lanjut.
