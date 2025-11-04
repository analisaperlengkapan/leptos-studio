# Changelog

All notable changes to Leptos Studio will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Known Issues
- Build errors with command palette and app.rs (closure trait issues)
- Some UI components need refactoring for Leptos 0.8 compatibility

## [0.1.0] - 2025-08-25

### Added
- Export presets: Plain, thaw-ui, leptos-material, leptos-use
- Flexible export system for HTML, Markdown, and Leptos code
- Integration tests for export presets
- Property-based testing for edge cases
- Unicode and emoji support in component properties
- Stress testing for deeply nested layouts

### Changed
- Refactored export functions to accept preset parameter
- Updated sidebar with export preset dropdown
- Improved clipboard integration using web-sys

### Fixed
- Build and test warnings
- Clipboard copy functionality
- Export signature consistency

## [0.0.9] - 2024-08-22

### Added
- Component library management (add/remove custom components)
- Component props validation (Rust identifier names, HTML templates)
- Theme switcher (Light/Dark/Custom)
- Responsive design preview (Desktop/Tablet/Mobile)
- Error messages for invalid component input
- Sidebar and canvas refactoring

### Changed
- More modular and maintainable sidebar/canvas code
- Props now fully utilized in components
- Improved UI feedback

### Fixed
- Unused props and handler warnings
- Build warnings related to component macros

## [0.0.8] - 2024-08-20

### Added
- Initial release
- Drag and drop UI builder
- Basic components: Button, Text, Input, Container
- Real-time property editor
- Live preview panel
- Nested container support
- Custom component system
- Layout persistence with localStorage
- Code export to Leptos format
- Undo/redo functionality
- Basic keyboard shortcuts

### Technical
- Built with Rust and Leptos 0.8
- WASM compilation via Trunk
- Reactive state management with signals
- Drag and drop using web-sys APIs

## [0.0.7] - 2024-08-15

### Added
- Project initialization
- Basic project structure
- Development environment setup
