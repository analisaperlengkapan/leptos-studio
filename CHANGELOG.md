# Leptos Studio Changelog

## [0.2.4] - 2025-09-03

### Added - Professional-Grade Features 🚀
- 🎨 **VS Code-style Command Palette**: Global command palette dengan Cmd+K shortcut dan fuzzy search untuk semua aksi
- 🧭 **Figma-style Breadcrumb Navigation**: Hierarchical navigation bar yang menampilkan struktur komponen saat ini
- ✨ **Enhanced Drag & Drop System**: Visual feedback, drop zones, dan smooth animations untuk pengalaman yang lebih profesional
- ⌨️ **Global Keyboard Shortcuts**: Undo (Cmd+Z), Redo (Cmd+Y), Delete, Escape untuk workflow yang efisien
- 🎭 **Design Token System**: CSS custom properties untuk consistent theming dan design system
- 📐 **Professional Layout Structure**: Flexbox-based responsive layout dengan clean CSS architecture

### Changed - Code Quality & Architecture 🔧
- 🏗️ **Complete UI Architecture Refactor**: Modular component system dengan keyboard.rs, command_palette.rs, breadcrumb.rs, drag_drop.rs
- 🎨 **CSS Design System Overhaul**: Professional styling dengan design tokens, consistent spacing, dan responsive behavior
- 📱 **Responsive Design Implementation**: Mobile-first approach dengan adaptive layouts
- 🧹 **Code Quality Improvements**: Fixed unused variables, optimized pattern matching, clean compilation

### Fixed - Stability & Testing 🛠️
- ✅ **Test Configuration**: Fixed Cargo.toml crate-type untuk enable library access dari integration tests
- 🧪 **All Tests Passing**: 62/62 tests passing dengan comprehensive coverage
- 🔧 **Compilation Cleanup**: Resolved pattern matching errors, type mismatches, dan unused variable warnings
- 📦 **Build Optimization**: Clean build pipeline dengan minimal warnings

### Technical Details
- Leptos 0.6 dengan WASM compilation via Trunk
- Professional-grade keyboard navigation dan shortcuts
- International standards compliance following Figma, VS Code, dan Webflow best practices
- Complete test coverage untuk semua fitur baru

## [0.2.3] - 2025-08-25

### Added
- 🎉 **Code generation templates (export presets)**: Export kode Leptos kini mendukung preset: Plain, thaw-ui, leptos-material, leptos-use. UI export preset di sidebar, backend export sinkron.
- 📤 **Flexible export system**: Semua fungsi export (Leptos, HTML, Markdown) kini menerima preset/opsi ekspor.
- 🧪 **Test coverage update**: Semua pengujian integrasi diperbarui untuk preset baru, 100% lulus.

### Changed
- Refactor seluruh pemanggilan export agar konsisten dengan argumen preset baru.
- Sidebar: dropdown preset export, clipboard copy fix (web_sys).

### Fixed
- Semua error dan warning build/test/lint di workspace dioptimalkan (signature, import, clipboard API, dsb).
- Pengujian property-based dan edge case untuk preset baru.

#
## [0.2.1] - 2025-08-25


- 🧪 **Extensive integration & property-based tests**: 50+ pengujian untuk semua edge case ekspor kode, serialisasi, undo/redo, validasi props, custom component, unicode, emoji, stress test, dan layout besar
- 🔍 **Edge case & stress test**: Pengujian untuk komponen kosong, input tanpa placeholder, label panjang/unicode, deeply nested, kombinasi custom & basic, dan validasi error handling

### Added
- Coverage pengujian kini hampir 100% untuk semua logic utama builder, export, dan custom component
- README diperbarui untuk menyoroti robustness dan cakupan pengujian

- ♻️ Hot reload for custom components: perubahan template langsung tampil di canvas & preview tanpa reload
- Semua warning lint pada pengujian dioptimalkan (unused import, dsb)
- Tidak ada error atau warning build pada workspace


### Changed
- Sidebar, Canvas, Preview, Export, PropertyEditor: migrasi penuh ke sistem custom_components baru (berbasis LibraryComponent)

### Fixed
- Build error akibat perbedaan tipe custom_components di seluruh modul
- Bug pada edit, hapus, dan validasi custom component

# Changelog

All notable changes to Leptos Studio will be documented in this file.

## [0.2.4] - 2025-09-03

### Added
- 🚀 **Production-ready UI**: Removed debug styling, implemented clean professional interface
- ⚡ **Performance optimizations**: Reduced render overhead, optimized signal usage
- 🎨 **Enhanced styling**: Modern button design system, improved responsive layout
- 📦 **Build optimizations**: Updated Cargo.toml for better WASM performance
- 🧪 **Test configuration fixed**: All 62+ tests now execute successfully

### Changed
- UI now uses professional styling instead of debug borders and colors
- Performance monitoring only runs in debug builds
- Optimized dependencies and build profiles
- Enhanced loading experience with spinner and better messaging
- Fixed test imports and eliminated duplicate definitions

### Fixed
- ✅ Resolved all compilation errors in test files
- ✅ Fixed duplicate import and function definition issues
- ✅ Cleaned up redundant local imports in integration tests
- ✅ All tests now run successfully: **62 tests passing**
- Removed all debug console.log statements
- Fixed theme color type compatibility issues
- Improved build configuration for production use

### Technical
- Updated Cargo.toml with proper metadata and optimization flags
- Enhanced CSS with modern design system
- Improved HTML with better SEO meta tags and accessibility

---

## [0.2.0] - 2025-08-25

### Added
- 🧩 Component library management: tambah/hapus custom component langsung dari sidebar
- 🛡️ Advanced component props validation: custom component form now checks for valid Rust identifier names and valid HTML template, with user-friendly error messages
- 🎨 Theme switcher: sidebar & canvas reaktif terhadap perubahan tema (Light/Dark/Custom)
- 📱 Responsive design preview: canvas bisa diubah ke mode Desktop/Tablet/Mobile, label mode tampil di canvas
- 🧹 Sidebar/canvas refactor: props kini digunakan penuh, warning hilang, kode lebih bersih
- 💡 UI improvements: tombol, label, dan feedback lebih jelas

### Changed
- Sidebar dan canvas kini lebih modular dan maintainable
- README dan copilot-instructions.md diperbarui sesuai progres

### Fixed
- Semua warning build terkait unused props/handler dihilangkan

### Technical
- Patch Leptos component macro usage agar sesuai best practice
- Responsive state management untuk theme & mode preview

---

## [0.1.0] - 2025-08-22

### Added
- 🚀 Initial release of Leptos Studio
- ✨ Drag-and-drop interface untuk komponen UI (Button, Text, Input, Container)
- 🎨 Real-time property editor untuk komponen di canvas
- 👀 Live preview hasil desain
- 📦 Dukungan nested containers (komponen di dalam container)
- 🛠️ Custom component system (buat komponen kustom)
- 💾 Layout persistence menggunakan localStorage
- 🔄 Code export/generation ke format Leptos
- 📚 Dokumentasi lengkap (README, LICENSE, CONTRIBUTING)
- 🏗️ Arsitektur modular dengan separation of concerns

### Technical
- Built with Rust + Leptos framework
- WASM compilation via Trunk
- Reactive state management dengan signals
- Serialization support untuk layout persistence
- Drag-and-drop API menggunakan web-sys

### Infrastructure
- GitLab repository setup
- Automated build pipeline
- Development environment configuration
