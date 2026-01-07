# Leptos Studio

**Leptos Studio** is a comprehensive visual UI builder designed for the [Leptos](https://github.com/leptos-rs/leptos) web framework. It empowers developers to construct user interfaces visually, manage component state, and export production-ready Rust code, all within a browser-based environment.

## 🚀 Features

### Visual Editor
*   **Drag-and-Drop Canvas**: Build layouts intuitively by dragging components from the palette.
*   **Component Library**: Includes a robust set of standard components:
    *   **Basics**: Button, Text, Input, Image.
    *   **Layout**: Container, Card (with built-in styling).
    *   **Forms**: Select, Input.
    *   **Custom**: Support for defining custom component templates.
*   **Responsive Preview**: Test designs instantly across Desktop, Tablet, and Mobile viewports.

### Design & Customization
*   **Property Editor**: Detailed control over component attributes, styling, and layout properties.
*   **Theme Editor**: Manage global design tokens, including colors and typography.
*   **Animation Support**: Configure animations (Fade, Slide, Bounce, etc.) directly in the editor.
*   **Styling System**: leverage CSS variables and Flexbox layouts for responsive designs.

### Developer Tools
*   **Code Export**: Generate idiomatic Rust/Leptos code from your visual designs.
*   **Git Integration**: Built-in version control system (simulated via LocalStorage) allowing you to commit, view history, and restore previous states.
*   **Undo/Redo**: Full history stack for safe experimentation.
*   **Tree View**: Hierarchical navigation of your component structure.
*   **Debug Panel**: Inspect internal application state and performance metrics.

## 🛠️ Tech Stack

*   **Language**: [Rust](https://www.rust-lang.org/) (Edition 2021)
*   **Framework**: [Leptos](https://leptos.dev/) (CSR - Client Side Rendering)
*   **Build Tool**: [Trunk](https://trunkrs.dev/)
*   **Storage**: Browser LocalStorage (for project persistence and Git simulation)
*   **WASM Target**: `wasm32-unknown-unknown`

## 🏁 Getting Started

For a detailed guide, please refer to [QUICKSTART.md](QUICKSTART.md).

### Prerequisites

*   [Rust](https://rustup.rs/) (latest stable)
*   WASM target: `rustup target add wasm32-unknown-unknown`
*   [Trunk](https://trunkrs.dev/): `cargo install --locked trunk`

### Installation & Running

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/analisaperlengkapan/leptos-studio.git
    cd leptos-studio
    ```

2.  **Run the development server**:
    ```bash
    trunk serve
    ```

3.  **Open the application**:
    Navigate to `http://localhost:8899` in your browser.

## 📂 Project Structure

For an in-depth architecture overview, see [DEVELOPMENT.md](DEVELOPMENT.md).

*   **`src/app.rs`**: Main application entry point and layout.
*   **`src/builder/`**: Core UI components for the builder (Canvas, Palette, Property Editor, etc.).
*   **`src/domain/`**: Data models (Component definitions, Validation logic).
*   **`src/services/`**: Business logic (Git, Export, Project management).
*   **`src/state/`**: Global state management using Leptos signals.
*   **`src/utils/`**: Helper functions and utilities.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to get started, run tests, and submit pull requests.

## 📄 License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.
