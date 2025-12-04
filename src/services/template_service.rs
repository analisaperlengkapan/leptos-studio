//! Template Service
//!
//! Provides pre-built layout templates and component presets for
//! quick-start designs. Templates can be applied to canvas for
//! rapid UI prototyping.

use crate::domain::{
    ButtonComponent, ButtonSize, ButtonVariant, CanvasComponent, ContainerComponent, FlexDirection,
    InputComponent, InputType, LayoutType, Spacing, TextComponent, TextStyle, TextTag,
};
use serde::{Deserialize, Serialize};

/// Template category for organization
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateCategory {
    /// Landing page layouts
    LandingPage,
    /// Dashboard layouts
    Dashboard,
    /// Form layouts
    Form,
    /// Navigation layouts
    Navigation,
    /// Card layouts
    Card,
    /// Hero sections
    Hero,
    /// Footer layouts
    Footer,
    /// Custom user templates
    Custom,
}

/// A complete layout template with components
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    /// Unique template identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Description of the template
    pub description: String,
    /// Template category
    pub category: TemplateCategory,
    /// Preview thumbnail URL (optional)
    pub thumbnail: Option<String>,
    /// The components that make up this template
    pub components: Vec<CanvasComponent>,
    /// Tags for search/filter
    pub tags: Vec<String>,
}

impl Template {
    /// Create a new template
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        category: TemplateCategory,
        components: Vec<CanvasComponent>,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category,
            thumbnail: None,
            components,
            tags: Vec::new(),
        }
    }

    /// Add tags to the template
    pub fn with_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.into_iter().map(String::from).collect();
        self
    }
}

/// Template service for managing and applying templates
pub struct TemplateService;

impl TemplateService {
    /// Create a new template service
    pub fn new() -> Self {
        Self
    }
    
    /// Get all built-in templates
    pub fn builtin_templates() -> Vec<Template> {
        vec![
            Self::login_form_template(),
            Self::contact_form_template(),
            Self::hero_section_template(),
            Self::pricing_card_template(),
            Self::navigation_bar_template(),
            Self::footer_template(),
            Self::dashboard_header_template(),
            Self::feature_grid_template(),
        ]
    }

    /// Get templates by category
    pub fn templates_by_category(category: TemplateCategory) -> Vec<Template> {
        Self::builtin_templates()
            .into_iter()
            .filter(|t| t.category == category)
            .collect()
    }

    /// Search templates by name or tags
    pub fn search_templates(query: &str) -> Vec<Template> {
        let query_lower = query.to_lowercase();
        Self::builtin_templates()
            .into_iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower)
                    || t.description.to_lowercase().contains(&query_lower)
                    || t.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Login form template
    fn login_form_template() -> Template {
        let mut container = ContainerComponent::new();
        container.layout = LayoutType::Flex {
            direction: FlexDirection::Column,
            wrap: false,
        };
        container.gap = 16;
        container.padding = Spacing {
            top: 32,
            right: 32,
            bottom: 32,
            left: 32,
        };

        // Title
        let mut title = TextComponent::new("Login".to_string());
        title.style = TextStyle::Heading1;
        title.tag = TextTag::H1;

        // Email input
        let mut email_input = InputComponent::new();
        email_input.placeholder = "Email address".to_string();
        email_input.input_type = InputType::Email;
        email_input.required = true;

        // Password input
        let mut password_input = InputComponent::new();
        password_input.placeholder = "Password".to_string();
        password_input.input_type = InputType::Password;
        password_input.required = true;

        // Submit button
        let mut submit_btn = ButtonComponent::new("Sign In".to_string());
        submit_btn.variant = ButtonVariant::Primary;
        submit_btn.size = ButtonSize::Large;

        // Forgot password link
        let forgot_text = TextComponent::new("Forgot your password?".to_string());

        container.children = vec![
            CanvasComponent::Text(title),
            CanvasComponent::Input(email_input),
            CanvasComponent::Input(password_input),
            CanvasComponent::Button(submit_btn),
            CanvasComponent::Text(forgot_text),
        ];

        Template::new(
            "login-form",
            "Login Form",
            "A clean login form with email and password fields",
            TemplateCategory::Form,
            vec![CanvasComponent::Container(container)],
        )
        .with_tags(vec!["login", "auth", "form", "email", "password"])
    }

