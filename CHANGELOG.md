# Changelog

All notable changes to Leptos Studio will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.2.0] - 2025-08-25

### Added
- 🧩 Component library management: tambah/hapus custom component langsung dari sidebar
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
