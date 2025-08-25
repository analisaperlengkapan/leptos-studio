# Leptos Studio

**Leptos Studio** adalah visual UI builder drag-and-drop untuk framework Rust Leptos, terinspirasi FlutterFlow dan React Builder. Memungkinkan pengembang untuk membangun antarmuka pengguna secara visual dengan mudah dan mengekspor kode Leptos yang siap digunakan.

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

2. **Edit Properties**: Klik komponen di canvas, edit di property panel
3. **Nested Layout**: Drag komponen ke dalam Container untuk layout bersarang
5. **Export Code**: Klik tombol "Export" untuk generate kode Leptos
6. **Save/Load**: Gunakan tombol Save/Load untuk menyimpan layout

## 🏗️ Arsitektur

```
src/
├── app.rs              # Main application component
│   ├── sidebar.rs      # Component palette (custom component management)
│   ├── property_editor.rs # Property editing panel (edit + validate custom component)
│   ├── preview.rs      # Live preview (hot reload)
│   └── export.rs       # Code generation (sinkron custom component)
├── components/         # Reusable UI components
└── lib.rs             # Library root
```

> **Note:** Arsitektur custom component kini sepenuhnya berbasis `LibraryComponent` (bukan tuple/string), sehingga semua modul builder sinkron dan mendukung hot reload.

## 🤝 Kontribusi

Kontribusi sangat diterima! Lihat [CONTRIBUTING.md](CONTRIBUTING.md) untuk panduan detail.

## 📝 Lisensi

Proyek ini dilisensikan di bawah MIT License - lihat file [LICENSE](LICENSE) untuk detail.

## 📊 Status


![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.2.3-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 🧪 Pengujian & Robustness

- Lebih dari 50 pengujian integrasi dan property-based: mencakup semua edge case ekspor kode, serialisasi, undo/redo, validasi props, custom component, unicode, emoji, stress test, dan layout besar.
- Semua pengujian lulus, workspace bebas error dan warning (lint/build/test).
- Pengujian edge case: komponen kosong, input tanpa placeholder, label panjang/unicode, deeply nested, kombinasi custom & basic, validasi error handling, dan serialisasi custom component.
- Sistem preset ekspor diuji penuh (Plain, thaw-ui, leptos-material, leptos-use) dan terintegrasi di seluruh pipeline export.
