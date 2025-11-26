# Changelog

All notable changes to Leptos Studio will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Documentation
- Complete English translation of all documentation (README, ARCHITECTURE)
- New `FEATURES.md` - Comprehensive guide to advanced features
- New `DEVELOPMENT.md` - Developer guide for contributing and extending
- Enhanced inline code comments in all modules

#### Responsive Design Preview
- **Mobile viewport** (375px × 667px) for smartphone design
- **Tablet viewport** (768px × 1024px) for tablet design
- **Desktop viewport** (1920px × 1080px) for full desktop design
- Visual viewport selector buttons in top navigation
- Floating indicator showing current viewport size and dimensions
- Automatic canvas resizing and component adaptation

**Module**: `src/builder/responsive_preview.rs`
**Components**: `ResponsivePreviewControls`, `CanvasViewport`, `ResponsiveIndicator`

#### Enhanced Component Library Management
- **Search functionality** - Fuzzy search across component names, kinds, and categories
- **Category filtering** - Organize components by type (Basic, Input, Container, Custom, All)
- **Component cards** - Improved visual representation with icons and metadata
- **Favorites system foundation** - Ready for marking frequently used components
- Better component discovery and organization

**Module**: `src/builder/component_library_enhanced.rs`
**Components**: `ComponentCard`, `LibrarySearchBar`, `CategoryFilter`, `ComponentFavorite`
**Utilities**: `search_components`, `get_categories`, `ComponentCategory` enum

#### Advanced Styling System
- **Visual CSS property editor** supporting 13+ CSS properties:
  - Layout: padding, margin, width, height, display, flex-direction, gap
  - Colors: background-color, border-color
  - Sizing: border-width, border-radius
  - Typography: font-size, font-weight, text-align
- **Theme presets**:
  - Light - Clean, professional light theme
  - Dark - Dark mode with high contrast
  - High Contrast - Maximum accessibility (WCAG compliant)
  - Colorful - Vibrant, eye-catching colors
  - Minimal - Subtle, minimal styling
- **CSS string generation** - Auto-generate inline CSS from properties
- **Custom CSS class support** - Apply custom classes to components
- **Style persistence** - Styles saved with components

**Module**: `src/builder/styling_system.rs`
**Components**: `StyleEditor`, `ThemeSelector`
**Types**: `ComponentStyle`, `ThemePreset`

#### Component Constraints & Guidelines
- **Size constraints** with validation:
  - Min/max width and height enforcement
  - Aspect ratio constraints
  - Predefined constraints for common components (Button, Input, Container, Text)
- **Layout alignment options**:
  - FlexStart, Center, FlexEnd
  - SpaceBetween, SpaceAround, SpaceEvenly
  - CSS Flexbox compatibility
- **Design guidelines**:
  - Button guidelines (Primary, Secondary, Icon buttons)
  - Input guidelines (Text input, Large textarea)
  - Spacing guidelines (Compact, Default, Comfortable, Loose)
- **Responsive breakpoints**:
  - Mobile (320px), Small (480px), Tablet (768px)
  - Desktop (1024px), Large (1280px), Extra Large (1920px)
- **Grid systems**:
  - Bootstrap 12-column grid (16px gap, 1200px width)
  - Material 8-column grid (8px gap, 1200px width)
  - Column width calculator

**Module**: `src/builder/component_constraints.rs`
**Types**: `SizeConstraints`, `AlignmentOption`, `DesignGuideline`, `ResponsiveBreakpoint`, `GridSystem`

#### Hot Reload for Custom Components
- Real-time updates when modifying custom component templates
- No page refresh required for template changes
- Automatic validation and error feedback
- Canvas updates immediately reflect new templates
- Confirmation notifications for successful updates

#### Improved Error Handling & Validation
- Comprehensive validation framework:
  - Component name validation (alphanumeric + underscore)
  - HTML template validation
  - Duplicate name detection
- User-friendly error messages with specific guidance
- Multiple notification types:
  - Success (green) ✅
  - Error (red) ❌
  - Warning (yellow) ⚠️
  - Info (blue) ℹ️
- Clear, actionable error display in snackbar notifications

### Changed

- Enhanced state management with new reactive signals
- Improved UI/UX with responsive controls and better component organization
- More consistent error handling across all operations
- Better CSS organization with semantic variables
- Updated main navigation with responsive controls

### Fixed

- Improved component library performance with search optimization
- Better handling of custom component updates
- More reliable state synchronization

## [Previous Versions]

### [0.1.0] - Initial Release

- Drag & drop component builder
- Live preview
- Project save/load
- Code export (Leptos, HTML, Markdown, JSON)
- Custom component creation
- Undo/redo system
- Command palette
- Git panel (stub)
- Debug panel
- Responsive design tokens
- Comprehensive testing

---

## Legend

- **Added** for new features
- **Changed** for changes in existing functionality
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** in case of vulnerabilities

---

## Future Roadmap

### Planned Features

- [ ] Full Git integration backend
- [ ] Advanced component library with sharing/importing
- [ ] Real-time collaborative editing
- [ ] Theme customization UI
- [ ] Component animation editor
- [ ] Layout grid visualization
- [ ] Component documentation generator
- [ ] Export to React, Vue, Angular
- [ ] VSCode extension
- [ ] Figma plugin

### Experimental

- [ ] AI-assisted component generation
- [ ] Automatic accessibility checking
- [ ] Performance analysis
- [ ] Design system tokenization
