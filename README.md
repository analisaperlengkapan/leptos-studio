# Leptos Studio

A visual UI builder for the [Leptos](https://github.com/leptos-rs/leptos) web framework. Build reactive web interfaces with drag-and-drop ease, then export clean, idiomatic Rust code.

## Features

- **Visual Editor**: Drag and drop components (Buttons, Inputs, Containers, etc.) onto a canvas.
- **Component Library**: Rich set of built-in components and support for custom templates.
- **Variable Management**: Define global variables and bind them to component properties for dynamic UIs.
- **Theme Editor**: Customize global design tokens (Colors, Typography, Spacing, Border Radius) visually.
- **Responsive Preview**: Test your design on Mobile, Tablet, and Desktop viewports.
- **History**: Robust Undo/Redo system with "Time Travel" to restore any previous state.
- **Code Export**: Generate production-ready Leptos Rust code, HTML, or JSON.
- **Project Management**: Create, save, and manage multiple projects.
- **Command Palette**: Quick access to all actions via `Ctrl+K` / `Cmd+K`.
- **Auto-Save**: Never lose your work with configurable auto-save.

## Architecture

The project is structured as a Cargo workspace:

- **`frontend/`**: The Leptos WebAssembly application.
    - Uses `leptos_router` for navigation (`/`, `/editor/:id`).
    - Uses `async_trait` for pluggable Git backends (Remote vs LocalStorage).
- **`backend/`**: Axum-based API server.
    - Handles persistence for Projects, Templates, Git history, and Analytics.
    - Stores data in simple JSON files for portability.

## Getting Started

### Prerequisites

- Rust (latest stable)
- `trunk` (for frontend build): `cargo install trunk`

### Development

1.  **Start the Backend:**
    ```bash
    cd backend
    cargo run
    ```

2.  **Start the Frontend:**
    ```bash
    cd frontend
    trunk serve
    ```

3.  Open [http://localhost:8899](http://localhost:8899) in your browser.

## Keyboard Shortcuts

- **General**:
    - `Ctrl + K`: Open Command Palette
    - `Ctrl + S`: Save Project
    - `Ctrl + E`: Export Code
    - `?`: Show Shortcuts Help
- **Editing**:
    - `Ctrl + Z`: Undo
    - `Ctrl + Y`: Redo
    - `Ctrl + C / V`: Copy / Paste
    - `Delete`: Delete Selected
- **Selection**:
    - `Ctrl + A`: Select All
    - `Esc`: Deselect

## Contributing

1.  Fork the repository.
2.  Create a feature branch.
3.  Commit your changes.
4.  Push to the branch.
5.  Open a Pull Request.

## License

Apache-2.0
