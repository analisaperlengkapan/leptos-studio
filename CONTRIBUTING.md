# Contributing to Leptos Studio

Terima kasih atas minat Anda untuk berkontribusi ke Leptos Studio! Kami menyambut kontribusi dari semua kalangan.

## 🚀 Cara Berkontribusi

### 1. Setup Development Environment

```bash
# Fork repository di GitLab
# Clone fork Anda
git clone git@gitlab.com:yourusername/leptos-studio.git
cd leptos-studio

# Install Rust dan Trunk jika belum ada
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install trunk

# Jalankan development server
trunk serve
```

### 2. Workflow Kontribusi

1. **Create Issue**: Buat issue untuk bug report atau feature request
2. **Fork & Branch**: Fork repository dan buat branch baru
   ```bash
   git checkout -b feature/nama-fitur
   ```
3. **Develop**: Implementasikan perubahan Anda
4. **Test**: Pastikan aplikasi berfungsi dengan baik
   ```bash
   trunk build  # Test build
   ```
5. **Commit**: Commit dengan pesan yang jelas
   ```bash
   git commit -m "feat: tambah fitur xyz"
   ```
6. **Push & PR**: Push ke fork dan buat merge request

### 3. Pedoman Kode

#### Rust/Leptos Best Practices
- Ikuti konvensi naming Rust (snake_case untuk variables, PascalCase untuk types)
- Gunakan `#[component]` macro untuk Leptos components
- Maintain reactive patterns dengan signals
- Handle errors dengan proper error types

#### Code Style
```rust
// ✅ Good
#[component]
pub fn MyComponent(props: MyProps) -> impl IntoView {
    let state = create_rw_signal(initial_value);
    
    view! {
        <div class="my-component">
            // content
        </div>
    }
}

// ❌ Bad
pub fn mycomponent() -> impl IntoView { /* ... */ }
```

#### Commit Messages
Gunakan [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` - fitur baru
- `fix:` - bug fix
- `docs:` - perubahan dokumentasi
- `refactor:` - refactoring kode
- `test:` - menambah/mengubah tests

### 4. Areas yang Membutuhkan Kontribusi

#### 🔧 Priority High
- [ ] Undo/redo system implementation
- [ ] Unit dan integration tests
- [ ] Performance optimizations

#### 🎨 UI/UX Improvements  
- [ ] Component library management
- [ ] Theme system
- [ ] Responsive design preview
- [ ] Better error handling UI

#### 📚 Documentation
- [ ] API documentation
- [ ] User tutorials
- [ ] Video guides
- [ ] Translation (Indonesian/English)

#### 🧪 Testing
- [ ] Unit tests untuk core modules
- [ ] Integration tests untuk drag-and-drop
- [ ] E2E tests dengan browser automation

### 5. Project Structure

```
src/
├── app.rs                 # Main app component
├── builder/              # Core builder functionality
│   ├── canvas.rs         # Drag-and-drop canvas
│   ├── sidebar.rs        # Component palette  
│   ├── property_editor.rs # Property editing
│   ├── preview.rs        # Live preview
│   └── export.rs         # Code generation
├── components/           # Reusable UI components
└── lib.rs               # Library root
```

### 6. Review Process

1. **Automated Checks**: CI akan memeriksa build dan formatting
2. **Code Review**: Maintainer akan review code changes
3. **Testing**: Manual testing untuk UI/UX changes
4. **Merge**: Setelah approval, akan di-merge ke main branch

### 7. Community Guidelines

- **Respectful**: Hormati semua kontributor
- **Constructive**: Berikan feedback yang membangun  
- **Collaborative**: Bekerja sama untuk mencapai tujuan bersama
- **Inclusive**: Sambut kontributor dari berbagai background

### 8. Getting Help

- **Discord**: Join server untuk diskusi real-time
- **Issues**: Gunakan GitHub issues untuk pertanyaan
- **Documentation**: Baca docs di README dan wiki

### 9. Recognition

Kontributor akan diakui di:
- Contributors section di README
- Release notes untuk kontribusi signifikan
- Special mentions di project updates

Mari bersama-sama membangun Leptos Studio menjadi tools terbaik untuk Rust UI development! 🚀
