# Contributing to Leptos Studio

Thank you for your interest in contributing to Leptos Studio! We welcome contributions from everyone.

## Getting Started

### Development Environment

```bash
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/leptos-studio.git
cd leptos-studio

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown

# Run development server
trunk serve
```

## How to Contribute

### 1. Report Bugs

If you find a bug, please create an issue with:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Browser and Rust version
- Screenshots if applicable

### 2. Suggest Features

For feature requests, create an issue with:
- Clear description of the feature
- Use cases and benefits
- Implementation suggestions (if any)

### 3. Submit Pull Requests

1. **Fork the repository** and create a feature branch
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write clear, readable code
   - Follow existing code style
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   cargo test
   cargo clippy
   trunk build
   ```

4. **Commit your changes**
   ```bash
   git commit -m "feat: add new feature"
   ```
   
   Use conventional commit messages:
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `refactor:` - Code refactoring
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks

5. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request on GitHub with:
   - Clear description of changes
   - Reference to related issues
   - Screenshots for UI changes

## Code Style Guidelines

### Rust Code

- Follow standard Rust formatting (`cargo fmt`)
- Pass Clippy lints (`cargo clippy`)
- Use meaningful variable and function names
- Add doc comments for public APIs

```rust
/// Creates a new canvas component.
///
/// # Arguments
/// * `props` - Canvas configuration properties
///
/// # Returns
/// A Leptos view representing the canvas
#[component]
pub fn Canvas(props: CanvasProps) -> impl IntoView {
    // Implementation
}
```

### Leptos Components

- Use `#[component]` macro for components
- Prefer signals for reactive state
- Keep components focused and single-purpose
- Use `view!` macro for templates

```rust
#[component]
pub fn MyComponent(
    #[prop(into)] label: String,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let count = RwSignal::new(0);
    
    view! {
        <button 
            disabled=disabled
            on:click=move |_| count.set(count.get() + 1)
        >
            {label} " - " {count}
        </button>
    }
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Writing Tests

Add tests for new features:

```rust
#[test]
fn test_component_creation() {
    let component = CanvasComponent::Button {
        label: "Click me".to_string()
    };
    
    match component {
        CanvasComponent::Button { label } => {
            assert_eq!(label, "Click me");
        }
        _ => panic!("Expected Button component"),
    }
}
```

## Leptos-Specific Tools

### Format View Macros

```bash
# Install leptosfmt
cargo install leptosfmt

# Format all files
leptosfmt src/
```

### Leptos Linting

```bash
# Install dylint and leptos-lints
cargo install cargo-dylint dylint-link

# Run leptos-specific lints
cargo dylint --all
```

## Project Areas

### High Priority
- Fix build errors in command_palette.rs and app.rs
- Improve Leptos 0.8 compatibility
- Add more comprehensive tests
- Performance optimizations

### Feature Improvements
- Enhanced component library
- Better error handling and user feedback
- Additional export formats
- Improved responsive design tools

### Documentation
- More code examples
- User tutorials and guides
- API documentation improvements
- Video walkthroughs

## Code Review Process

1. **Automated Checks**: CI runs tests, clippy, and format checks
2. **Manual Review**: Maintainers review code quality and design
3. **Testing**: Changes are tested in development build
4. **Approval**: At least one maintainer approval required
5. **Merge**: Squash and merge to main branch

## Community Guidelines

- Be respectful and constructive
- Help others learn and grow
- Focus on the code, not the person
- Welcome newcomers and answer questions
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Documentation**: Check README.md and code comments

## Recognition

Contributors will be recognized in:
- GitHub contributors list
- Release notes for significant contributions
- Project acknowledgments

Thank you for contributing to Leptos Studio! 🦀
