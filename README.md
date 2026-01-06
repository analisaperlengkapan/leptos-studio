# Leptos Studio

[![CI](https://github.com/analisaperlengkapan/leptos-studio/workflows/CI/badge.svg)](https://github.com/analisaperlengkapan/leptos-studio/actions/workflows/ci.yml)
[![Deploy](https://github.com/analisaperlengkapan/leptos-studio/workflows/Deploy/badge.svg)](https://github.com/analisaperlengkapan/leptos-studio/actions/workflows/deploy.yml)
[![Security](https://github.com/analisaperlengkapan/leptos-studio/workflows/Security/badge.svg)](https://github.com/analisaperlengkapan/leptos-studio/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/analisaperlengkapan/leptos-studio/branch/main/graph/badge.svg)](https://codecov.io/gh/analisaperlengkapan/leptos-studio)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![WASM](https://img.shields.io/badge/WASM-ready-brightgreen.svg)](https://webassembly.org/)
[![Status](https://img.shields.io/badge/status-experimental-orange.svg)](#experimental-notice)
[![Stability](https://img.shields.io/badge/stability-unstable-yellow.svg)](#experimental-notice)

---

## ⚠️ Experimental Notice {#experimental-notice}

> **🚧 This project is experimental and under active development.**
>
> - **Not production-ready**: APIs, features, and behavior may change without notice
> - **Unstable**: Expect breaking changes between versions
> - **Work in progress**: Some features are incomplete or may have bugs
> - **Use at your own risk**: Not recommended for production applications
>
> We welcome feedback and contributions! See [Contributing](#contributing) to get involved.

---

> A visual UI builder for the [Leptos](https://github.com/leptos-rs/leptos) framework (Rust + WASM)

Leptos Studio is a visual UI builder that allows you to compose layouts with drag & drop, manage components, and export code in multiple formats. It aims to streamline the development process for Leptos applications by providing a rich, interactive design environment.

## Key Features

### 🎨 Visual Layout Building
- **Canvas Drag & Drop**: Compose layouts intuitively by dragging components (Button, Text, Input, Container) onto the canvas.
- **Nested Structures**: Support for complex nested layouts using Containers (Flex/Grid).
- **Component Selection**: Click to select and edit component properties.

### 🧩 Comprehensive Component System
- **Enhanced Component Library**:
  - **Fuzzy Search**: Quickly find components by name, kind, or category.
  - **Categorization**: Organized into Basic, Input, Container, and Custom categories.
  - **Favorites**: Mark frequently used components for quick access.
- **Custom Components**:
  - Create reusable components using HTML templates.
  - **Hot Reload**: Real-time updates on the canvas when you modify custom component templates.
  - **Validation**: Automatic validation for component names and template syntax.

### 📱 Responsive Design Tools
- **Responsive Preview**: Design for multiple viewports simultaneously.
  - **Mobile**: 375px × 667px
  - **Tablet**: 768px × 1024px
  - **Desktop**: 1920px × 1080px
- **Visual Feedback**: Components automatically adapt to the selected viewport size.

### 💅 Advanced Styling System
- **Visual Style Editor**: Edit CSS properties without writing code.
  - Controls for padding, margin, dimensions, typography, borders, and colors.
  - Flexbox and layout controls.
- **Theme Presets**: Apply pre-configured themes (Light, Dark, High Contrast, Colorful, Minimal) instantly.
- **CSS Generation**: Auto-generates inline CSS or class-based styles.

### ⚡ Command Palette
- **Quick Actions**: Access all editor commands via a VS Code-style palette (Ctrl+K).
- **Search**: Fuzzy search for commands and components.
- **Sync**: Synchronized with design tokens and global variables.

### 💾 Project Management
- **Local Persistence**: Projects are automatically saved to LocalStorage.
- **JSON Import/Export**: Save full project state (layout + settings) to JSON files for backup or sharing.
- **Undo/Redo History**: Robust history system with snapshots for every action.

### 📤 Multi-Format Code Export
Export your visual designs to production-ready code:
- **Leptos**: Native Rust/Leptos component code (`LeptosCodeGenerator`).
- **HTML/Tailwind**: Standard HTML with Tailwind CSS classes.
- **React/JSX**: React component syntax.
- **Svelte**: Svelte component syntax.
- **TypeScript Types**: Type definitions for your components.
- **JSON Schema**: Schema validation for your data structures.
- **Markdown**: Documentation generation.

### 🛡️ Design Constraints & Validation
- **Constraints System**: Enforces valid component configurations (e.g., min/max sizes, nesting rules).
- **Real-time Validation**: Immediate feedback on invalid inputs or structures.
- **Error Handling**: User-friendly notifications (Snackbars) for all operations.

### 🖼️ Template Gallery
Kickstart your project with pre-built layout templates:
- **Common Patterns**: Login/Contact Forms, Hero Sections, Navigation Bars, Pricing Cards.
- **Dashboard UI**: Headers, Feature Grids, Filtering, and Search layouts.

### ♿ Accessibility Features
Built-in tools to ensure your designs are accessible:
- **ARIA Integration**: Support for ARIA labels and roles.
- **Navigation**: Skip links and keyboard navigation support.
- **Focus Management**: Focus trap for modals and screen reader live region announcements.

### 🔧 Debug & Developer Tools
- **Debug Panel**: Monitor component counts, render performance metrics (`render_count`, `render_time`), and undo/redo state.
- **Status Bar**: Real-time feedback on nesting depth, selection status, and active responsive mode.

## Architecture

Leptos Studio is built with a modular architecture:

- **`src/domain/`**: Core data models (`CanvasComponent`, validators, errors).
- **`src/state/`**: Global state management (`AppState`, `CanvasState`, `UiState`) using Leptos signals.
- **`src/builder/`**: UI components for the editor (Canvas, PropertyEditor, etc.).
- **`src/services/`**: Business logic for export, persistence, and Git integration.

## Getting Started

### Prerequisites

- **Rust Toolchain**: Stable release.
- **WASM Target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install trunk` (WASM application bundler)

### Running Locally

1. **Clone the repository**:
   ```bash
   git clone https://github.com/analisaperlengkapan/leptos-studio.git
   cd leptos-studio
   ```

2. **Start the development server**:
   ```bash
   trunk serve
   ```

3. **Open in Browser**:
   Visit `http://localhost:8899` to start building.

## Testing

### Unit Tests
Run standard Rust unit tests:
```bash
cargo test
```

### WASM Tests
Run tests in a headless browser environment:
```bash
cargo test --target wasm32-unknown-unknown
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Setting up your development environment.
- Code style guidelines.
- Pull request process.

## Security

Security is a priority. Please review our [Security Policy](SECURITY.md) for responsible disclosure guidelines.

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.

---

<div align="center">
  
**[Website](https://analisaperlengkapan.github.io/leptos-studio)** • 
**[Documentation](DOCUMENTATION.md)** • 
**[Contributing](CONTRIBUTING.md)** • 
**[Issues](https://github.com/analisaperlengkapan/leptos-studio/issues)**

Made with ❤️ by the Leptos Studio team

</div>
