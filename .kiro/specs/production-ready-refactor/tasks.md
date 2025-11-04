# Implementation Plan

## Phase 1: Foundation & Domain Layer

- [ ] 1. Setup new module structure
  - Create domain/, state/, services/, utils/ directories
  - Update mod.rs files with proper exports
  - Add necessary dependencies to Cargo.toml (thiserror, uuid, regex)
  - _Requirements: 1.1, 1.2, 1.3_

- [ ] 2. Implement domain models and types
- [ ] 2.1 Create ComponentId type with UUID
  - Implement ComponentId struct with Uuid
  - Add serialization/deserialization support
  - Implement Display and Debug traits
  - _Requirements: 1.1, 2.1_

- [ ] 2.2 Implement component type enum and trait
  - Create ComponentType enum (Button, Text, Input, Container, Custom)
  - Define Component trait with validate() and component_type() methods
  - Add serialization support
  - _Requirements: 1.1, 2.1_

- [ ] 2.3 Create individual component structs
  - Implement ButtonComponent with id, label, variant, disabled fields
  - Implement TextComponent with id, content, style fields
  - Implement InputComponent with id, placeholder, input_type fields
  - Implement ContainerComponent with id, children, layout fields
  - Implement CustomComponent with id, name, template, props fields
  - _Requirements: 1.1, 2.1_

- [ ] 2.4 Implement CanvasComponent enum
  - Create enum with variants for each component type
  - Implement helper methods (id(), component_type())
  - Add Clone, Debug, Serialize, Deserialize derives
  - _Requirements: 1.1, 2.1_

- [ ] 3. Implement validation system
- [ ] 3.1 Create error types with thiserror
  - Define ValidationError enum with descriptive variants
  - Define AppError enum wrapping all error types
  - Implement user_message() for user-friendly error display
  - Create AppResult<T> type alias
  - _Requirements: 2.1, 2.2, 2.4, 2.5_

- [ ] 3.2 Implement Validator trait and validators
  - Define Validator<T> trait with validate() method
  - Implement ComponentNameValidator (alphanumeric + underscore, no leading digit)
  - Implement HtmlTemplateValidator (min length, contains HTML tags)
  - Add validation tests
  - _Requirements: 2.1, 2.5, 8.1, 8.2_

- [ ]* 3.3 Write unit tests for validation
  - Test ComponentNameValidator with valid/invalid names
  - Test HtmlTemplateValidator with valid/invalid templates
  - Test edge cases (empty, special characters, unicode)
  - _Requirements: 6.1, 6.4_

## Phase 2: State Management Layer

- [ ] 4. Implement state management structure
- [ ] 4.1 Create AppState with context provider
  - Define AppState struct with canvas, ui, settings fields
  - Implement provide_context() and use_context() methods
  - Add Clone derive and proper signal types
  - _Requirements: 3.1, 3.2, 3.3_

- [ ] 4.2 Implement CanvasState
  - Create CanvasState with components, selected, history signals
  - Add helper methods for component operations
  - Integrate with Leptos context system
  - _Requirements: 3.1, 3.2_

- [ ] 4.3 Implement UiState
  - Create UiState with modal/panel visibility signals
  - Add notification signal with Notification type
  - Add theme and responsive_mode signals
  - _Requirements: 3.1, 3.2_

- [ ] 4.4 Implement SettingsState with persistence
  - Create SettingsState struct with theme, auto_save, export_preset
  - Implement Persistable trait
  - Add load_or_default() method
  - _Requirements: 3.1, 3.2_

- [ ] 5. Implement history system (undo/redo)
- [ ] 5.1 Create History struct with VecDeque
  - Define Snapshot struct with components, selected, timestamp
  - Implement History with undo_stack and redo_stack
  - Add MAX_HISTORY_SIZE constant (50)
  - _Requirements: 14.1, 14.2, 14.3_

- [ ] 5.2 Implement undo/redo operations
  - Implement push() method with stack size limit
  - Implement undo() method returning previous snapshot
  - Implement redo() method returning next snapshot
  - Add can_undo() and can_redo() helper methods
  - _Requirements: 14.1, 14.3, 14.4_

- [ ]* 5.3 Write unit tests for history system
  - Test push with stack size limit
  - Test undo/redo operations
  - Test can_undo/can_redo states
  - Test edge cases (empty stack, max size)
  - _Requirements: 6.1, 6.3_

