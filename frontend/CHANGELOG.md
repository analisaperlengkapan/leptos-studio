# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Backend Services
- **Event Bus System** (`services/event_bus.rs`)
  - Centralized event system for decoupled component communication
  - AppEvent enum with variants: ComponentAdded, ComponentRemoved, SelectionChanged, etc.
  - Subscriber pattern for reactive event handling

- **Template Service** (`services/template_service.rs`)
  - Pre-built layout templates for quick-start designs
  - 8 built-in templates: Login Form, Contact Form, Hero Section, Pricing Card, Navigation Bar, Footer, Dashboard Header, Feature Grid
  - Template categories: Form, Hero, Navigation, Card, Dashboard, Footer, LandingPage, Custom
  - Search and filter functionality

- **Analytics Service** (`services/analytics_service.rs`)
  - Usage metrics tracking (component operations, export counts)
  - Performance metrics collection
  - Session information management

- **Advanced Export Generators** (`services/export_advanced.rs`)
  - JSON Schema generator for component validation
  - TypeScript types generator
  - React/JSX component generator
  - HTML + Tailwind CSS generator
  - Svelte component generator
  - Vue component generator (bonus)
  - CSS extraction generator (bonus)

#### State Management
- **Derived State** (`state/derived.rs`)
  - Memoized computed values for efficient reactive updates
  - Component count, type counts, nesting depth calculations
  - Selection state, undo/redo availability signals

#### UI Components
- **Template Gallery** (`builder/template_gallery.rs`)
  - Visual gallery for browsing and applying templates
  - Category filtering and search
  - Template preview on hover
  - One-click application to canvas

- **Status Bar** (`builder/status_bar.rs`)
  - Component count indicator
  - Nesting depth display
  - Selection status
  - Undo/Redo quick buttons
  - Responsive mode indicator
  - Theme indicator
  - Render performance metrics

- **Component Palette** (`builder/component_palette.rs`)
  - Enhanced component browser with fuzzy search
  - Category tabs
  - Keyboard navigation support
  - Component favorites

- **Accessibility Features** (`builder/accessibility.rs`)
  - AccessibilityProvider wrapper component
  - SkipLink for keyboard navigation
  - LiveRegion for screen reader announcements
  - FocusTrap for modal dialogs
  - ProgressBar with ARIA attributes
  - Tooltip with accessible descriptions
  - VisuallyHidden and KeyboardHint components

- **Breakpoint Editor** (`builder/breakpoint_editor.rs`)
  - Custom responsive breakpoint configuration
  - Visual breakpoint preview ruler
  - Add/edit/delete custom breakpoints
  - Responsive wrapper component

### Changed
- **Export Modal** enhanced with grouped format options
  - Framework Code: Leptos, React/JSX, Svelte
  - Web Output: HTML, HTML + Tailwind CSS
  - Data Formats: JSON, JSON Schema, TypeScript Types
  - Documentation: Markdown
- **App Layout** now includes StatusBar at bottom
- **App Header** includes Templates button
- **Main wrapper** uses AccessibilityProvider for a11y features
- All interactive elements have ARIA labels and roles

### Improved
- Overall accessibility with skip links and screen reader support
- Export functionality with 5 additional formats
- State management with derived/memoized signals
- Component organization with event-driven architecture

---

## [0.1.0] - 2025-11-26

Initial release of Leptos Studio.

### Added
- Visual UI builder for Leptos framework
- Canvas with drag & drop functionality
- Component library (Button, Text, Input, Container, Custom)
- Property editor
- Command palette
- Project management (New, Import, Export)
- Code export (Leptos, HTML, Markdown, JSON)
- Debug panel
- Git panel (stub implementation)
- Undo/redo functionality
- LocalStorage persistence
- Comprehensive GitHub Actions CI/CD pipeline
- Dependabot configuration for automated dependency updates
- Security scanning workflows (CodeQL, cargo-audit, cargo-deny)
- Automated release workflow
- WASM size and performance benchmarking
- PR quality checks (size labels, conventional commits, TODO detection)
- Automated stale issue/PR management
- Markdown link checking
- Issue and PR templates
- CODEOWNERS file
