# Leptos Studio

[![CI](https://github.com/analisaperlengkapan/leptos-studio/workflows/CI/badge.svg)](https://github.com/analisaperlengkapan/leptos-studio/actions/workflows/ci.yml)
[![Deploy](https://github.com/analisaperlengkapan/leptos-studio/workflows/Deploy/badge.svg)](https://github.com/analisaperlengkapan/leptos-studio/actions/workflows/deploy.yml)
[![Security](https://github.com/analisaperlengkapan/leptos-studio/workflows/Security/badge.svg)](https://github.com/analisaperlengkapan/leptos-studio/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/analisaperlengkapan/leptos-studio/branch/main/graph/badge.svg)](https://codecov.io/gh/analisaperlengkapan/leptos-studio)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![WASM](https://img.shields.io/badge/WASM-ready-brightgreen.svg)](https://webassembly.org/)

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
  - HTML
  - Markdown (documentation)
  - JSON (CanvasComponent structure)

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

🚀 **Active Development** - This project is under active development. Features and APIs may change.

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