- [ ] 6. Implement persistence layer
- [ ] 6.1 Create Persistable trait
  - Define trait with save() and load() methods
  - Implement get_local_storage() helper
  - Add proper error handling with AppError
  - _Requirements: 2.1, 2.2, 8.5_

- [ ] 6.2 Implement persistence for SettingsState
  - Implement Persistable for SettingsState
  - Add storage_key() method
  - Test save/load with LocalStorage
  - _Requirements: 2.1, 2.2_

- [ ] 6.3 Implement persistence for component library
  - Add auto-save effect for component library changes
  - Implement load with validation
  - Handle migration from old format
  - _Requirements: 2.1, 2.2, 8.5, 13.4_

## Phase 3: Services Layer

- [ ] 7. Implement export service
- [ ] 7.1 Create CodeGenerator trait
  - Define trait with generate() and file_extension() methods
  - Create ExportPreset enum (Plain, ThawUi, LeptosMaterial, LeptosUse)
  - Add proper error handling
  - _Requirements: 11.1, 11.2, 11.3_

- [ ] 7.2 Implement LeptosCodeGenerator
  - Implement generate_imports() based on preset
  - Implement generate_component() recursive method
  - Handle all component types (Button, Text, Input, Container, Custom)
  - Format output with proper indentation
  - _Requirements: 11.1, 11.3, 11.4, 11.5_

- [ ] 7.3 Implement HtmlCodeGenerator
  - Create HtmlCodeGenerator struct
  - Implement CodeGenerator trait
  - Generate clean HTML with proper formatting
  - _Requirements: 11.1, 11.3_

- [ ] 7.4 Implement MarkdownCodeGenerator and JsonCodeGenerator
  - Implement MarkdownCodeGenerator for documentation
  - Implement JsonCodeGenerator for raw export
  - Add proper formatting for each format
  - _Requirements: 11.1, 11.3_

- [ ]* 7.5 Write unit tests for export service
  - Test each generator with sample components
  - Test nested containers
  - Test custom components
  - Test different presets
  - _Requirements: 6.1_

- [ ] 8. Implement component registry service
- [ ] 8.1 Create ComponentRegistry struct
  - Define struct with HashMap for components
  - Add validators HashMap
  - Implement new() with builtin components
  - _Requirements: 9.1, 9.2, 13.1_

- [ ] 8.2 Implement registry operations
  - Implement register() with validation
  - Implement unregister() with error handling
  - Implement get() and list() methods
  - Implement search() with fuzzy matching
  - _Requirements: 9.1, 9.4, 13.2_

- [ ] 8.3 Register builtin components
  - Register Button, Text, Input, Container components
  - Define props_schema for each component
  - Add default values and descriptions
  - _Requirements: 9.1, 13.1_

- [ ]* 8.4 Write unit tests for component registry
  - Test register/unregister operations
  - Test duplicate name detection
  - Test search functionality
  - Test validation integration
  - _Requirements: 6.1_

- [ ] 9. Implement validation service
- [ ] 9.1 Create ValidationService
  - Centralize validation logic
  - Integrate with validators
  - Provide clear error messages
  - _Requirements: 2.5, 8.1_

- [ ] 9.2 Add HTML sanitization
  - Implement sanitize_html() function
  - Remove script tags and event handlers
  - Use regex for pattern matching
  - Add tests for XSS prevention
  - _Requirements: 8.2, 8.4_

## Phase 4: UI Components Refactor

- [ ] 10. Refactor Canvas component
- [ ] 10.1 Create new canvas module structure
  - Create builder/canvas/ directory
  - Split into canvas.rs, drag_drop.rs, renderer.rs
  - Update mod.rs exports
  - _Requirements: 1.1, 1.2_

- [ ] 10.2 Implement new Canvas component with AppState
  - Use AppState context instead of props
  - Implement memo for canvas_state
  - Add proper event handlers (drop, dragover)
  - Use ComponentId for keyed lists
  - _Requirements: 3.1, 3.2, 3.3, 4.2_

- [ ] 10.3 Implement ComponentRenderer
  - Create separate renderer component
  - Handle all component types
  - Add selection highlighting
  - Implement click handlers
  - _Requirements: 1.1, 3.2_

- [ ] 10.4 Refactor drag-drop system
  - Extract drag-drop logic to separate module
  - Add visual feedback improvements
  - Integrate with history system
  - Handle nested container drops
  - _Requirements: 3.2, 14.3_

- [ ] 11. Refactor Sidebar component
- [ ] 11.1 Create new sidebar module structure
  - Create builder/sidebar/ directory
  - Split into sidebar.rs, component_library.rs, theme_selector.rs
  - Update mod.rs exports
  - _Requirements: 1.1, 1.2_

