# Requirements Document - Leptos Studio Development Plan

## 1. Project Overview

### 1.1 Project Goals

Mengembangkan Leptos Studio dari visual UI builder yang sudah functional menjadi tool production-ready dengan performance optimization, enhanced code generation, dan user experience improvements. Fokus pada features yang realistic dan achievable dalam browser-based WASM architecture tanpa backend dependency.

### 1.2 Target Users

#### 1.2.1 Primary Users
- Rust developers yang menggunakan Leptos framework untuk web development
- Teams yang membutuhkan rapid prototyping untuk UI development
- Full-stack developers yang prefer visual tools untuk front-end work
- Open-source contributors dalam Leptos ecosystem

#### 1.2.2 Secondary Users
- Design-to-code workflow users yang ingin bridge antara design dan implementation
- Educators teaching Leptos/Rust web development
- Enterprise teams adopting Rust untuk web applications

### 1.3 Scope

#### 1.3.1 In Scope (Realistic & Achievable)
- Enhanced responsive design system (breakpoint-specific properties, media query generation)
- Performance optimization (virtual scrolling, lazy rendering, memory management)
- Advanced code generation (project scaffolding, better formatting, validation)
- User experience improvements (onboarding, accessibility, internationalization)
- Component library enhancements (import/export, better organization)
- Error handling improvements (better messages, recovery mechanisms)

#### 1.3.2 Out of Scope (Requires Backend or Not Feasible)
- Version control integration (Git operations require filesystem access via backend)
- True hot reload with file watcher (browser cannot watch external files)
- Real-time collaboration (requires WebSocket server)
- Backend code generation
- Native mobile app support
- AI-assisted features (future consideration)
- Cloud hosting/deployment services

## 2. Current State Assessment

### 2.1 Fully Implemented Features (Production Ready)

#### 2.1.1 Core Builder Features
- ✅ Drag & Drop system dengan visual feedback
- ✅ Property Editor untuk component configuration
- ✅ Live Preview panel
- ✅ Canvas dengan nested container support
- ✅ Component selection dan manipulation

#### 2.1.2 Component System
- ✅ Component Library (Button, Text, Input, Container)
- ✅ Custom Component CRUD (Add, Edit, Delete)
- ✅ Component validation (Rust identifier, HTML syntax)
- ✅ Component search/filter functionality

#### 2.1.3 Code Export
- ✅ Export to Leptos code
- ✅ Export to HTML
- ✅ Export to Markdown
- ✅ Export to JSON (layout data)
- ✅ Multiple export presets (Plain, ThawUi, LeptosMaterial, LeptosUse)

#### 2.1.4 State Management
- ✅ Undo/Redo system dengan history stack
- ✅ Layout persistence via localStorage
- ✅ Auto-save functionality

#### 2.1.5 UI/UX Features
- ✅ Keyboard shortcuts (Cmd+Z, Cmd+Y, Delete, Escape, Cmd+K)
- ✅ Command Palette (fuzzy search untuk actions)
- ✅ Breadcrumb navigation
- ✅ Snackbar notifications
- ✅ Debug panel (render stats, memory usage)

#### 2.1.6 Design System
- ✅ Design tokens (colors, spacing, typography, borders, shadows)
- ✅ Theme system (Light, Dark, Custom)
- ✅ Responsive mode toggle (Desktop, Tablet, Mobile)

### 2.2 Partially Implemented Features

#### 2.2.1 Responsive Design
- ✅ ResponsiveMode enum (Desktop/Tablet/Mobile)
- ✅ Canvas resize based on mode
- ✅ UI controls untuk switch mode
- ❌ Breakpoint-specific properties di property editor
- ❌ Media query generation dalam exported code
- ❌ Side-by-side preview mode

#### 2.2.2 Component Library Management
- ✅ Add/Edit/Delete custom components
- ✅ Search/filter functionality
- ✅ Basic validation
- ❌ Component categories dengan collapse/expand
- ❌ Component thumbnails/previews
- ❌ Import/Export component packs ke file

### 2.3 Not Implemented (Future Features)

#### 2.3.1 Performance Optimizations
- ❌ Virtual scrolling untuk large component lists
- ❌ Lazy rendering untuk off-screen components
- ❌ Component instance pooling
- ❌ Memory usage optimization
- ❌ Web Workers untuk background tasks
- ❌ Service Worker untuk offline support

#### 2.3.2 Advanced Code Generation
- ❌ Full project scaffolding (Cargo.toml, main.rs, etc)
- ❌ Auto-format dengan rustfmt
- ❌ Documentation comments generation
- ❌ Import optimization
- ❌ CI/CD templates

