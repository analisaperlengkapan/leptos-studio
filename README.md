# Leptos Studio

Leptos Studio adalah aplikasi visual UI builder drag-and-drop untuk framework Rust Leptos, terinspirasi FlutterFlow dan React Builder.

## Fitur Utama
- Sidebar komponen (Button, Text, Input, Container, dsb)
- Canvas drag-and-drop untuk membangun layout
- Editor properti untuk tiap komponen
- Preview hasil desain
- Generator kode Rust/Leptos dari desain visual

## Struktur Awal
- src/
  - main.rs
  - app.rs
  - components/
    - mod.rs
    - button.rs
    - text.rs
    - input.rs
    - container.rs
  - builder/
    - mod.rs
    - canvas.rs
    - sidebar.rs
    - property_editor.rs
    - codegen.rs

## Cara Menjalankan
1. Pastikan sudah menginstall Rust dan toolchain WASM.
2. Jalankan perintah build dan serve sesuai petunjuk di README ini.

---

> Catatan: Ini adalah struktur awal. Fitur drag-and-drop dan generator kode akan diimplementasikan bertahap.