- [ ] 11.2 Implement new Sidebar with AppState
  - Use AppState context
  - Remove prop drilling
  - Integrate with ComponentRegistry
  - _Requirements: 3.1, 3.2_

- [ ] 11.3 Refactor component library UI
  - Use ComponentRegistry for component list
  - Implement search with debouncing
  - Add category filtering
  - Improve drag-drop reordering
  - _Requirements: 13.1, 13.2, 13.3, 4.3_

- [ ] 11.4 Implement theme selector component
  - Extract theme selection to separate component
  - Add color picker for custom theme
  - Integrate with design tokens
  - _Requirements: 1.1, 15.1_

- [ ] 12. Refactor Property Editor component
- [ ] 12.1 Create new property editor module
  - Create builder/property_editor/ directory
  - Split into property_editor.rs, validators.rs
  - Update mod.rs exports
  - _Requirements: 1.1, 1.2_

- [ ] 12.2 Implement new PropertyEditor with AppState
  - Use AppState context for selected component
  - Use memo for selected_component
  - Remove prop drilling
  - _Requirements: 3.1, 3.2_

- [ ] 12.3 Create PropertyForm components
  - Implement ButtonPropertyForm
  - Implement TextPropertyForm
  - Implement InputPropertyForm
  - Implement ContainerPropertyForm
  - Implement CustomPropertyForm
  - _Requirements: 1.1, 1.5_

- [ ] 12.4 Add real-time validation
  - Integrate ValidationService
  - Show validation errors inline
  - Prevent invalid values
  - Add visual feedback
  - _Requirements: 2.5, 7.5, 8.1_

- [ ] 13. Refactor other UI components
- [ ] 13.1 Refactor Preview component
  - Use AppState context
  - Optimize re-renders with memo
  - Add responsive preview modes
  - _Requirements: 3.1, 3.2, 15.1_

- [ ] 13.2 Refactor CommandPalette component
  - Improve search algorithm
  - Add command categories
  - Integrate with keyboard shortcuts
  - _Requirements: 7.2, 10.2_

- [ ] 13.3 Refactor Breadcrumb component
  - Use ComponentId for navigation
  - Add proper ARIA labels
  - Improve visual design
  - _Requirements: 7.3, 7.5_

- [ ] 13.4 Refactor Snackbar component
  - Use notification from UiState
  - Add different notification types (success, error, warning, info)
  - Add auto-dismiss with configurable timeout
  - Improve animations
  - _Requirements: 3.1, 7.5_

- [ ] 14. Update main App component
- [ ] 14.1 Refactor App component with new architecture
  - Initialize AppState with provide_context
  - Remove local state management
  - Simplify event handlers
  - _Requirements: 3.1, 3.2_

- [ ] 14.2 Integrate keyboard shortcuts with new state
  - Update keyboard action handlers
  - Use AppState for all operations
  - Add proper error handling
  - _Requirements: 7.2, 10.2_

- [ ] 14.3 Add global error boundary
  - Catch and display errors gracefully
  - Log errors for debugging
  - Provide recovery options
  - _Requirements: 2.2, 2.5, 7.5_

## Phase 5: Testing & Quality

- [ ] 15. Add comprehensive unit tests
- [ ] 15.1 Test domain models
  - Test component creation and validation
  - Test ComponentId generation and equality
  - Test serialization/deserialization
  - _Requirements: 6.1_

- [ ] 15.2 Test state management
  - Test AppState initialization
  - Test state updates and reactivity
  - Test persistence
  - _Requirements: 6.1_

- [ ] 15.3 Test services
  - Test export service with all generators
  - Test component registry operations
  - Test validation service
  - _Requirements: 6.1_

- [ ]* 15.4 Add property-based tests
  - Use proptest or quickcheck
  - Test validation with random inputs
  - Test serialization round-trips
  - _Requirements: 6.4_

- [ ] 16. Add integration tests
- [ ]* 16.1 Test drag-drop flow
  - Test adding components to canvas
  - Test nested container drops
  - Test undo/redo after drop
  - _Requirements: 6.2_

- [ ]* 16.2 Test edit flow
  - Test selecting component
  - Test editing properties
  - Test validation errors
  - _Requirements: 6.2_

- [ ]* 16.3 Test export flow
  - Test export with different presets
  - Test export with nested components
  - Test export with custom components
  - _Requirements: 6.2_

