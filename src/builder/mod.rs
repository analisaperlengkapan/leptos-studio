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
pub use preview::Preview;
pub use property_editor::PropertyEditor;
pub use responsive_preview::{ResponsivePreviewControls, CanvasViewport, ResponsiveIndicator};
pub use component_library_enhanced::{
    ComponentCategory, search_components, get_categories, 
    ComponentCard, LibrarySearchBar, CategoryFilter, ComponentFavorite
};
pub use styling_system::{
    ComponentStyle, ThemePreset, StyleEditor, ThemeSelector
};
pub use component_constraints::{
    SizeConstraints, AlignmentOption, DesignGuideline, 
    ResponsiveBreakpoint, GridSystem
};
pub mod debug_panel;
