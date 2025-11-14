# Leptos Studio

Leptos Studio adalah visual UI builder untuk framework [Leptos](https://github.com/leptos-rs/leptos) (Rust + WASM). Aplikasi ini memungkinkan Anda menyusun layout dengan drag & drop, mengelola komponen, dan mengekspor kode ke beberapa format.

## Fitur Utama

- **Canvas drag & drop**  
  Susun layout dengan komponen Button, Text, Input, Container, dan Custom.

- **Custom Components**  
  Tambah komponen custom berbasis HTML template dengan validasi nama dan template.

- **Command Palette**  
  Pencarian perintah cepat (VS Code style) dengan fuzzy search, tersinkron dengan design tokens & CSS variables.

- **Project Management**  
  Panel Project untuk:
  - `New` project (reset layout & history).
  - `Copy JSON` / `Download JSON` project (layout + settings).
  - `Import JSON` untuk memulihkan project.

- **Export Code**  
  Ekspor layout menjadi:
  - Kode Leptos (`LeptosCodeGenerator`)
  - HTML
  - Markdown (dokumentasi)
  - JSON (struktur CanvasComponent)

- **Debug Panel**  
  Menampilkan jumlah komponen, custom components, kemampuan undo/redo, dan metrik render (`render_count`, `render_time`).

- **Git Panel (stub)**  
  Panel Git menggunakan abstraksi `GitBackend` dengan implementasi default `NoopGitBackend` yang aman di browser-only mode. Panel sudah menyediakan tombol `Status`, `Log`, `Commit`, dan `Push` (belum terhubung ke backend nyata).

## Menjalankan Proyek

### Prasyarat

- Rust toolchain (stable)
- `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
```

- [Trunk](https://trunkrs.dev/) untuk dev server WASM:

```bash
cargo install trunk
```

### Development Server

Di direktori proyek:

```bash
trunk serve
```

Secara default Trunk akan melayani aplikasi di `http://localhost:8899` (lihat `Trunk.toml`).

## Testing

### Unit & Integration tests

Jalankan semua test native:

```bash
cargo test
```

### WASM tests

Proyek menyertakan test WASM (mis. `tests/wasm_smoke.rs`) yang memverifikasi layanan export di lingkungan `wasm32`:

```bash
rustup target add wasm32-unknown-unknown
cargo test --target wasm32-unknown-unknown
```

CI (disarankan) sebaiknya menjalankan kedua perintah di atas.

## Arsitektur Singkat

Struktur utama crate:

- `src/domain/`  
  Model domain: `CanvasComponent` dan variannya (`ButtonComponent`, `TextComponent`, `InputComponent`, `ContainerComponent`, `CustomComponent`), error (`AppError`, `ValidationError`), dan validator.

- `src/state/`  
  Global `AppState` (canvas, UI, settings, project), `CanvasState`, `UiState`, `SettingsState`, history undo/redo, dan persistence (LocalStorage).  
  `Project` merepresentasikan layout + settings yang bisa diekspor ke JSON.

- `src/services/`  
  - `export_service`: generator kode (Leptos/HTML/JSON/Markdown) + unit tests.  
  - `project_service`: serialize/deserialize `Project` ke/dari JSON.  
  - `git_service`: trait `GitBackend` + `NoopGitBackend` untuk Git.

- `src/builder/`  
  Komponen UI utama: `Canvas`, `Sidebar`, `PropertyEditor`, `Preview`, `CommandPalette`, `DebugPanel`, `GitPanel`, `ProjectPanel`, drag & drop tools (`DragState`, `DropZone`), dll.

- `src/utils/`  
  utilitas umum, mis. clipboard.

- `style.css`  
  Design tokens dan styling global, termasuk semantic CSS variables untuk Command Palette dan styling Canvas/Sidebar.

## Catatan Pengembangan

- **ComponentRegistry**  
  Helper kecil di `builder::component_library` untuk membantu operasi atas `LibraryComponent` (misalnya cek nama duplikat dan menurunkan custom components dari `component_library`).

- **Undo/Redo**  
  Canvas menyimpan `History<Snapshot>` untuk mendukung undo/redo layout.

- **Error Handling & Notifications**  
  Semua operasi penting menggunakan `AppError::user_message()` untuk menampilkan pesan yang ramah pengguna melalui `Notification` dan `Snackbar`.

## Ide Lanjutan

- Implementasi nyata `GitBackend` (HTTP backend atau Tauri) untuk menghubungkan GitPanel dengan repository.
- Memperluas `ComponentRegistry` dan schema props agar Property Editor bisa lebih generik.
- Dokumentasi arsitektur lebih rinci (`docs/` atau `ARCHITECTURE.md`) bila proyek berkembang lebih jauh.
