# Leptos Studio

**Leptos Studio** adalah visual UI builder drag-and-drop untuk framework Rust Leptos, terinspirasi FlutterFlow dan React Builder. Memungkinkan pengembang untuk membangun antarmuka pengguna secara visual dengan mudah dan mengekspor kode Leptos yang siap digunakan.

## 🚀 Fitur Utama


### ✅ Fitur yang Tersedia
- ♻️ **Hot reload custom component**: Edit template custom component, preview langsung update tanpa reload
- 🧩 **Component library management**: Tambah/hapus custom component langsung dari sidebar
- 🛡️ **Component props validation**: Form custom component memvalidasi nama (harus identifier Rust valid) & template (harus HTML snippet valid), error message tampil jika input tidak valid
- 🗂️ **Version control (Git) UI**: Sidebar menampilkan status, commit, dan log Git secara langsung
- 📤 **Flexible export**: Export desain ke format Leptos, HTML, Markdown, dan JSON
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

# Install dependencies (otomatis via Cargo)
# Jalankan development server
trunk serve

# Akses aplikasi di http://localhost:8080/
```

### Build untuk Production
```bash
trunk build --release
```

## 🎯 Cara Penggunaan

1. **Drag Components**: Seret komponen dari sidebar ke canvas
2. **Edit Properties**: Klik komponen di canvas, edit di property panel
3. **Nested Layout**: Drag komponen ke dalam Container untuk layout bersarang
4. **Custom Components**: Buat komponen kustom via form di sidebar
5. **Export Code**: Klik tombol "Export" untuk generate kode Leptos
6. **Save/Load**: Gunakan tombol Save/Load untuk menyimpan layout


## 🏗️ Arsitektur

```
src/
├── app.rs              # Main application component
├── builder/            # Core builder modules
│   ├── canvas.rs       # Canvas dengan drag-and-drop (hot reload custom component)
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