    /// Contact form template
    fn contact_form_template() -> Template {
        let mut container = ContainerComponent::new();
        container.layout = LayoutType::Flex {
            direction: FlexDirection::Column,
            wrap: false,
        };
        container.gap = 12;
        container.padding = Spacing {
            top: 24,
            right: 24,
            bottom: 24,
            left: 24,
        };

        // Title
        let mut title = TextComponent::new("Contact Us".to_string());
        title.style = TextStyle::Heading2;
        title.tag = TextTag::H2;

        // Name input
        let mut name_input = InputComponent::new();
        name_input.placeholder = "Your name".to_string();
        name_input.required = true;

        // Email input
        let mut email_input = InputComponent::new();
        email_input.placeholder = "Email address".to_string();
        email_input.input_type = InputType::Email;
        email_input.required = true;

        // Phone input
        let mut phone_input = InputComponent::new();
        phone_input.placeholder = "Phone number (optional)".to_string();
        phone_input.input_type = InputType::Tel;

        // Submit button
        let mut submit_btn = ButtonComponent::new("Send Message".to_string());
        submit_btn.variant = ButtonVariant::Primary;

        container.children = vec![
            CanvasComponent::Text(title),
            CanvasComponent::Input(name_input),
            CanvasComponent::Input(email_input),
            CanvasComponent::Input(phone_input),
            CanvasComponent::Button(submit_btn),
        ];

        Template::new(
            "contact-form",
            "Contact Form",
            "A simple contact form with name, email, and phone fields",
            TemplateCategory::Form,
            vec![CanvasComponent::Container(container)],
        )
        .with_tags(vec!["contact", "form", "email", "phone"])
    }

    /// Hero section template
    fn hero_section_template() -> Template {
        let mut container = ContainerComponent::new();
        container.layout = LayoutType::Flex {
            direction: FlexDirection::Column,
            wrap: false,
        };
        container.gap = 24;
        container.padding = Spacing {
            top: 64,
            right: 32,
            bottom: 64,
            left: 32,
        };

        // Main headline
        let mut headline = TextComponent::new("Build Amazing UIs".to_string());
        headline.style = TextStyle::Heading1;
        headline.tag = TextTag::H1;

        // Subtitle
        let mut subtitle = TextComponent::new(
            "Create beautiful, reactive web applications with Leptos Studio".to_string(),
        );
        subtitle.style = TextStyle::Body;
        subtitle.tag = TextTag::P;

        // CTA buttons container
        let mut buttons_container = ContainerComponent::new();
        buttons_container.layout = LayoutType::Flex {
            direction: FlexDirection::Row,
            wrap: false,
        };
        buttons_container.gap = 12;

        // Primary CTA
        let mut primary_btn = ButtonComponent::new("Get Started".to_string());
        primary_btn.variant = ButtonVariant::Primary;
        primary_btn.size = ButtonSize::Large;

        // Secondary CTA
        let mut secondary_btn = ButtonComponent::new("Learn More".to_string());
        secondary_btn.variant = ButtonVariant::Outline;
        secondary_btn.size = ButtonSize::Large;

        buttons_container.children = vec![
            CanvasComponent::Button(primary_btn),
            CanvasComponent::Button(secondary_btn),
        ];

        container.children = vec![
            CanvasComponent::Text(headline),
            CanvasComponent::Text(subtitle),
            CanvasComponent::Container(buttons_container),
        ];

        Template::new(
            "hero-section",
            "Hero Section",
            "A hero section with headline, subtitle, and CTA buttons",
            TemplateCategory::Hero,
            vec![CanvasComponent::Container(container)],
        )
        .with_tags(vec!["hero", "landing", "headline", "cta"])
    }