- [ ]* 16.4 Test keyboard shortcuts
  - Test all keyboard actions
  - Test shortcut conflicts
  - Test input field exclusion
  - _Requirements: 6.2_

- [ ] 17. Performance optimization
- [ ] 17.1 Optimize reactivity
  - Audit signal usage
  - Add memos for expensive computations
  - Reduce unnecessary re-renders
  - _Requirements: 3.2, 3.3, 4.2_

- [ ] 17.2 Optimize rendering
  - Use stable keys in For components
  - Add lazy loading for large lists
  - Debounce expensive operations
  - _Requirements: 3.5, 4.2, 4.4_

- [ ] 17.3 Optimize bundle size
  - Enable LTO and optimize for size
  - Remove unused dependencies
  - Analyze WASM bundle
  - _Requirements: 4.1_

- [ ]* 17.4 Add performance benchmarks
  - Benchmark render time for large layouts
  - Benchmark serialization/deserialization
  - Profile memory usage
  - _Requirements: 4.2_

## Phase 6: Polish & Production Ready

- [ ] 18. Improve accessibility
- [ ] 18.1 Add ARIA labels and roles
  - Add aria-label to all interactive elements
  - Add proper roles (button, navigation, etc)
  - Add aria-pressed, aria-expanded states
  - _Requirements: 7.1, 7.3_

- [ ] 18.2 Improve keyboard navigation
  - Ensure all features accessible via keyboard
  - Add focus indicators
  - Test with screen readers
  - _Requirements: 7.2_

- [ ] 18.3 Improve color contrast
  - Audit all colors for WCAG AA compliance
  - Update design tokens if needed
  - Test with color blindness simulators
  - _Requirements: 7.4_

- [ ] 19. Security hardening
- [ ] 19.1 Add HTML sanitization
  - Implement sanitize_html() in validation service
  - Apply to all custom component templates
  - Add tests for XSS prevention
  - _Requirements: 8.2, 8.4_

- [ ] 19.2 Add Content Security Policy
  - Update index.html with CSP meta tag
  - Test WASM execution with CSP
  - Document CSP requirements
  - _Requirements: 8.3_

- [ ] 19.3 Audit data validation
  - Review all input validation
  - Add validation for LocalStorage data
  - Test with malicious inputs
  - _Requirements: 8.1, 8.5_

- [ ] 20. Documentation
- [ ] 20.1 Add rustdoc comments
  - Document all public APIs
  - Add examples to documentation
  - Document error cases
  - _Requirements: 5.3, 10.3_

- [ ] 20.2 Update README
  - Update architecture section
  - Add migration guide
  - Update development setup
  - _Requirements: 10.5_

- [ ] 20.3 Create user documentation
  - Write getting started guide
  - Document all features
  - Create keyboard shortcuts reference
  - Add troubleshooting guide
  - _Requirements: 10.3_

- [ ]* 20.4 Create video tutorials
  - Record basic usage tutorial
  - Record advanced features tutorial
  - Record custom component tutorial
  - _Requirements: 10.3_

- [ ] 21. Final polish
- [ ] 21.1 Update design tokens
  - Review and update CSS custom properties
  - Ensure consistent spacing and colors
  - Test dark theme
  - _Requirements: 15.2, 15.3_

- [ ] 21.2 Improve error messages
  - Review all error messages
  - Make them more user-friendly
  - Add actionable suggestions
  - _Requirements: 2.2, 2.5, 7.5_

- [ ] 21.3 Add loading states
  - Add loading indicators for async operations
  - Add skeleton screens where appropriate
  - Improve perceived performance
  - _Requirements: 7.5_

- [ ] 21.4 Final testing and bug fixes
  - Test all features end-to-end
  - Fix any remaining bugs
  - Test on different browsers
  - Test on different screen sizes
  - _Requirements: 6.2, 6.5_

- [ ] 22. Release preparation
- [ ] 22.1 Update version and changelog
  - Bump version to 1.0.0
  - Write comprehensive changelog
  - Update all version references
  - _Requirements: 10.5_

- [ ] 22.2 Create migration guide
  - Document breaking changes
  - Provide migration examples
  - Add data migration tool if needed
  - _Requirements: 10.5_

- [ ] 22.3 Setup CI/CD
  - Configure automated tests
  - Setup automated builds
  - Configure deployment pipeline
  - _Requirements: 6.5, 10.4_

- [ ] 22.4 Final release
  - Tag release in Git
  - Publish to crates.io (if applicable)
  - Announce release
  - _Requirements: 10.5_
