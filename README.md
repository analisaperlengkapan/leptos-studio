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

Leptos Studio is a visual UI builder that allows you to compose layouts with drag & drop, manage components, and export code in multiple formats.

## Key Features

- **Canvas Drag & Drop**  
  Compose layouts with Button, Text, Input, Container, and Custom components.

- **Custom Components**  
  Add custom components based on HTML templates with name and template validation.

- **Command Palette**  
  Quick command search (VS Code style) with fuzzy search, synchronized with design tokens & CSS variables.

- **Project Management**  
  Project Panel for:
  - `New` project (reset layout & history).
  - `Copy JSON` / `Download JSON` project (layout + settings).
  - `Import JSON` to restore a project.

- **Export Code**  
  Export layouts to multiple formats:
  - Leptos code (`LeptosCodeGenerator`)
  - HTML / HTML + Tailwind CSS
  - React/JSX Component
  - Svelte Component
  - TypeScript Types
  - JSON Schema
  - Markdown (documentation)
  - JSON (CanvasComponent structure)

- **Template Gallery** 🆕  
  Pre-built layout templates for quick-start designs:
  - Login/Contact Forms
  - Hero Sections
  - Navigation Bars
  - Pricing Cards
  - Dashboard Headers
  - Feature Grids
  - Category filtering and search

- **Status Bar** 🆕  
  Bottom status bar showing:
  - Component count
  - Nesting depth
  - Selection status
  - Undo/Redo availability
  - Responsive mode indicator
  - Theme indicator
  - Render performance metrics

- **Accessibility Features** 🆕  
  - Skip links for keyboard navigation
  - ARIA labels and roles
  - Screen reader live region announcements
  - Focus trap for modals
  - Keyboard shortcuts

- **Responsive Design Tools** 🆕  
  - Custom breakpoint editor
  - Responsive preview controls
  - Device-specific preview modes (Desktop, Tablet, Mobile)

- **Debug Panel**  
  Display component count, custom components, undo/redo capabilities, and render metrics (`render_count`, `render_time`).

- **Git Panel (stub)**  
  Git Panel uses the `GitBackend` abstraction with a default `NoopGitBackend` implementation that is safe for browser-only mode. The panel provides `Status`, `Log`, `Commit`, and `Push` buttons (not yet connected to a real backend).

## Running the Project

### Prerequisites

- Rust toolchain (stable)
- `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
```

- [Trunk](https://trunkrs.dev/) for WASM dev server:

```bash
cargo install trunk
```

### Development Server

From the project directory:

```bash
trunk serve
```

By default, Trunk serves the application at `http://localhost:8899` (see `Trunk.toml`).

## Testing

### Unit & Integration Tests

Run all native tests:

```bash
cargo test
```

### WASM Tests

The project includes WASM tests (e.g., `tests/wasm_smoke.rs`) that verify export services in the `wasm32` environment:

```bash
rustup target add wasm32-unknown-unknown
cargo test --target wasm32-unknown-unknown
```

CI (recommended) should run both commands above.

## Architecture Overview

Main crate structure:

- `src/domain/`  
  Domain models: `CanvasComponent` and its variants (`ButtonComponent`, `TextComponent`, `InputComponent`, `ContainerComponent`, `CustomComponent`), errors (`AppError`, `ValidationError`), and validators.

- `src/state/`  
  Global `AppState` (canvas, UI, settings, project), `CanvasState`, `UiState`, `SettingsState`, undo/redo history, and persistence (LocalStorage).  
  `Project` represents layout + settings that can be exported to JSON.

- `src/services/`  
  - `export_service`: code generators (Leptos/HTML/JSON/Markdown) + unit tests.  
  - `project_service`: serialize/deserialize `Project` to/from JSON.  
  - `git_service`: `GitBackend` trait + `NoopGitBackend` for Git.

- `src/builder/`  
  Main UI components: `Canvas`, `Sidebar`, `PropertyEditor`, `Preview`, `CommandPalette`, `DebugPanel`, `GitPanel`, `ProjectPanel`, drag & drop tools (`DragState`, `DropZone`), etc.

- `src/utils/`  
  Common utilities, e.g., clipboard operations.

- `style.css`  
  Design tokens and global styling, including semantic CSS variables for Command Palette and Canvas/Sidebar styling.

## Development Notes

- **ComponentRegistry**  
  Small helper in `builder::component_library` for operations on `LibraryComponent` (e.g., checking for duplicate names and filtering custom components from `component_library`).

- **Undo/Redo**  
  Canvas stores `History<Snapshot>` to support layout undo/redo.

- **Error Handling & Notifications**  
  All important operations use `AppError::user_message()` to display user-friendly messages via `Notification` and `Snackbar`.

## Future Ideas

- Real implementation of `GitBackend` (HTTP backend or Tauri) to connect GitPanel with repositories.
- Extend `ComponentRegistry` and props schema so Property Editor can be more generic.
- Detailed architecture documentation (`docs/` or `ARCHITECTURE.md`) as the project evolves.

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- How to set up your development environment
- Code style guidelines
- Commit message conventions
- Pull request process
- Testing requirements

### Quick Start for Contributors

```bash
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/leptos-studio.git
cd leptos-studio

# Install dependencies
rustup target add wasm32-unknown-unknown
cargo install trunk

# Run development server
trunk serve

# Run tests
cargo test
cargo test --target wasm32-unknown-unknown
```

## Security

Security is a top priority. If you discover a vulnerability, please review our [Security Policy](SECURITY.md) for responsible disclosure guidelines.

For security-related questions or concerns, please do not open public issues.

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.

### Apache 2.0 License Summary

- ✅ Commercial use
- ✅ Modification
- ✅ Distribution
- ✅ Patent use
- ✅ Private use
- ℹ️ License and copyright notice required
- ℹ️ State changes

## Acknowledgments

- Built with [Leptos](https://github.com/leptos-rs/leptos) - A cutting-edge Rust framework for building web applications
- Powered by [WebAssembly](https://webassembly.org/) for optimal performance
- Inspired by modern visual builders and the Rust ecosystem

## Project Status

🧪 **Experimental / Pre-Alpha** - This project is in its early stages and under active development.

### Current State

| Aspect | Status |
|--------|--------|
| **Stability** | ⚠️ Unstable - Breaking changes expected |
| **API** | 🔄 Subject to change |
| **Features** | 🚧 Many features are incomplete |
| **Documentation** | 📝 Work in progress |
| **Testing** | ✅ Unit tests available |
| **Production Use** | ❌ Not recommended |

### What Works

- ✅ Basic drag-and-drop UI composition
- ✅ Component property editing
- ✅ Code export (Leptos, HTML, JSON, Markdown)
- ✅ Project save/load (localStorage)
- ✅ Undo/redo functionality
- ✅ Custom component support

### Known Limitations

- 🚧 Git integration is a stub (no real backend)
- 🚧 Responsive preview is basic
- 🚧 Limited component library
- 🚧 No real-time collaboration
- 🚧 No cloud storage
- 🚧 Component templates need expansion

### Roadmap

See our [GitHub Issues](https://github.com/analisaperlengkapan/leptos-studio/issues) for planned features and known issues.

### Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each release.

---

<div align="center">
  
**[Website](https://analisaperlengkapan.github.io/leptos-studio)** • 
**[Documentation](DOCUMENTATION.md)** • 
**[Contributing](CONTRIBUTING.md)** • 
**[Issues](https://github.com/analisaperlengkapan/leptos-studio/issues)**

Made with ❤️ by the Leptos Studio team

</div>