    /// Pricing card template
    fn pricing_card_template() -> Template {
        let mut card = ContainerComponent::new();
        card.layout = LayoutType::Flex {
            direction: FlexDirection::Column,
            wrap: false,
        };
        card.gap = 16;
        card.padding = Spacing {
            top: 24,
            right: 24,
            bottom: 24,
            left: 24,
        };

        // Plan name
        let mut plan_name = TextComponent::new("Pro Plan".to_string());
        plan_name.style = TextStyle::Heading2;
        plan_name.tag = TextTag::H2;

        // Price
        let mut price = TextComponent::new("$29/month".to_string());
        price.style = TextStyle::Heading1;
        price.tag = TextTag::Span;

        // Feature list
        let feature1 = TextComponent::new("✓ Unlimited projects".to_string());
        let feature2 = TextComponent::new("✓ Priority support".to_string());
        let feature3 = TextComponent::new("✓ Advanced analytics".to_string());
        let feature4 = TextComponent::new("✓ Custom branding".to_string());

        // Subscribe button
        let mut subscribe_btn = ButtonComponent::new("Subscribe".to_string());
        subscribe_btn.variant = ButtonVariant::Primary;
        subscribe_btn.size = ButtonSize::Large;

        card.children = vec![
            CanvasComponent::Text(plan_name),
            CanvasComponent::Text(price),
            CanvasComponent::Text(feature1),
            CanvasComponent::Text(feature2),
            CanvasComponent::Text(feature3),
            CanvasComponent::Text(feature4),
            CanvasComponent::Button(subscribe_btn),
        ];

        Template::new(
            "pricing-card",
            "Pricing Card",
            "A pricing card with plan details and features",
            TemplateCategory::Card,
            vec![CanvasComponent::Container(card)],
        )
        .with_tags(vec!["pricing", "card", "subscription", "features"])
    }

    /// Navigation bar template
    fn navigation_bar_template() -> Template {
        let mut nav = ContainerComponent::new();
        nav.layout = LayoutType::Flex {
            direction: FlexDirection::Row,
            wrap: false,
        };
        nav.gap = 24;
        nav.padding = Spacing {
            top: 16,
            right: 24,
            bottom: 16,
            left: 24,
        };

        // Logo/Brand
        let mut brand = TextComponent::new("Brand".to_string());
        brand.style = TextStyle::Heading2;
        brand.tag = TextTag::Span;

        // Nav links
        let link1 = TextComponent::new("Home".to_string());
        let link2 = TextComponent::new("About".to_string());
        let link3 = TextComponent::new("Services".to_string());
        let link4 = TextComponent::new("Contact".to_string());

        // CTA button
        let mut cta = ButtonComponent::new("Sign Up".to_string());
        cta.variant = ButtonVariant::Primary;

        nav.children = vec![
            CanvasComponent::Text(brand),
            CanvasComponent::Text(link1),
            CanvasComponent::Text(link2),
            CanvasComponent::Text(link3),
            CanvasComponent::Text(link4),
            CanvasComponent::Button(cta),
        ];

        Template::new(
            "navigation-bar",
            "Navigation Bar",
            "A horizontal navigation bar with links and CTA",
            TemplateCategory::Navigation,
            vec![CanvasComponent::Container(nav)],
        )
        .with_tags(vec!["navbar", "navigation", "header", "menu"])
    }

    /// Footer template
    fn footer_template() -> Template {
        let mut footer = ContainerComponent::new();
        footer.layout = LayoutType::Flex {
            direction: FlexDirection::Row,
            wrap: true,
        };
        footer.gap = 32;
        footer.padding = Spacing {
            top: 32,
            right: 32,
            bottom: 32,
            left: 32,
        };

        // Company section
        let mut company_section = ContainerComponent::new();
        company_section.layout = LayoutType::Flex {
            direction: FlexDirection::Column,
            wrap: false,
        };
        company_section.gap = 8;

        let mut company_title = TextComponent::new("Company".to_string());
        company_title.style = TextStyle::Heading3;
        company_section.children = vec![
            CanvasComponent::Text(company_title),
            CanvasComponent::Text(TextComponent::new("About".to_string())),
            CanvasComponent::Text(TextComponent::new("Careers".to_string())),
            CanvasComponent::Text(TextComponent::new("Press".to_string())),
        ];

        // Resources section
        let mut resources_section = ContainerComponent::new();
        resources_section.layout = LayoutType::Flex {
            direction: FlexDirection::Column,
            wrap: false,
        };
        resources_section.gap = 8;

        let mut resources_title = TextComponent::new("Resources".to_string());
        resources_title.style = TextStyle::Heading3;
        resources_section.children = vec![
            CanvasComponent::Text(resources_title),
            CanvasComponent::Text(TextComponent::new("Documentation".to_string())),
            CanvasComponent::Text(TextComponent::new("Blog".to_string())),
            CanvasComponent::Text(TextComponent::new("Support".to_string())),
        ];

        // Copyright
        let mut copyright = TextComponent::new("© 2024 Your Company. All rights reserved.".to_string());
        copyright.style = TextStyle::Caption;

        footer.children = vec![
            CanvasComponent::Container(company_section),
            CanvasComponent::Container(resources_section),
            CanvasComponent::Text(copyright),
        ];

        Template::new(
            "footer",
            "Footer",
            "A multi-column footer with links and copyright",
            TemplateCategory::Footer,
            vec![CanvasComponent::Container(footer)],
        )
        .with_tags(vec!["footer", "links", "copyright"])
    }

