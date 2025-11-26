pub mod breadcrumb;
pub mod canvas; // Modular canvas structure (canvas.rs + renderer.rs)
pub mod command_palette;
pub mod component_constraints;
pub mod component_library;
pub mod component_library_enhanced;
pub mod design_tokens;
pub mod drag_drop;
pub mod git_panel;
pub mod keyboard;
pub mod preview;
pub mod project;
pub mod property_editor;
pub mod responsive_preview;
pub mod sidebar;
pub mod snackbar;
pub mod styling_system;

// Re-export components for easier imports
pub use component_constraints::{
    AlignmentOption, DesignGuideline, GridSystem, ResponsiveBreakpoint, SizeConstraints,
};
pub use component_library_enhanced::{
    CategoryFilter, ComponentCard, ComponentCategory, ComponentFavorite, LibrarySearchBar,
    get_categories, search_components,
};
pub use preview::Preview;
pub use property_editor::PropertyEditor;
pub use responsive_preview::{CanvasViewport, ResponsiveIndicator, ResponsivePreviewControls};
pub use styling_system::{ComponentStyle, StyleEditor, ThemePreset, ThemeSelector};
pub mod debug_panel;
