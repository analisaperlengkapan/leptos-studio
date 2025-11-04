# Leptos Studio

A visual UI builder for the [Leptos](https://leptos.dev/) Rust web framework. Drag and drop components onto a canvas, configure their properties, and export Leptos code.

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)

## Features

- **Drag & Drop Interface**: Build UI by dragging components onto a canvas
- **Component Library**: Button, Text, Input, Container, and custom components
- **Property Editor**: Configure component properties in real-time
- **Live Preview**: See changes immediately as you build
- **Code Export**: Generate Leptos code with multiple presets (Plain, thaw-ui, leptos-material, leptos-use)
- **Layout Persistence**: Save and load layouts using browser localStorage
- **Nested Containers**: Create complex layouts with nested components
- **Custom Components**: Define and reuse your own components
- **Undo/Redo**: Full history management for all operations
- **Keyboard Shortcuts**: Standard shortcuts for copy, paste, delete, undo, redo
- **Responsive Preview**: Toggle between Desktop, Tablet, and Mobile views
- **Theme Support**: Light, Dark, and Custom themes

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.90 or later
- [Trunk](https://trunkrs.dev/) for building WASM applications

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Running Locally

```bash
# Clone the repository
git clone https://github.com/analisaperlengkapan/leptos-studio.git
cd leptos-studio

# Start development server
trunk serve

# Open http://localhost:8080 in your browser
```

### Building for Production

```bash
# Build optimized WASM bundle
trunk build --release

# Output will be in the dist/ directory
```

## Usage

1. **Add Components**: Drag components from the sidebar onto the canvas
2. **Configure Properties**: Click a component to edit its properties in the sidebar
3. **Nest Components**: Drag components into Container components for complex layouts
4. **Export Code**: Click the Export button to generate Leptos code
5. **Save/Load**: Use Save/Load buttons to persist your layout to localStorage

### Keyboard Shortcuts

- `Ctrl+Z` - Undo
- `Ctrl+Y` / `Ctrl+Shift+Z` - Redo
- `Ctrl+C` - Copy selected component
- `Ctrl+V` - Paste component
- `Ctrl+D` - Duplicate selected component
- `Delete` / `Backspace` - Delete selected component
- `Ctrl+S` - Save layout
- `Escape` - Deselect

## Project Structure

```
src/
├── app.rs                 # Main application component
├── lib.rs                 # Library entry point
├── builder/               # Core builder modules
│   ├── canvas.rs          # Canvas with drag & drop
│   ├── sidebar.rs         # Component library and properties
│   ├── preview.rs         # Live preview panel
│   ├── export.rs          # Code generation
│   ├── property_editor.rs # Property editing UI
│   ├── component_library.rs # Component definitions
│   ├── keyboard.rs        # Keyboard shortcut handling
│   ├── command_palette.rs # Command palette UI
│   ├── breadcrumb.rs      # Navigation breadcrumbs
│   ├── git_panel.rs       # Git integration UI
│   └── ...               # Other modules
├── components/            # Reusable UI components
│   ├── button.rs
│   ├── text.rs
│   ├── input.rs
│   └── container.rs
└── tests/                 # Test files
```

## Technology Stack

- **Leptos 0.8**: Reactive UI framework for Rust
- **WASM**: Runs in the browser via WebAssembly
- **Trunk**: Build tool for Rust WASM applications
- **serde/serde_json**: Serialization for layout persistence
- **web-sys**: Web API bindings

## Development

### Testing

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Linting

```bash
# Run clippy
cargo clippy --all-targets --all-features

# Format code
cargo fmt
```

### Optional: Leptos-specific Tools

```bash
# Install leptosfmt for formatting view! macros
cargo install leptosfmt

# Format Leptos code
leptosfmt src/

# Install leptos-lints for additional linting
cargo install cargo-dylint dylint-link
cargo dylint --all
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

Built with [Leptos](https://leptos.dev/) - A performant web framework for Rust.