#### 2.3.3 User Experience
- ❌ Interactive onboarding tutorial
- ❌ Accessibility compliance (WCAG 2.1 AA)
- ❌ Internationalization (i18n/l10n)
- ❌ Component marketplace
- ❌ Video tutorials integration

### 2.4 Broken/Misleading Features (Need Fix)

#### 2.4.1 Git Integration
- ⚠️ **STATUS**: Broken - Requires backend server
- **ISSUE**: `git_panel.rs` uses HTTP requests to `/api/git/*` endpoints
- **PROBLEM**: Browser-based WASM app cannot access filesystem directly
- **SOLUTION**: Either remove feature OR implement backend server OR use Git web APIs (limited)

#### 2.4.2 "Hot Reload" Claim
- ⚠️ **STATUS**: Misleading terminology
- **CLAIM**: "Hot reload custom component: Edit template, preview langsung update tanpa reload"
- **REALITY**: This is just reactive form editing via Leptos signals, NOT true hot reload
- **TRUE HOT RELOAD**: Would require file watcher monitoring external files
- **SOLUTION**: Rename to "Reactive Component Editing" or clarify documentation

## 3. Business Requirements

### 3.1 Value Proposition

#### 3.1.1 Faster Development
Visual editor yang sudah functional dapat dipercepat dengan performance optimizations dan better code generation, meningkatkan produktivitas developer

#### 3.1.2 Production-Ready Code
Enhanced code generation dengan formatting, validation, dan project scaffolding menghasilkan code yang langsung dapat digunakan tanpa manual cleanup

#### 3.1.3 Professional User Experience
Onboarding, accessibility, dan internationalization membuat tool accessible untuk wider audience

#### 3.1.4 Ecosystem Integration
Existing integration dengan Leptos ecosystem (thaw-ui, leptos-material, leptos-use) dapat diperluas dengan better template system

### 3.2 Development Priorities

#### 3.2.1 Performance First
Focus on optimization untuk handle large projects (1000+ components) tanpa degradation

#### 3.2.2 Code Quality
Generated code harus production-ready: formatted, validated, documented

#### 3.2.3 User Experience
Lower learning curve dengan onboarding, improve accessibility untuk compliance

#### 3.2.4 Documentation Accuracy
Fix misleading claims, document limitations clearly, set realistic expectations

## 4. Functional Requirements

### 4.1 HIGH PRIORITY - Performance & Scalability

#### 4.1.1 Virtual Scrolling System
**Status**: Not Implemented | **Priority**: High | **Effort**: Medium

##### Requirements
- Implement virtual scrolling untuk component library list (>100 items)
- Render hanya visible items + buffer zone (10 items)
- Smooth scrolling dengan position calculations
- Dynamic item height support
- Performance target: 60 FPS scrolling

##### Technical Approach
- Use Intersection Observer API untuk detect visibility
- Implement windowing technique
- Maintain scroll position on updates
- Cache rendered components

#### 4.1.2 Lazy Rendering untuk Canvas
**Status**: Not Implemented | **Priority**: High | **Effort**: High

##### Requirements
- Lazy render off-screen canvas components
- Progressive rendering untuk large layouts (1000+ components)
- Maintain interactivity during rendering
- Visual loading indicators
- Performance target: <3s initial render for 1000 components

##### Technical Approach
- Render visible viewport first
- Queue off-screen components
- Use RequestAnimationFrame untuk batching
- Implement rendering budget (5ms per frame)

#### 4.1.3 Memory Optimization
**Status**: Partial | **Priority**: High | **Effort**: Medium

##### Requirements
- Memory usage monitoring dan alerts
- Undo stack size limits (configurable, default 50 states)
- Lazy deserialization untuk saved layouts
- Garbage collection triggers untuk large operations
- Target: <200MB untuk project dengan 1000 components

##### Technical Approach
- Implement LRU cache untuk undo stack
- Use weak references where appropriate
- Monitor heap size via performance API
- Trigger manual GC on large operations

#### 4.1.4 Bundle Size Optimization
**Status**: Partial | **Priority**: Medium | **Effort**: Low

##### Requirements
- WASM bundle target: <2MB compressed
- Code splitting untuk optional features
- Bundle size monitoring dalam CI
- Tree-shaking optimization
- Lazy loading untuk heavy dependencies

##### Technical Approach
- Use wasm-opt with aggressive optimization
- Split command palette, git panel as optional modules
- Monitor bundle size with cargo-bloat
- Remove unused dependencies

