# Leptos Studio – Quick Start Guide

Get started building beautiful UIs with Leptos Studio in minutes!

## What is Leptos Studio?

Leptos Studio is a visual UI builder for the [Leptos](https://leptos.dev) framework (Rust + WebAssembly). It enables rapid UI development with:

- **Drag & drop** component building
- **Live preview** of your designs
- **Code export** to Leptos, HTML, Markdown, and JSON formats
- **Responsive design** preview for multiple devices
- **Custom components** with HTML templates
- **Undo/redo** with full history
- **Project management** with save/load capabilities

## Installation & Setup

### 1. Prerequisites

Ensure you have:
- Rust toolchain (stable): https://rustup.rs/
- WASM target: `rustup target add wasm32-unknown-unknown`
- Trunk (WASM bundler): `cargo install trunk`

### 2. Clone Repository

```bash
git clone https://github.com/analisaperlengkapan/leptos-studio.git
cd leptos-studio
```

### 3. Start Development Server

```bash
trunk serve
```

The application will be available at `http://localhost:8899`

## Basic Usage

### 1. Add Components

1. Open the **Sidebar** on the left
2. Browse available components:
   - **Basic**: Button, Text
   - **Input**: Text input, textarea
   - **Container**: Layouts, flex containers
   - **Custom**: Your custom components
3. **Drag** a component onto the canvas

### 2. Edit Component Properties

1. **Select** a component on the canvas (click it)
2. Open the **Property Editor** on the right
3. Modify properties like:
   - Text content
   - Button variant and size
   - Colors and styling
   - Spacing and layout

### 3. Create Custom Components

1. Click **"Add Custom Component"** in the sidebar
2. Enter a unique component **name**
3. Provide an **HTML template**:
   ```html
   <div class="my-card">
     <h3>{{title}}</h3>
     <p>{{description}}</p>
   </div>
   ```
4. Click **"Add Component"**
5. Your custom component is ready to use!

### 4. Preview on Different Devices

Use the **Responsive Preview Controls** at the top:

- **📱 Mobile** - 375px × 667px smartphone view
- **📱 Tablet** - 768px × 1024px tablet view  
- **🖥️ Desktop** - 1920px × 1080px desktop view

### 5. Export Code

1. Click the **"Export"** button in the toolbar
2. Select export format:
   - **Leptos Component** - Ready-to-use Leptos code
   - **HTML** - Static HTML output
   - **Markdown** - Documentation
   - **JSON** - Raw component structure
3. **Copy** to clipboard or **Download** as file

### 6. Save & Load Projects

**Save**: Use Ctrl+S (Cmd+S on Mac) - saves to browser localStorage

**Load**: Click "Load" button to restore previously saved layouts

**Export Project**: In the project panel, "Download JSON" to save locally

**Import Project**: Click "Import JSON" to restore from file

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+S (Cmd+S) | Save layout |
| Ctrl+Z (Cmd+Z) | Undo |
| Ctrl+Y (Cmd+Shift+Z) | Redo |
| Del | Delete selected component |
| Ctrl+C (Cmd+C) | Copy component |
| Ctrl+V (Cmd+V) | Paste component |
| Ctrl+D (Cmd+D) | Duplicate component |
| Ctrl+K (Cmd+K) | Open command palette |
| Ctrl+E (Cmd+E) | Export |
| Escape | Deselect component |

## Advanced Features

### Search Components

1. Look for the **search bar** in the sidebar
2. Type component name to filter
3. Results update in real-time

### Filter by Category

- Click category buttons to filter:
  - **Basic** - Core components
  - **Input** - Form controls
  - **Container** - Layout components
  - **Custom** - User-defined components

### Apply Themes

1. In the Property Editor, find **"Styling"** section
2. Choose a theme preset:
   - **Light** - Professional light theme
   - **Dark** - Dark mode
   - **High Contrast** - Accessibility focused
   - **Colorful** - Vibrant colors
   - **Minimal** - Subtle styling

### Customize Styles

1. Select a component
2. In Property Editor, modify:
   - **Background Color** - Color picker
   - **Padding** - E.g., "8px 16px"
   - **Border Radius** - Rounded corners (0-100px)

## Project Structure

### Main Areas

```
┌─────────────────────────────────────────────┐
│           Header (Title & Buttons)          │
├──────────┬───────────────┬──────────────────┤
│          │               │                  │
│ Sidebar  │    Canvas     │  Property Editor │
│          │               │                  │
│(Lib +    │ (Main Editor) │  (Properties +   │
│ Custom)  │               │   Preview)       │
│          │               │                  │
└──────────┴───────────────┴──────────────────┘
```

### Navigation

- **Sidebar (Left)** - Component library and custom components
- **Canvas (Center)** - Your design area
- **Property Editor (Right)** - Edit selected component properties
- **Top Bar** - Controls and responsive preview options

## Common Tasks

### Task: Create a Login Form

1. Drag a **Container** onto the canvas
2. Drag a **Text** component (for title)
   - Set text to "Login"
   - Increase font size to 24px
3. Drag an **Input** component
   - Set placeholder to "Email"
4. Drag another **Input** component
   - Set placeholder to "Password"
   - Change input type to password
5. Drag a **Button** component
   - Set text to "Sign In"
   - Choose Primary variant
6. Arrange components vertically
7. Export as Leptos code

### Task: Create a Component Theme

1. Select a component
2. Click "Styling" in Property Editor
3. Choose **Theme Preset** (e.g., "Dark")
4. Customize individual properties as needed
5. Export to see generated CSS

### Task: Prepare Mobile Design

1. Click **📱 Mobile** in viewport controls
2. Design your UI for 375px width
3. Click **🖥️ Desktop** to see full layout
4. Adjust responsively as needed

## Troubleshooting

### Components Not Showing

1. Check browser console for errors
2. Ensure component is not hidden (opacity, display)
3. Try dragging component again

### Export Not Working

1. Check browser console for errors
2. Ensure components are valid
3. Try refreshing page and retry

### Layout Lost After Refresh

1. Layouts are saved to **localStorage** automatically
2. Click "Load" to restore
3. Export project to file for backup: "Download JSON"

### Can't Add Custom Component

1. Verify component **name** contains only letters, numbers, underscore
2. Check for **duplicate names** - each must be unique
3. Ensure HTML template is **valid HTML**

## Tips & Best Practices

1. **Organize with Containers** - Use containers to group related components
2. **Use Theme Presets** - Start with a theme, then customize
3. **Export Often** - Save your work to files regularly
4. **Test Responsively** - Check mobile, tablet, and desktop views
5. **Name Components Clearly** - Use descriptive names for custom components
6. **Plan Layout** - Sketch your design before building

## Learning Resources

- **[FEATURES.md](./FEATURES.md)** - Advanced features documentation
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Technical architecture
- **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Developer guide
- **[Leptos Docs](https://leptos.dev)** - Leptos framework documentation

## Getting Help

### Documentation

- 📖 **[README.md](./README.md)** - Project overview
- 📚 **[FEATURES.md](./FEATURES.md)** - Advanced features guide
- 🏗️ **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Technical details
- 👨‍💻 **[DEVELOPMENT.md](./DEVELOPMENT.md)** - Contributing guide

### Community

- 🐙 GitHub Issues - Report bugs or request features
- 💬 GitHub Discussions - Ask questions and discuss

## Next Steps

1. **Explore** the UI and try dragging components
2. **Create** a simple login form (see Common Tasks above)
3. **Export** your design as Leptos code
4. **Read** FEATURES.md to learn about advanced capabilities
5. **Join** the community and contribute!

---

**Happy building! 🚀**

For more information, visit the [full documentation](./README.md).
