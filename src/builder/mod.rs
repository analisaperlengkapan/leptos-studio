pub mod accessibility;
pub mod breadcrumb;
pub mod breakpoint_editor;
pub mod canvas; // Modular canvas structure (canvas.rs + renderer.rs)
pub mod code_panel;
pub mod command_palette;
pub mod component_constraints;
pub mod component_library;
pub mod component_library_enhanced;
pub mod component_palette;
pub mod design_tokens;
pub mod drag_drop;
pub mod export_modal;
pub mod git_panel;
pub mod history_panel;
pub mod hooks;
pub mod keyboard;
pub mod preview;
pub mod project;
pub mod property_editor;
pub mod property_editors;
pub mod property_inputs;
pub mod responsive_preview;
pub mod sidebar;
pub mod snackbar;
pub mod status_bar;
pub mod styling_system;
pub mod template_gallery;
pub mod toolbar;
pub mod tree_view;

// Re-export components for easier imports
pub use accessibility::{
    Announcer, FocusTrap, KeyboardHint, LiveRegion, ProgressBar, SkipLink, Tooltip, VisuallyHidden,
};
pub use breakpoint_editor::{BreakpointEditor, BreakpointState, ResponsiveWrapper};
pub use component_constraints::{
    AlignmentOption, DesignGuideline, GridSystem, ResponsiveBreakpoint, SizeConstraints,
};
pub use component_library_enhanced::{
    CategoryFilter, ComponentCard, ComponentCategory, ComponentFavorite, LibrarySearchBar,
    get_categories, search_components,
};
pub use component_palette::ComponentPalette;
pub use preview::Preview;
pub use property_editor::PropertyEditor;
pub use responsive_preview::{CanvasViewport, ResponsiveIndicator, ResponsivePreviewControls};
pub use status_bar::{StatusBar, StatusIndicator};
pub use styling_system::{ComponentStyle, StyleEditor, ThemePreset, ThemeSelector};
pub use template_gallery::TemplateGallery;
pub use tree_view::TreeView;
pub mod debug_panel;