### 4.2 HIGH PRIORITY - Enhanced Responsive Design

#### 4.2.1 Breakpoint-Specific Properties
**Status**: Not Implemented | **Priority**: High | **Effort**: High

##### Requirements
- Property editor support breakpoint overrides
- Properties: padding, margin, font-size, display, width, height
- Visual indicator untuk breakpoint-specific values
- Copy properties across breakpoints
- Cascade from larger to smaller breakpoints

##### Technical Approach
- Extend CanvasComponent dengan breakpoint maps
- UI: Tabs atau dropdown untuk breakpoint selection
- Store per-breakpoint values in component data
- Implement inheritance logic (desktop → tablet → mobile)

#### 4.2.2 Media Query Generation
**Status**: Not Implemented | **Priority**: High | **Effort**: Medium

##### Requirements
- Auto-generate CSS media queries dalam exported code
- Mobile-first approach (base styles + min-width queries)
- Support custom breakpoints
- Generate responsive utility classes
- Clean, readable output

##### Technical Approach
- Modify export.rs generate_leptos_code()
- Collect breakpoint-specific properties
- Generate `@media (min-width: Xpx) { ... }` blocks
- Group properties by breakpoint
- Use CSS custom properties untuk reusability

#### 4.2.3 Side-by-Side Preview
**Status**: Not Implemented | **Priority**: Medium | **Effort**: Medium

##### Requirements
- Preview multiple breakpoints simultaneously
- Layout: 3-column view (Desktop | Tablet | Mobile)
- Synchronized scrolling (optional)
- Toggle individual breakpoints on/off
- Responsive iframe rendering

##### Technical Approach
- Create MultiViewport component
- Use iframe sandbox untuk isolated rendering
- Implement postMessage komunikasi
- CSS Grid layout untuk responsive arrangement

### 4.3 HIGH PRIORITY - Advanced Code Generation

#### 4.3.1 Project Scaffolding
**Status**: Not Implemented | **Priority**: High | **Effort**: High

##### Requirements
- Generate full Leptos project structure
- Create Cargo.toml dengan dependencies
- Generate main.rs entry point
- Create Trunk.toml build config
- Include README dan .gitignore
- Export as downloadable ZIP file

##### Technical Approach
- Template-based generation system
- Use JSZip atau similar untuk ZIP creation
- Download via browser File API
- Include dependency version detection
- Add setup instructions in generated README

#### 4.3.2 Code Formatting Integration
**Status**: Not Implemented | **Priority**: Medium | **Effort**: Medium

##### Requirements
- Auto-format exported Leptos code
- Follow rustfmt conventions
- Configurable indentation (2 or 4 spaces)
- Format view! macro properly
- Clean imports (remove unused, sort alphabetically)

##### Technical Approach
- Implement basic formatter rules
- Indent tracking during code generation
- Import deduplication logic
- Consider leptosfmt-wasm integration (if available)

#### 4.3.3 Documentation Generation
**Status**: Not Implemented | **Priority**: Low | **Effort**: Low

##### Requirements
- Generate doc comments untuk components
- Include prop descriptions
- Usage examples
- Component hierarchy documentation

