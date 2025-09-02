# Leptos Studio

**Leptos Studio** adalah visual UI builder drag-and-drop untuk framework Rust Leptos, dengan fitur-fitur professional-grade yang mengikuti standar internasional dari visual builder terkenal seperti Figma, VS Code, dan Webflow. Memungkinkan pengembang untuk membangun antarmuka pengguna secara visual dengan mudah dan mengekspor kode Leptos yang siap digunakan.

## ✨ Professional-Grade Features

### 🎨 **VS Code-style Command Palette**
- Global command palette dengan **Cmd+K** (atau Ctrl+K) shortcut
- Fuzzy search untuk semua aksi dan command
- Keyboard navigation untuk workflow yang efisien

### 🧭 **Figma-style Breadcrumb Navigation** 
- Hierarchical navigation bar yang menampilkan struktur komponen
- Click untuk navigate ke parent/child components
- Visual indicator untuk komponen yang sedang dipilih

### ✨ **Enhanced Drag & Drop System**
- Visual feedback dengan smooth animations
- Drop zones dengan highlight dan preview
- Professional drag indicators dan ghost elements

### ⌨️ **Global Keyboard Shortcuts**
- **Undo**: `Cmd+Z` / `Ctrl+Z`
- **Redo**: `Cmd+Y` / `Ctrl+Y` 
- **Delete**: `Delete` key untuk menghapus komponen
- **Command Palette**: `Cmd+K` / `Ctrl+K`
- **Escape**: Cancel current action

### 🎭 **Design Token System**
- CSS custom properties untuk consistent theming
- Professional color palette dan typography
- Responsive design dengan mobile-first approach

## 🚀 Fitur Utama

### ✅ Fitur yang Tersedia
- ♻️ **Hot reload custom component**: Edit template custom component, preview langsung update tanpa reload
- 🧩 **Component library management**: Tambah/hapus custom component langsung dari sidebar
- 🛡️ **Component props validation**: Form custom component memvalidasi nama (harus identifier Rust valid) & template (harus HTML snippet valid), error message tampil jika input tidak valid
- 🗂️ **Version control (Git) UI**: Sidebar menampilkan status, commit, dan log Git secara langsung
- 📤 **Flexible export & code generation templates**: Export desain ke format Leptos, HTML, Markdown, JSON, dan preset ekspor (Plain, thaw-ui, leptos-material, leptos-use)
### 🔄 Roadmap Fitur

## 🛠 Instalasi dan Penggunaan

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Trunk](https://trunkrs.dev/) untuk WASM build
- Browser modern dengan support WebAssembly

### Menjalankan Aplikasi
```bash
# Clone repository
git clone git@gitlab.com:analisiskebutuhan/leptos-studio.git
cd leptos-studio
## Ekosistem & Best Practice Leptos

Leptos Studio mengikuti praktik terbaik dari ekosistem [Leptos](https://github.com/leptos-rs/leptos):

- **Fine-grained reactivity**: Semua komponen builder dan custom component menggunakan signals & context secara idiomatik.
- **Komponen modular**: Arsitektur berbasis komponen reusable.
- **Ekosistem library**: Mendukung ekspor/preset untuk [thaw-ui](https://github.com/thaw-ui/thaw), [leptos-material](https://github.com/jordi-star/leptos-material), [leptos-use](https://leptos-use.rs/), dsb.
- **Autoformat**: Disarankan menggunakan [leptosfmt](https://github.com/bram209/leptosfmt) untuk format macro `view!` (instruksi di bawah).
- **Linting**: Menggunakan [leptos-lints](https://github.com/leptos-rs/leptos-lints) untuk menjaga kualitas kode.

# Install dependencies (otomatis via Cargo)

### Build untuk Production
```

## 🎯 Cara Penggunaan

### 🖱️ **Basic Workflow**
1. **Drag & Drop**: Drag komponen dari sidebar ke canvas
2. **Edit Properties**: Klik komponen di canvas, edit di property panel
3. **Nested Layout**: Drag komponen ke dalam Container untuk layout bersarang
4. **Export Code**: Klik tombol "Export" untuk generate kode Leptos
5. **Save/Load**: Gunakan tombol Save/Load untuk menyimpan layout

### ⌨️ **Professional Shortcuts**
- **`Cmd+K`**: Buka Command Palette untuk akses cepat semua fitur
- **`Cmd+Z/Y`**: Undo/Redo perubahan
- **`Delete`**: Hapus komponen yang dipilih
- **`Escape`**: Cancel action atau close modal

### 🧭 **Navigation**
- **Breadcrumbs**: Click pada breadcrumb bar untuk navigate ke parent/child
- **Canvas**: Click komponen di canvas untuk select dan edit
- **Sidebar**: Component library dan property editor yang reaktif

## 🏗️ Arsitektur

### 📁 **Project Structure**
```
src/
├── app.rs              # Main application dengan professional layout
├── builder/            # Core builder modules
│   ├── sidebar.rs      # Component palette & property editor
│   ├── canvas.rs       # Interactive canvas dengan drag & drop
│   ├── preview.rs      # Live preview panel
│   ├── export.rs       # Code generation system
│   ├── keyboard.rs     # Global keyboard shortcuts
│   ├── command_palette.rs # VS Code-style command palette  
│   ├── breadcrumb.rs   # Figma-style navigation
│   ├── drag_drop.rs    # Enhanced drag & drop system
│   └── design_tokens.rs # CSS design system
├── components/         # Reusable UI components
└── lib.rs             # Library root
```

### 🎨 **Design System**
- **CSS Custom Properties**: Design tokens untuk consistent styling
- **Flexbox Layout**: Responsive dan adaptive design
- **Component-based Architecture**: Modular dan reusable components
- **Professional Typography**: Optimized font hierarchy dan spacing

## 🤝 Kontribusi

Kontribusi sangat diterima! Lihat [CONTRIBUTING.md](CONTRIBUTING.md) untuk panduan detail.

## 📝 Lisensi

Proyek ini dilisensikan di bawah MIT License - lihat file [LICENSE](LICENSE) untuk detail.

## 📊 Status

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.2.4-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Tests](https://img.shields.io/badge/tests-62%2F62%20passing-brightgreen)

## 🧪 Pengujian & Quality Assurance

### ✅ **Comprehensive Test Coverage**
- **62 tests passing** dengan 100% success rate
- Integration tests untuk semua fitur professional-grade
- Property-based testing untuk edge cases
- Stress testing untuk large layouts dan complex scenarios

### 🔧 **Code Quality**
- Clean compilation dengan minimal warnings
- Professional-grade error handling dan validation  
- Consistent code style dan architecture patterns
- Extensive documentation dan API references

### 🎯 **Testing Scenarios**
- Drag & drop functionality dengan berbagai komponen
- Keyboard shortcuts dan command palette
- Export code generation untuk semua preset (Plain, thaw-ui, leptos-material, leptos-use)
- Custom component management dan validation
- Undo/redo system dengan complex state management
- Unicode, emoji, dan special character handling
- Deeply nested components dan large layout stress tests