    /// Dashboard header template
    fn dashboard_header_template() -> Template {
        let mut header = ContainerComponent::new();
        header.layout = LayoutType::Flex {
            direction: FlexDirection::Row,
            wrap: false,
        };
        header.gap = 16;
        header.padding = Spacing {
            top: 16,
            right: 24,
            bottom: 16,
            left: 24,
        };

        // Title
        let mut title = TextComponent::new("Dashboard".to_string());
        title.style = TextStyle::Heading1;
        title.tag = TextTag::H1;

        // Search input
        let mut search = InputComponent::new();
        search.placeholder = "Search...".to_string();

        // Action buttons
        let mut add_btn = ButtonComponent::new("+ Add New".to_string());
        add_btn.variant = ButtonVariant::Primary;

        let mut settings_btn = ButtonComponent::new("⚙ Settings".to_string());
        settings_btn.variant = ButtonVariant::Ghost;

        header.children = vec![
            CanvasComponent::Text(title),
            CanvasComponent::Input(search),
            CanvasComponent::Button(add_btn),
            CanvasComponent::Button(settings_btn),
        ];

        Template::new(
            "dashboard-header",
            "Dashboard Header",
            "A dashboard header with search and action buttons",
            TemplateCategory::Dashboard,
            vec![CanvasComponent::Container(header)],
        )
        .with_tags(vec!["dashboard", "header", "search", "actions"])
    }

    /// Feature grid template
    fn feature_grid_template() -> Template {
        let mut grid = ContainerComponent::new();
        grid.layout = LayoutType::Grid {
            columns: 3,
            rows: 2,
        };
        grid.gap = 24;
        grid.padding = Spacing {
            top: 32,
            right: 32,
            bottom: 32,
            left: 32,
        };

        // Create feature cards
        let features = vec![
            ("🚀", "Fast Performance", "Lightning-fast rendering"),
            ("🔒", "Secure", "Built-in security features"),
            ("📱", "Responsive", "Works on all devices"),
            ("⚡", "Real-time", "Live updates and sync"),
            ("🎨", "Customizable", "Easy to customize"),
            ("📊", "Analytics", "Built-in analytics"),
        ];

        for (icon, title, desc) in features {
            let mut card = ContainerComponent::new();
            card.layout = LayoutType::Flex {
                direction: FlexDirection::Column,
                wrap: false,
            };
            card.gap = 8;
            card.padding = Spacing {
                top: 16,
                right: 16,
                bottom: 16,
                left: 16,
            };

            let mut icon_text = TextComponent::new(icon.to_string());
            icon_text.style = TextStyle::Heading1;

            let mut title_text = TextComponent::new(title.to_string());
            title_text.style = TextStyle::Heading3;
            title_text.tag = TextTag::H3;

            let mut desc_text = TextComponent::new(desc.to_string());
            desc_text.style = TextStyle::Body;

            card.children = vec![
                CanvasComponent::Text(icon_text),
                CanvasComponent::Text(title_text),
                CanvasComponent::Text(desc_text),
            ];

            grid.children.push(CanvasComponent::Container(card));
        }

        Template::new(
            "feature-grid",
            "Feature Grid",
            "A 3x2 grid of feature cards with icons",
            TemplateCategory::LandingPage,
            vec![CanvasComponent::Container(grid)],
        )
        .with_tags(vec!["features", "grid", "cards", "landing"])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_templates() {
        let templates = TemplateService::builtin_templates();
        assert!(!templates.is_empty());
        assert!(templates.len() >= 8);
    }

    #[test]
    fn test_search_templates() {
        let results = TemplateService::search_templates("login");
        assert!(!results.is_empty());
        assert!(results.iter().any(|t| t.id == "login-form"));
    }

    #[test]
    fn test_templates_by_category() {
        let form_templates = TemplateService::templates_by_category(TemplateCategory::Form);
        assert!(!form_templates.is_empty());
        assert!(form_templates.iter().all(|t| t.category == TemplateCategory::Form));
    }

    #[test]
    fn test_template_components() {
        let templates = TemplateService::builtin_templates();
        for template in templates {
            assert!(!template.components.is_empty(), "Template {} has no components", template.id);
        }
    }
}