##### Technical Approach
- Template-based doc comment generation
- Extract metadata from components
- Format as Rust doc comments (///)

### 4.4 MEDIUM PRIORITY - User Experience

#### 4.4.1 Interactive Onboarding
**Status**: Not Implemented | **Priority**: Medium | **Effort**: Medium

##### Requirements
- First-time user tutorial (5 steps)
- Interactive walkthrough: Drag → Edit → Export
- Skip tutorial option
- Progress indicator
- Completion celebration

##### Technical Approach
- Modal-based tutorial overlay
- Highlight target elements
- Use localStorage untuk track completion
- Dismissible dengan keyboard (Escape)

#### 4.4.2 Accessibility Compliance (WCAG 2.1 AA)
**Status**: Partial | **Priority**: Medium | **Effort**: High

##### Requirements
- All interactive elements keyboard accessible
- ARIA labels untuk screen readers
- Color contrast ratio ≥ 4.5:1
- Focus indicators visible
- Skip navigation links
- Form labels properly associated

##### Technical Approach
- Audit dengan axe-core atau similar
- Add ARIA attributes where missing
- Implement focus management
- Test dengan screen readers (NVDA, VoiceOver)
- Document accessibility features

#### 4.4.3 Internationalization (i18n)
**Status**: Not Implemented | **Priority**: Low | **Effort**: High

##### Requirements
- Multi-language UI support (English, Indonesian priority)
- Locale-aware date/number formatting
- RTL layout support
- Language switcher in settings
- Persistent language preference

##### Technical Approach
- Use leptos_i18n atau similar
- Extract all UI strings to translation files
- Implement language detection (browser preference)
- Use ICU MessageFormat untuk plurals
- Test with RTL languages (Arabic sample)

### 4.5 MEDIUM PRIORITY - Component Library Polish

#### 4.5.1 Component Categories
**Status**: Partial | **Priority**: Medium | **Effort**: Low

##### Requirements
- Collapsible category sections
- Categories: Basic, Forms, Layout, Navigation, Custom
- Drag-to-reorder within category
- Empty state untuk empty categories
- Category icons

##### Technical Approach
- Implement accordion component
- Group components by category field
- Use details/summary HTML elements
- Store collapsed state in localStorage

#### 4.5.2 Component Import/Export
**Status**: Not Implemented | **Priority**: Medium | **Effort**: Medium

##### Requirements
- Export component library to JSON file
- Import components from JSON file
- Validation on import (schema check)
- Merge strategy for duplicates
- Export individual components atau full library

##### Technical Approach
- Serialize component_library signal to JSON
- Use File API untuk import/export
- JSON schema validation
- Conflict resolution UI (skip, overwrite, rename)

#### 4.5.3 Component Thumbnails
**Status**: Not Implemented | **Priority**: Low | **Effort**: High

##### Requirements
- Generate visual preview thumbnail untuk components
- Update thumbnail on template change
- Display in component library
- Fallback icon for components without thumbnail

##### Technical Approach
- Render component to off-screen canvas
- Convert to data URL atau blob
- Store thumbnail in component metadata
- Lazy generation (on-demand)

### 4.6 LOW PRIORITY - Enhanced Error Handling

#### 4.6.1 Better Error Messages
**Status**: Partial | **Priority**: Medium | **Effort**: Low

##### Requirements
- Actionable error messages dengan suggestions
- Error codes untuk reference
- Link to documentation when relevant
- Copy error details button
- Error history log

##### Technical Approach
- Create error message database
- Include "How to fix" suggestions
- Implement error boundary per section
- Use snackbar dengan action buttons

#### 4.6.2 State Recovery
**Status**: Partial | **Priority**: Low | **Effort**: Medium

##### Requirements
- Auto-save every 30 seconds to backup slot
- Recover from backup on crash detection
- Multiple backup slots (keep last 3)
- User-initiated restore from backup
- Clear old backups automatically

##### Technical Approach
- Separate localStorage key untuk backups
- Timestamp-based backup rotation
- Detect crash via unload event listener
- Prompt user on next load if backup detected

### 4.7 NOT FEASIBLE - Requires Backend

#### 4.7.1 Version Control Integration (Git)
**Status**: Broken (Backend-Dependent) | **Priority**: N/A | **Feasibility**: ❌

##### Current Issue
- Existing git_panel.rs uses HTTP API endpoints (`/api/git/status`, `/api/git/commit`)
- Browser-based WASM cannot access filesystem directly
- Git operations require shell command execution

##### Possible Solutions
1. **Remove Feature** - Simplest, most honest approach
2. **Implement Backend** - Add Rust backend server (Actix/Axum)
3. **Use Git Web APIs** - Limited to GitHub/GitLab APIs (requires auth, limited operations)
4. **Document as "Requires Backend"** - Keep code but disable feature

##### Recommendation
**Remove feature from client-side app** OR **clearly document backend requirement**

### 4.8 NOT FEASIBLE - Browser Limitations

#### 4.8.1 True Hot Reload (File Watcher)
**Status**: Not Possible | **Priority**: N/A | **Feasibility**: ❌

##### Current Misleading Claim
- Documentation claims "hot reload custom component"
- Reality: Just reactive form editing via signals

##### Browser Limitations
- Cannot watch external files
- Cannot monitor filesystem changes
- File API only works with user-selected files

##### Possible Solutions
1. **Rename Feature** - "Reactive Component Editing" (accurate)
2. **Fix Documentation** - Remove "hot reload" terminology
3. **Backend File Watcher** - Requires backend server

##### Recommendation
**Fix documentation to accurately describe feature as reactive editing**

### 4.9 FUTURE CONSIDERATION - Requires Significant Architecture Changes

#### 4.9.1 Real-Time Collaboration
**Requirements**: WebSocket server, CRDT implementation, user authentication
**Feasibility**: ❌ (Requires backend)

#### 4.9.2 Plugin System
**Requirements**: Plugin API design, sandboxing, security model
**Feasibility**: ⚠️ (Possible but complex, low priority)

#### 4.9.3 AI-Assisted Features
**Requirements**: AI model integration, API costs
**Feasibility**: ⚠️ (Possible via external APIs, low priority)

## 5. Non-Functional Requirements

### 5.1 Performance

#### 5.1.1 Response Time (Realistic Targets)
- User interaction latency: <100ms untuk click, drag, type actions (✅ Currently meeting)
- Canvas rendering: Maintain 60 FPS untuk smooth animations (⚠️ Degrades with >100 components)
- Property update latency: <50ms via reactive signals (✅ Currently meeting)
- Initial application load: <3 seconds including WASM compilation (✅ Currently ~2.5s)
- Search/filter operations: <50ms untuk 1000+ items (❌ Need implementation)

#### 5.1.2 Throughput (Current vs Target)
- **Current**: Supports ~100 components comfortably
- **Target**: Support 1000+ components dengan virtual scrolling
- **Current**: Undo stack unlimited (memory leak risk)
- **Target**: Limit to 50 states dengan LRU eviction
- Export generation: <5 seconds untuk 500+ components (✅ Currently fast enough)

#### 5.1.3 Resource Usage (Measured Baselines)
- **Current Memory**: ~100MB untuk small project (10 components)
- **Target Memory**: <200MB untuk 1000 components (need optimization)
- **Current WASM bundle**: ~1.8MB compressed (✅ Meeting target)
- **Target Bundle**: <2MB compressed (maintain current size)
- **Current localStorage**: ~50KB untuk typical project (✅ Efficient)
- **Target localStorage**: <5MB limit awareness (add warnings)

### 5.2 Security

#### 5.2.1 Input Security (Current Status: ✅ Good)
- ✅ XSS prevention via Leptos framework automatic escaping
- ✅ HTML validation untuk custom component templates
- ✅ Rust identifier validation untuk component names
- ✅ No arbitrary code execution (WASM sandbox)
- ⚠️ Content Security Policy: Need explicit CSP headers in deployment

#### 5.2.2 Data Security (Current Status: ⚠️ Adequate)
- ✅ Privacy-first design (all data client-side, no tracking)
- ✅ No telemetry without consent
- ⚠️ localStorage unencrypted (browser limitation)
- ⚠️ Exported code not sanitized for injection (low risk)
- ✅ Dependency audit via cargo-audit in CI

#### 5.2.3 Authentication & Authorization (Current Status: N/A)
- ❌ Git authentication: Feature broken/disabled (no backend)
- N/A: No user accounts, no authentication required
- N/A: Single-user application (no authorization model)

### 5.3 Scalability (Realistic Limits)

#### 5.3.1 Data Volume (Current vs Target)
- **Current**: Tested up to ~100 components
- **Target**: Support 1000+ components (requires virtual scrolling)
- **Current**: Component library ~20 custom components
- **Target**: Support 100+ custom components efficiently
- **Current**: Undo history unlimited
- **Target**: Limit to 50-100 steps (configurable)
- **Current**: Nested depth tested to 5 levels
- **Target**: Support 10+ nested levels
- **localStorage limit**: 5-10MB (browser-dependent, non-configurable)

#### 5.3.2 Concurrent Users (Architecture Limitation)
- ✅ Single-user mode: Optimal (current architecture)
- ❌ Multi-user collaboration: **NOT FEASIBLE** without backend
- N/A: Conflict resolution not applicable

### 5.4 Usability

#### 5.4.1 Ease of Use (Current: ✅ Good)
- ✅ Learning curve: ~3 minutes untuk basic drag-drop workflow
- ✅ Keyboard shortcuts: Comprehensive (Cmd+Z/Y, Delete, Escape, Cmd+K)
- ✅ Command palette: Fuzzy search untuk all actions
- ✅ Design tokens: Consistent styling
- ⚠️ Tablet support: Functional but not optimized

#### 5.4.2 Accessibility (Current: ⚠️ Partial)
- ⚠️ WCAG 2.1 Level AA: Partial compliance (needs audit)
- ❌ Screen reader support: Limited ARIA labels
- ✅ Keyboard navigation: Full support
- ⚠️ Focus indicators: Present but could be more visible
- ⚠️ Color contrast: Mostly compliant (needs verification)

#### 5.4.3 Help & Documentation (Current: ⚠️ Basic)
- ❌ Inline tooltips: Not implemented
- ✅ Documentation: README, API.md, CONTRIBUTING.md complete
- ❌ Interactive tutorial: Not implemented
- ✅ User manual: Comprehensive in README
- ❌ Video tutorials: Not available

### 5.5 Reliability

#### 5.5.1 Availability (Current: ✅ Excellent)
- ✅ Client-side application: 100% uptime (no server dependency)
- ✅ Works offline after initial load
- ❌ Service Worker: Not implemented (future enhancement)
- ✅ No single point of failure (stateless client)

#### 5.5.2 Data Integrity (Current: ⚠️ Basic)
- ❌ Auto-save: Manual save only (should add periodic auto-save)
- ✅ State persistence: localStorage working
- ⚠️ Backup creation: Limited (undo stack only)
- ✅ Data validation: On load from localStorage
- ⚠️ Backward compatibility: Not explicitly tested

#### 5.5.3 Error Handling (Current: ⚠️ Adequate)
- ✅ Error boundaries: Implemented in key areas
- ✅ Snackbar notifications: User-friendly messages
- ❌ Error reporting: Not implemented
- ⚠️ Recovery suggestions: Limited

### 5.6 Maintainability

#### 5.6.1 Code Quality (Current: ✅ Excellent)
- ✅ Test coverage: 62+ tests passing (unit + integration)
- ✅ Documentation: API.md comprehensive
- ✅ Clippy compliance: Zero warnings
- ✅ Rustfmt: Consistent formatting
- ✅ Code complexity: Well-structured, modular

#### 5.6.2 Architecture (Current: ✅ Good)
- ✅ Modular design: 16 builder modules
- ✅ Separation of concerns: Clear module boundaries
- ✅ Leptos best practices: Signals, contexts, components
- ⚠️ Plugin architecture: Not designed yet

#### 5.6.3 Documentation (Current: ✅ Comprehensive)
- ✅ README: Complete with examples
- ✅ API.md: Internal API documented
- ✅ CONTRIBUTING.md: Guidelines clear
- ✅ CHANGELOG.md: Well-maintained
- ✅ SECURITY.md: Security policy documented
- ✅ Inline comments: Present where needed

### 5.7 Compatibility

### 5.7 Compatibility

#### 5.7.1 Browser Support (Tested & Verified)
- ✅ Chrome/Edge: Version 90+ (Primary development browser)
- ✅ Firefox: Version 88+ (Tested, working)
- ⚠️ Safari: Version 14+ (Basic testing, WASM issues possible)
- ⚠️ Opera: Should work (not explicitly tested)
- ❌ Internet Explorer: Not supported (no WASM)

#### 5.7.2 Framework Compatibility (Current Status)
- ✅ Leptos: Version 0.6 (current development version)
- ⚠️ Leptos 0.7, 0.8: Compatibility not tested (breaking changes possible)
- ✅ Rust: 1.70+ MSRV (tested with 1.75+)
- ✅ WASM target: wasm32-unknown-unknown
- ✅ Trunk: 0.18+ (latest stable)

#### 5.7.3 Operating System (Platform Independent)
- ✅ Windows: 10, 11 (via browser)
- ✅ macOS: 11+ (via browser)
- ✅ Linux: Any distro with modern browser
- ✅ Platform-independent (browser-based)

## 6. Constraints & Assumptions

### 6.1 Technical Constraints (CRITICAL LIMITATIONS)

#### 6.1.1 Browser Architecture Constraints
- **Pure client-side**: No backend server (by design)
- **No filesystem access**: Browser cannot read/write arbitrary files
- **No shell commands**: Cannot execute Git, rustfmt, clippy, etc.
- **localStorage limits**: 5-10MB typical (browser-dependent, non-configurable)
- **Same-origin policy**: Cannot access external resources without CORS
- **WASM limitations**: No threads (yet), limited SIMD support

#### 6.1.2 Impossible Features (Browser Limitations)
- ❌ **Git integration** without backend (no filesystem/shell access)
- ❌ **True hot reload** with file watcher (cannot monitor external files)
- ❌ **Direct rustfmt execution** (no shell access)
- ❌ **Real-time collaboration** without WebSocket server
- ❌ **Native notifications** (limited browser API)

#### 6.1.3 Performance Constraints
- **Main thread blocking**: Heavy computations freeze UI
- **Memory limits**: Varies by browser (typically 2-4GB)
- **WASM startup overhead**: ~500ms-1s compilation time
- **No multi-threading**: Single-threaded execution model

#### 6.1.4 Technology Constraints
- **Leptos framework**: Dependent on framework stability and updates
- **WASM ecosystem**: Limited library support compared to native
- **Browser APIs**: Feature availability varies by browser
- **No native file system**: Limited to File API (user-initiated only)

### 6.2 Business Constraints

## 5. Constraints & Assumptions

### 5.1 Technical Constraints

#### 5.1.1 Architecture Constraints
- Browser-based architecture (no native desktop app)
- Pure client-side processing (no backend server)
- WASM compilation overhead at startup
- localStorage size limits (5-10MB typical, browser-dependent)
- Same-origin policy limitations

#### 5.1.2 Technology Constraints
- Dependent on Leptos framework stability
- Rust compilation required untuk testing exported code
- Web APIs availability (File API, localStorage, etc)
- WASM feature support (threads, SIMD)

#### 5.1.3 Performance Constraints
- Browser memory limits (varies by device)
- Main thread blocking untuk heavy computations
- Network latency untuk Git operations
- File system access limitations (via browser APIs)

### 6.2 Business Constraints

#### 6.2.1 Licensing (Maintained)
- ✅ Open-source: Apache 2.0 license
- ✅ All dependencies compatible licenses
- ✅ No proprietary code

#### 6.2.2 Development Model
- ✅ Community-driven development
- ✅ Volunteer contributors
- ✅ No dedicated funding/budget
- ✅ Releases driven by feature completion

#### 6.2.3 Distribution
- ✅ Self-hosted deployment model (GitHub Pages, Netlify, etc)
- ✅ No cloud hosting service required
- ✅ No monetization/premium features
- ✅ Free to use and modify

### 6.3 Assumptions (Verified)

#### 6.3.1 User Assumptions
- ✅ Users have modern browsers dengan WASM support (Chrome 90+, Firefox 88+)
- ⚠️ Users have basic Leptos knowledge (documentation provides learning resources)
- ✅ Users have Rust toolchain untuk testing exports (documented requirement)
- ⚠️ Users understand reactive programming (onboarding can help)
- ❌ Git workflows: **REMOVED ASSUMPTION** (Git features broken/removed)

#### 6.3.2 Technical Assumptions
- ❌ Internet for Git: **N/A** (Git features not feasible)
- ✅ localStorage enabled: Required (graceful fallback needed)
- ✅ JavaScript enabled: Required for WASM
- ✅ Reasonable device specs: 4GB+ RAM, modern CPU (documented minimum)

#### 6.3.3 Environment Assumptions
- ✅ Development: Linux/macOS/Windows dengan Rust installed
- ✅ Trunk installed: Required for building
- ❌ Git installed: **NOT REQUIRED** (client-side app doesn't use Git)
- ✅ Text editor: VS Code recommended (any editor works)

### 6.4 Documentation Issues (CRITICAL FIXES NEEDED)

#### 6.4.1 Misleading Claims
- ❌ **"Hot reload custom component"**: Actually reactive form editing, NOT file watcher
- ❌ **"Version control (Git) UI"**: Feature is broken, requires backend
- ⚠️ Checklist shows "Hot reload" as missing, but README claims it's implemented

#### 6.4.2 Required Documentation Updates
1. Remove or clarify "hot reload" terminology
2. Document Git integration as "requires backend" or remove feature
3. Update checklist to reflect actual implementation status
4. Add "Browser Limitations" section to README
5. Document architecture constraints clearly

## 7. Success Criteria (Realistic & Measurable)

### 7.1 Phase 1: Documentation & Foundation (Immediate)

#### 7.1.1 Documentation Cleanup (CRITICAL)
- [ ] Fix misleading "hot reload" claim (rename to "reactive editing")
- [ ] Document Git integration limitations (requires backend OR remove feature)
- [ ] Add "Browser Limitations" section to README
- [ ] Update checklist to reflect actual implementation status
- [ ] Document architecture constraints explicitly

#### 7.1.2 Code Quality Maintenance
- [ ] All existing tests passing (62+ tests)
- [ ] Zero clippy warnings maintained
- [ ] CHANGELOG updated with corrections
- [ ] API.md reflects current implementation

### 7.2 Phase 2: Performance Optimization (Q1 2026)

#### 7.2.1 Virtual Scrolling Implementation
- [ ] Component library supports 100+ items without lag
- [ ] 60 FPS scrolling maintained
- [ ] Memory usage <150MB untuk 100 component library

#### 7.2.2 Large Layout Support
- [ ] Canvas handles 1000+ components with lazy rendering
- [ ] Initial render <5s untuk 1000 components
- [ ] 60 FPS during interactions
- [ ] Memory usage <300MB untuk 1000 component project

#### 7.2.3 Memory Optimization
- [ ] Undo stack limited to 50 states (configurable)
- [ ] No memory leaks detected (profiling verification)
- [ ] localStorage usage warnings at 4MB threshold

### 7.3 Phase 3: Enhanced Responsive Design (Q2 2026)

#### 7.3.1 Breakpoint-Specific Properties
- [ ] Property editor supports 3 breakpoints (desktop/tablet/mobile)
- [ ] Visual indicator untuk overridden properties
- [ ] Copy properties across breakpoints
- [ ] Cascade from larger to smaller breakpoints working

#### 7.3.2 Media Query Generation
- [ ] Export generates correct CSS media queries
- [ ] Mobile-first approach (base + min-width queries)
- [ ] Generated CSS is clean and readable
- [ ] Responsive layout verified in actual browser

#### 7.3.3 Responsive Preview Enhancement
- [ ] Side-by-side view for 3 breakpoints (optional)
- [ ] Accurate viewport sizing
- [ ] Synchronized canvas updates

### 7.4 Phase 4: Advanced Code Generation (Q2-Q3 2026)

#### 7.4.1 Project Scaffolding
- [ ] Generate full Leptos project structure
- [ ] Cargo.toml with correct dependencies
- [ ] main.rs entry point generated
- [ ] Trunk.toml configuration included
- [ ] Export as downloadable ZIP

#### 7.4.2 Code Quality
- [ ] Basic formatting (indentation, spacing)
- [ ] Import cleanup (remove unused, deduplicate)
- [ ] Generate doc comments for components
- [ ] Code passes `cargo check` without errors

### 7.5 Phase 5: User Experience Polish (Q3-Q4 2026)

#### 7.5.1 Onboarding System
- [ ] Interactive 5-step tutorial implemented
- [ ] Skip tutorial option
- [ ] Tutorial completion tracked
- [ ] <3 minutes to complete tutorial

#### 7.5.2 Accessibility Compliance
- [ ] WCAG 2.1 Level AA audit completed
- [ ] All interactive elements keyboard accessible
- [ ] ARIA labels for screen readers
- [ ] Color contrast ratios ≥4.5:1 verified
- [ ] Focus indicators visible

#### 7.5.3 Component Library Polish
- [ ] Collapsible categories implemented
- [ ] Component import/export to JSON working
- [ ] Search/filter performance <50ms

### 7.6 Quality Gates (Every Release)

#### 7.6.1 Testing
- [ ] All tests passing (100%)
- [ ] No new clippy warnings introduced
- [ ] Performance benchmarks: No regression >10%
- [ ] Manual testing checklist completed

#### 7.6.2 Documentation
- [ ] CHANGELOG updated with all changes
- [ ] Breaking changes documented
- [ ] Migration guide provided (if applicable)
- [ ] README screenshots updated (if UI changed)

#### 7.6.3 Security
- [ ] `cargo audit` passing (no vulnerabilities)
- [ ] Dependency updates reviewed
- [ ] No new security warnings

### 7.7 Long-Term Goals (2027+)

#### 7.7.1 Backend Integration (Optional)
- [ ] Design backend API for Git operations
- [ ] Implement Rust backend (Actix/Axum)
- [ ] Git integration working reliably
- [ ] File system access via backend

#### 7.7.2 Advanced Features
- [ ] Plugin system API design
- [ ] Basic plugin support (component plugins)
- [ ] Internationalization (English, Indonesian)
- [ ] Component marketplace (community)

### 7.8 Metrics Tracking (Ongoing)

#### 7.8.1 Performance Baselines
- **Current Load Time**: ~2.5s
- **Target Load Time**: <3s (maintain)
- **Current Memory**: ~100MB (small project)
- **Target Memory**: <200MB (typical), <500MB (large)
- **Current Bundle**: ~1.8MB compressed
- **Target Bundle**: <2MB (maintain)

#### 7.8.2 User Metrics (if analytics implemented)
- Time to first component drag
- Average project complexity (component count)
- Export frequency
- Feature usage statistics

---

**Document Version:** 2.0  
**Last Updated:** 2025-11-04  
**Status:** REVISED - Based on Deep Codebase Analysis  
**Changes**: Added Current State Assessment, Fixed misleading claims, Realistic prioritization

---

**Document Version:** 1.0  
**Last Updated:** 2025-11-04  
**Status:** Draft - Awaiting Approval
