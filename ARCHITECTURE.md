# Leptos Studio – Architecture Overview

Dokumen ini merangkum arsitektur **aktual** `leptos-studio` pada level tinggi.
Fokusnya pada pemisahan layer dan beberapa alur data utama yang sering disentuh saat pengembangan.

---

## 1. Layers & Modules

Leptos Studio dibangun dengan pemisahan tanggung jawab yang cukup jelas:

- **Domain (`src/domain`)**  
  Model dan aturan murni aplikasi, tanpa bergantung pada Leptos atau browser.

- **State (`src/state`)**  
  State aplikasi berbasis Leptos `RwSignal`, termasuk UI state, project state, dan history.

- **Services (`src/services`)**  
  Fungsi-fungsi "use case" yang mengoperasikan domain/state: export, project, git, dan property update.

- **Builder / UI (`src/builder`)**  
  Komponen-komponen Leptos yang merangkai state + services menjadi editor visual (canvas, sidebar, property editor, panel-panel).

- **Utils (`src/utils`)**  
  Fungsi pembantu generik seperti clipboard dan formatting.

- **Styling (`style.css`, `src/builder/design_tokens.rs`)**  
  Design tokens (warna, spacing, typography, dsb.) dan CSS global yang dipakai di seluruh UI.

### 1.1 Domain

File utama: `src/domain/component.rs` dan `src/domain/validation.rs`.

- Menyediakan tipe-tipe komponen: `ButtonComponent`, `TextComponent`, `InputComponent`, `ContainerComponent`, `CustomComponent`.
- Menyediakan enum pendukung: `ButtonVariant`, `ButtonSize`, `TextStyle`, `TextTag`, `InputType`, `LayoutType`, dll.
- `CanvasComponent` adalah enum pembungkus yang digunakan di Canvas untuk menyimpan semua komponen dalam satu koleksi.
- `PropValue` (`String`, `Number`, `Boolean`, `Null`) dipakai untuk mewakili nilai properti secara generik.
- Modul validasi berisi fungsi-fungsi untuk memeriksa nama komponen, template HTML, dan aturan lain.

### 1.2 State

File utama: `src/state/app_state.rs`, `src/state/project.rs`, `src/state/history.rs`.

- `AppState` adalah root state yang dibagikan via Leptos context.
- `UiState` menyimpan hal-hal seperti:
  - visibilitas panel (command palette, export modal, git, debug),
  - notifikasi,
  - `custom_components` dan `component_library`,
  - metrik render (`render_count`, `render_time`).
- `LibraryComponent` + `PropSchema` mendeskripsikan komponen di library dan schema properti yang menggerakkan Property Editor.
- `default_components()` mengisi `component_library` dengan Button/Text/Input/Container beserta `props_schema`-nya.
- Modul project menyediakan struktur serializable untuk seluruh project.
- Modul history mengelola undo/redo stack.

### 1.3 Services

File utama: `src/services/export_service.rs`, `project_service.rs`, `git_service.rs`, `property_service.rs`.

- **Export service** – mengubah state canvas menjadi beberapa format (Leptos, HTML, JSON, Markdown) dengan unit test.
- **Project service** – `project_to_json` dan `project_from_json` untuk export/import project.
- **Git service** – trait `GitBackend` dan `NoopGitBackend` sebagai stub untuk integrasi Git nyata (HTTP/Tauri) di masa depan.
- **Property service** – fungsi `update_button_prop`, `update_text_prop`, `update_input_prop` yang menerima `PropValue` dan mengembalikan komponen baru.

Services tidak tahu apa-apa tentang Leptos view; hanya beroperasi di atas domain/state types.

### 1.4 Builder / UI

File utama: `src/app.rs`, `src/builder/*`.

- `App` (di `src/app.rs`) menginisialisasi `AppState`, memasang keyboard handler, dan merender layout utama (Sidebar + Canvas + panel-panel).
- `builder/canvas` berisi komponen `Canvas` dan `ComponentRenderer` untuk merender `CanvasComponent` menjadi HTML.
- `builder/drag_drop.rs` mengelola state drag-and-drop.
- `builder/sidebar.rs` menampilkan library komponen dan editor Custom Components.
- `builder/property_editor.rs` menampilkan Property Editor yang kini schema-driven (Button, Text, Input).
- `builder/git_panel.rs`, `builder/project.rs`, `builder/debug_panel.rs`, dan `builder/command_palette.rs` adalah panel fitur tambahan.

