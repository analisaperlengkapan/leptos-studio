# Leptos Studio – Architecture Overview

This document summarizes the **actual** architecture of `leptos-studio` at a high level.
The focus is on layer separation and key data flows frequently touched during development.

---

## 1. Layers & Modules

Leptos Studio is built with clear separation of concerns:

- **Domain (`src/domain`)**  
  Pure application models and rules, with no dependencies on Leptos or browser APIs.

- **State (`src/state`)**  
  Application state based on Leptos `RwSignal`, including UI state, project state, and history.

- **Services (`src/services`)**  
  "Use case" functions that operate on domain/state: export, project, git, and property update.

- **Builder / UI (`src/builder`)**  
  Leptos components that compose state + services into a visual editor (canvas, sidebar, property editor, panels).

- **Utils (`src/utils`)**  
  Generic helper functions like clipboard and formatting utilities.

- **Styling (`style.css`, `src/builder/design_tokens.rs`)**  
  Design tokens (colors, spacing, typography, etc.) and global CSS used throughout the UI.

### 1.1 Domain

Main files: `src/domain/component.rs` and `src/domain/validation.rs`.

- Provides component types: `ButtonComponent`, `TextComponent`, `InputComponent`, `ContainerComponent`, `CustomComponent`.
- Provides supporting enums: `ButtonVariant`, `ButtonSize`, `TextStyle`, `TextTag`, `InputType`, `LayoutType`, etc.
- `CanvasComponent` is a wrapper enum used in Canvas to store all components in a single collection.
- `PropValue` (`String`, `Number`, `Boolean`, `Null`) represents property values generically.
- Validation module contains functions for checking component names, HTML templates, and other rules.

### 1.2 State

Main files: `src/state/app_state.rs`, `src/state/project.rs`, `src/state/history.rs`.

- `AppState` is the root state shared via Leptos context.
- `UiState` stores:
  - Panel visibility (command palette, export modal, git, debug),
  - Notifications,
  - `custom_components` and `component_library`,
  - Render metrics (`render_count`, `render_time`).
- `LibraryComponent` + `PropSchema` describe components in the library and property schemas that drive the Property Editor.
- `default_components()` populates `component_library` with Button/Text/Input/Container and their `props_schema`.
- Project module provides serializable structures for the entire project.
- History module manages the undo/redo stack.

### 1.3 Services

Main files: `src/services/export_service.rs`, `project_service.rs`, `git_service.rs`, `property_service.rs`.

- **Export service** – transforms canvas state into multiple formats (Leptos, HTML, JSON, Markdown) with unit tests.
- **Project service** – `project_to_json` and `project_from_json` for project export/import.
- **Git service** – `GitBackend` trait and `NoopGitBackend` stub for future real Git integration (HTTP/Tauri).
- **Property service** – functions like `update_button_prop`, `update_text_prop`, `update_input_prop` that accept `PropValue` and return updated components.

Services have no knowledge of Leptos views; they only operate on domain/state types.

### 1.4 Builder / UI

Main files: `src/app.rs`, `src/builder/*`.

- `App` (in `src/app.rs`) initializes `AppState`, attaches keyboard handlers, and renders the main layout (Sidebar + Canvas + panels).
- `builder/canvas` contains `Canvas` and `ComponentRenderer` components for rendering `CanvasComponent` to HTML.
- `builder/drag_drop.rs` manages drag-and-drop state.
- `builder/sidebar.rs` displays the component library and custom component editor.
- `builder/property_editor.rs` displays the schema-driven Property Editor (Button, Text, Input).
- `builder/git_panel.rs`, `builder/project.rs`, `builder/debug_panel.rs`, and `builder/command_palette.rs` are additional feature panels.

### 1.5 Utils & Styling

- `utils/clipboard.rs` – clipboard operations using Web API with error handling.
- `utils/format.rs` – helpers for string and file size formatting.
- `builder/design_tokens.rs` – design token to CSS variable mapping.
- `style.css` – CSS classes and variable usage for visual consistency.

---

## 2. Main Data Flows

This section summarizes key data flows in the application.

### 2.1 Drag & Drop Component to Canvas

1. User drags an item from Sidebar (`LibraryComponent`).
2. `drag_drop` updates `DragState` and captures the selected component data.
3. `Canvas` receives the drop event through `DropZone`.
4. Canvas creates a new `CanvasComponent` from the dragged information and adds it to canvas state in `AppState`.
5. This change is recorded in history to support undo/redo.

### 2.2 Schema-driven Property Editor

1. When user selects a component in Canvas, `canvas_state.selected` is updated.
2. `PropertyEditor` reads the selected `CanvasComponent` and its type (Button/Text/Input).
3. `PropertyEditor` finds the matching `LibraryComponent` in `UiState.component_library` and retrieves its `props_schema`.
4. For each `PropSchema`:
   - The current property value is determined from the domain component.
   - A generic input is rendered based on `prop_type` (`string`, `bool`, `enum:...`).
5. `on:input` / `on:change` events call `update_*_prop` in `property_service`, which returns the updated component.
6. `canvas_state.update_component` replaces the old component with the new version.

### 2.3 Export Project

1. User opens the export modal (via button or keyboard shortcut).
2. `App` prepares a snapshot of the project state (canvas + metadata) via `AppState` helpers.
3. `export_service` transforms the snapshot into the desired format (Leptos/HTML/JSON/Markdown).
4. The exported result can be:
   - Copied to clipboard (via `utils::clipboard`), or
   - Downloaded as a file (using browser data URLs).

### 2.4 Project Save/Load

1. Project Panel (`builder/project.rs`) provides `New`, `Export JSON`, and `Import JSON` buttons.
2. `New` creates an empty project and applies it to `AppState`.
3. `Export JSON` uses `project_service::project_to_json` to create a JSON representation; user can copy or download.
4. `Import JSON` uses `project_service::project_from_json`; if successful, applies the result to `AppState` via `apply_project`.

### 2.5 Git Panel

1. Git Panel (`builder/git_panel.rs`) uses the `GitBackend` trait from `git_service`.
2. Current runtime implementation is `NoopGitBackend` (performs no real operations), but the API is available:
   - status,
   - log,
   - commit, etc.
3. In the future, real implementations can be injected (e.g., via HTTP/Tauri) without major UI changes.

### 2.6 Keyboard Shortcuts

1. `builder/keyboard.rs` defines `KeyboardAction` and shortcuts (Undo, Redo, Save, Delete, Copy, Paste, Duplicate, OpenCommandPalette, Export, etc.).
2. `App` captures keyboard events and maps them to actions on `AppState` (e.g., calling history functions, opening panels).
3. The builder layer only maps user input → calls to state/services functions.

---

## 3. Design Principles

Several principles already reflected in the current codebase:

- **Separation of concerns** – domain, state, services, and UI are separated to enable testing and code evolution.
- **Schema-driven editing** – Property Editor for Button/Text/Input leverages `props_schema` + `PropValue` for more generic and extensible UI.
- **Testability** – many critical logic paths (especially export and domain) have unit tests.
- **Extensibility** – traits like `GitBackend`, `CustomComponent`, and `ComponentRegistry` are designed so new integrations (Git backend, custom component types) can be added without breaking the base architecture.
