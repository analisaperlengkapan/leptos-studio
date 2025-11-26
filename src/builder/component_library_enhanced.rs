use leptos::prelude::*;
use crate::builder::component_library::LibraryComponent;

/// Component category for organization
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentCategory {
    Basic,
    Input,
    Container,
    Custom,
    All,
}

impl ComponentCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentCategory::Basic => "Basic",
            ComponentCategory::Input => "Input",
            ComponentCategory::Container => "Container",
            ComponentCategory::Custom => "Custom",
            ComponentCategory::All => "All",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Input" => ComponentCategory::Input,
            "Container" => ComponentCategory::Container,
            "Custom" => ComponentCategory::Custom,
            "All" => ComponentCategory::All,
            _ => ComponentCategory::Basic,
        }
    }
}

/// Enhanced library search and filter
pub fn search_components(
    components: &[LibraryComponent],
    query: &str,
    category: &ComponentCategory,
) -> Vec<LibraryComponent> {
    let query_lower = query.to_lowercase();

    components
        .iter()
        .filter(|comp| {
            // Category filter
            let category_match = match category {
                ComponentCategory::All => true,
                _ => comp.category == category.as_str(),
            };

            // Search query filter (name, category, kind)
            let search_match = query.is_empty()
                || comp.name.to_lowercase().contains(&query_lower)
                || comp.kind.to_lowercase().contains(&query_lower)
                || comp.category.to_lowercase().contains(&query_lower);

            category_match && search_match
        })
        .cloned()
        .collect()
}

/// Get all categories from components
pub fn get_categories(components: &[LibraryComponent]) -> Vec<ComponentCategory> {
    let mut categories = vec![ComponentCategory::All];

    for comp in components {
        let cat = ComponentCategory::from_str(&comp.category);
        if !categories.contains(&cat) {
            categories.push(cat);
        }
    }

    categories
}

/// Component card display
#[component]
pub fn ComponentCard(
    #[prop(into)] component: LibraryComponent,
    #[prop(optional)] _on_drag_start: Option<Box<dyn Fn()>>,
) -> impl IntoView {
    let icon = match component.kind.as_str() {
        "Button" => "🔘",
        "Text" => "📝",
        "Input" => "⌨️",
        "Container" => "📦",
        "Custom" => "⚙️",
        _ => "📌",
    };

    view! {
        <div class="component-card">
            <div class="component-card-icon">{icon}</div>
            <div class="component-card-content">
                <h4 class="component-card-title">{component.name.clone()}</h4>
                <p class="component-card-category">{component.category.clone()}</p>
            </div>
            <div class="component-card-badge">{component.kind.clone()}</div>
        </div>
    }
}

/// Enhanced search bar for library
#[component]
pub fn LibrarySearchBar(
    #[prop(into)] on_search: Callback<String>,
) -> impl IntoView {
    let search_input = RwSignal::new(String::new());

    let handle_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        search_input.set(value.clone());
        on_search.run(value);
    };

    view! {
        <div class="library-search">
            <input
                type="text"
                placeholder="Search components..."
                class="library-search-input"
                on:input=handle_input
                prop:value=search_input
            />
            <span class="library-search-icon">"🔍"</span>
        </div>
    }
}

/// Category filter buttons
#[component]
pub fn CategoryFilter(
    #[prop(into)] categories: Vec<ComponentCategory>,
    #[prop(into)] active_category: RwSignal<ComponentCategory>,
) -> impl IntoView {
    view! {
        <div class="category-filter">
            {categories
                .into_iter()
                .map(|cat| {
                    let cat_clone = cat.clone();
                    let cat_for_label = cat.clone();
                    let is_active = move || active_category.get() == cat_clone;
                    view! {
                        <button
                            class={move || {
                                format!(
                                    "category-btn {}",
                                    if is_active() { "active" } else { "" }
                                )
                            }}
                            on:click=move |_| active_category.set(cat.clone())
                        >
                            {cat_for_label.as_str()}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Favorites indicator (for future implementation)
#[component]
pub fn ComponentFavorite(
    #[prop(into)] is_favorite: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <button
            class={move || {
                format!(
                    "favorite-btn {}",
                    if is_favorite.get() { "active" } else { "" }
                )
            }}
            on:click=move |_| {
                is_favorite.update(|v| *v = !*v);
            }
        >
            {move || if is_favorite.get() { "⭐" } else { "☆" }}
        </button>
    }
}