### 1.5 Utils & Styling

- `utils/clipboard.rs` – operasi copy/read clipboard berbasis Web API dengan error handling.
- `utils/format.rs` – helper untuk formatting string dan ukuran file.
- `builder/design_tokens.rs` – mapping design token → CSS variables.
- `style.css` – kumpulan class dan penggunaan CSS variables untuk konsistensi visual.

---

## 2. Main Data Flows

Bagian ini merangkum beberapa alur data kunci dalam aplikasi.

### 2.1 Drag & Drop Komponen ke Canvas

1. User drag item dari Sidebar (`LibraryComponent`).
2. `drag_drop` mengubah state `DragState` dan menangkap data komponen yang dipilih.
3. `Canvas` menerima event drop melalui `DropZone`.
4. Canvas membuat `CanvasComponent` baru dari informasi yang di-drag dan menambahkannya ke state canvas di `AppState`.
5. Perubahan ini dicatat di history untuk mendukung undo/redo.

### 2.2 Property Editor Schema-driven

1. Saat user memilih komponen di Canvas, `canvas_state.selected` di-update.
2. `PropertyEditor` membaca `CanvasComponent` terpilih dan jenisnya (Button/Text/Input).
3. `PropertyEditor` mencari `LibraryComponent` yang sesuai di `UiState.component_library` dan mengambil `props_schema`.
4. Untuk setiap `PropSchema`:
   - Ditentukan nilai properti saat ini dari komponen domain.
   - Dirender input generik berdasarkan `prop_type` (`string`, `bool`, `enum:...`).
5. Event `on:input` / `on:change` memanggil `update_*_prop` di `property_service`, yang mengembalikan komponen baru.
6. `canvas_state.update_component` mengganti komponen lama di state dengan versi baru.

### 2.3 Export Project

1. User membuka export modal (via tombol atau keyboard shortcut).
2. `App` menyiapkan snapshot state project (canvas + metadata) lewat helper di `AppState`.
3. `export_service` digunakan untuk mengubah snapshot menjadi format yang diinginkan (Leptos/HTML/JSON/Markdown).
4. Hasil export dapat di-
   - copy ke clipboard (via `utils::clipboard`), atau
   - di-download sebagai file (menggunakan data URL di browser).

### 2.4 Project Save/Load

1. Panel Project (`builder/project.rs`) menyediakan tombol `New`, `Export JSON`, dan `Import JSON`.
2. `New` membuat project kosong dan menerapkannya ke `AppState`.
3. `Export JSON` menggunakan `project_service::project_to_json` untuk membuat representasi JSON project; user bisa copy atau download.
4. `Import JSON` menggunakan `project_service::project_from_json`; jika berhasil, hasilnya diterapkan ke `AppState` dengan `apply_project`.

### 2.5 Git Panel

1. Panel Git (`builder/git_panel.rs`) menggunakan trait `GitBackend` dari `git_service`.
2. Implementasi runtime saat ini adalah `NoopGitBackend` (tidak melakukan operasi nyata), tetapi API sudah tersedia:
   - status,
   - log,
   - commit, dll.
3. Di masa depan, implementasi nyata bisa disuntikkan (mis. via HTTP/Tauri) tanpa perlu mengubah UI secara besar-besaran.

### 2.6 Keyboard Shortcuts

1. `builder/keyboard.rs` mendefinisikan `KeyboardAction` dan shortcut (Undo, Redo, Save, Delete, Copy, Paste, Duplicate, OpenCommandPalette, Export, dsb.).
2. `App` menangkap event keyboard dan memetakan ke aksi pada `AppState` (mis. memanggil fungsi history, membuka panel, dsb.).
3. Layer builder hanya bertugas memetakan input user → pemanggilan fungsi pada state/services.

---

## 3. Prinsip Desain

Beberapa prinsip yang sudah tercermin di kode saat ini:

- **Separation of concerns** – domain, state, services, dan UI dipisah untuk memudahkan testing dan evolusi kode.
- **Schema-driven editing** – Property Editor untuk Button/Text/Input memanfaatkan `props_schema` + `PropValue` agar UI lebih generik dan extensible.
- **Testability** – banyak logic penting (terutama export dan domain) memiliki unit test.
- **Extensibility** – trait `GitBackend`, `CustomComponent`, dan `ComponentRegistry` dirancang supaya integrasi baru (Git backend, jenis komponen custom) dapat ditambahkan tanpa mematahkan arsitektur dasar.
