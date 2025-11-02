# Deployment Guide

This guide covers deploying Leptos Studio to production environments.

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Build Process](#build-process)
3. [Deployment Options](#deployment-options)
4. [Configuration](#configuration)
5. [Performance Optimization](#performance-optimization)
6. [Monitoring](#monitoring)
7. [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Tools
- Rust (latest stable): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Trunk: `cargo install trunk`
- WASM target: `rustup target add wasm32-unknown-unknown`

### Optional Tools
- `wasm-opt` for size optimization: `cargo install wasm-opt`
- `cargo-audit` for security: `cargo install cargo-audit`

## Build Process

### Development Build
```bash
# Quick development build
trunk serve

# Access at http://localhost:8080
```

### Production Build
```bash
# Clean previous builds
cargo clean
rm -rf dist/

# Build with optimizations
trunk build --release

# Output in dist/ directory
```

### Build Optimization
```bash
# Further optimize WASM size
wasm-opt -Oz -o dist/leptos_studio_bg.wasm dist/leptos_studio_bg.wasm

# Gzip compression
gzip -k dist/*.wasm
gzip -k dist/*.js
```

## Deployment Options

### 1. GitHub Pages

**Automatic Deployment (Recommended)**

The repository includes a GitHub Actions workflow that automatically deploys to GitHub Pages on push to main branch.

**Manual Setup:**
```bash
# Build
trunk build --release --public-url /leptos-studio/

# Deploy
cd dist
git init
git add .
git commit -m "Deploy"
git push -f https://github.com/yourusername/leptos-studio.git main:gh-pages
```

**Configuration:**
1. Go to repository Settings → Pages
2. Source: Deploy from branch `gh-pages`
3. Directory: root
4. Save

### 2. Netlify

**netlify.toml:**
```toml
[build]
  command = "trunk build --release"
  publish = "dist"

[build.environment]
  RUSTUP_TOOLCHAIN = "stable"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200

[[headers]]
  for = "/*"
  [headers.values]
    X-Frame-Options = "DENY"
    X-Content-Type-Options = "nosniff"
    Referrer-Policy = "strict-origin-when-cross-origin"
    Permissions-Policy = "geolocation=(), microphone=(), camera=()"

[[headers]]
  for = "/*.wasm"
  [headers.values]
    Content-Type = "application/wasm"
    Cache-Control = "public, max-age=31536000, immutable"

[[headers]]
  for = "/*.js"
  [headers.values]
    Cache-Control = "public, max-age=31536000, immutable"
```

**Deploy:**
```bash
netlify deploy --prod --dir=dist
```

### 3. Vercel

**vercel.json:**
```json
{
  "buildCommand": "cargo install trunk && rustup target add wasm32-unknown-unknown && trunk build --release",
  "outputDirectory": "dist",
  "installCommand": "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
  "framework": null,
  "routes": [
    {
      "src": "/(.*)",
      "dest": "/index.html"
    }
  ],
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "X-Frame-Options",
          "value": "DENY"
        },
        {
          "key": "X-Content-Type-Options",
          "value": "nosniff"
        }
      ]
    },
    {
      "source": "/*.wasm",
      "headers": [
        {
          "key": "Content-Type",
          "value": "application/wasm"
        },
        {
          "key": "Cache-Control",
          "value": "public, max-age=31536000, immutable"
        }
      ]
    }
  ]
}
```

### 4. Docker

**Dockerfile:**
```dockerfile
# Build stage
FROM rust:1.75-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install trunk and wasm target
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

# Copy source
WORKDIR /app
COPY . .

# Build
RUN trunk build --release

# Production stage
FROM nginx:alpine

# Copy built files
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx config
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
```

**nginx.conf:**
```nginx
events {
    worker_connections 1024;
}

http {
    include       /etc/nginx/mime.types;
    default_type  application/octet-stream;

    # WASM MIME type
    types {
        application/wasm wasm;
    }

    # Gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript application/wasm;
    gzip_min_length 1000;

    server {
        listen 80;
        server_name _;

        root /usr/share/nginx/html;
        index index.html;

        # Security headers
        add_header X-Frame-Options "DENY" always;
        add_header X-Content-Type-Options "nosniff" always;
        add_header Referrer-Policy "strict-origin-when-cross-origin" always;
        add_header Permissions-Policy "geolocation=(), microphone=(), camera=()" always;

        # CSP for WASM
        add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:;" always;

        # Cache static assets
        location ~* \.(wasm|js|css)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
        }

        # SPA fallback
        location / {
            try_files $uri $uri/ /index.html;
        }
    }
}
```

**Build and run:**
```bash
docker build -t leptos-studio .
docker run -p 8080:80 leptos-studio
```

### 5. Static Host (Generic)

For any static hosting service (S3, CloudFlare Pages, etc.):

1. Build: `trunk build --release`
2. Upload `dist/` contents
3. Configure:
   - Single Page Application routing
   - WASM MIME type: `application/wasm`
   - Gzip compression
   - Security headers (see SECURITY.md)

## Configuration

### Environment Variables

**Build-time:**
```bash
# Set public URL for assets
export PUBLIC_URL="/leptos-studio"
trunk build --release --public-url $PUBLIC_URL
```

### Trunk Configuration

Edit `Trunk.toml`:
```toml
[build]
target = "dist"
release = true
minify = "on"
filehash = true

[serve]
address = "0.0.0.0"
port = 8080
open = false
```

## Performance Optimization

### 1. WASM Size Reduction

**Cargo.toml optimizations:**
```toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
opt-level = "s"         # Optimize for size
strip = true            # Strip debug symbols
panic = "abort"         # Smaller panic handler

[profile.release.package."*"]
opt-level = "s"
```

### 2. Code Splitting
```bash
# Enable code splitting in Trunk
trunk build --release --features code-split
```

### 3. Asset Optimization
- Minimize CSS (done automatically by Trunk)
- Optimize images: Use WebP format
- Lazy load components
- Use CDN for fonts

### 4. Caching Strategy
```
# Cache-Control headers
/index.html:           no-cache
/*.wasm:               max-age=31536000, immutable
/*.js:                 max-age=31536000, immutable
/*.css:                max-age=31536000, immutable
```

## Monitoring

### Performance Metrics
- First Contentful Paint (FCP): Target < 1.5s
- Time to Interactive (TTI): Target < 3.5s
- WASM load time: Monitor
- LocalStorage operations: < 100ms

### Error Tracking

**Sentry Integration (Optional):**
```rust
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    // Add Sentry initialization here if needed
    
    leptos::mount_to_body(app::App);
}
```

### Health Checks
- Create `/health` endpoint
- Monitor WASM loading
- Check LocalStorage availability

## Troubleshooting

### Common Issues

**1. WASM fails to load**
```
Solution: Check MIME type configuration
- Ensure server returns Content-Type: application/wasm
- Check CSP headers allow 'wasm-unsafe-eval'
```

**2. Assets not loading**
```
Solution: Verify public URL
- trunk build --release --public-url /your-path/
- Update base href in index.html
```

**3. LocalStorage errors**
```
Solution: Check browser storage
- Clear browser cache
- Check storage quota
- Verify HTTPS (required for some browsers)
```

**4. Build failures**
```
Solution: Clean and rebuild
cargo clean
rm -rf target/ dist/
trunk build --release
```

**5. Performance issues**
```
Solution: Profile and optimize
- Use browser DevTools Performance tab
- Check WASM size (target < 500KB)
- Enable gzip compression
- Use CDN for static assets
```

### Debug Mode

```bash
# Build with debug info
RUST_BACKTRACE=1 trunk build

# Serve with logging
RUST_LOG=debug trunk serve
```

## Production Checklist

Before deploying to production:

- [ ] Run `cargo test` - all tests pass
- [ ] Run `cargo clippy` - no warnings
- [ ] Run `cargo audit` - no vulnerabilities
- [ ] Build with `--release` flag
- [ ] Test in production-like environment
- [ ] Configure security headers (see SECURITY.md)
- [ ] Enable HTTPS
- [ ] Set up monitoring
- [ ] Configure caching
- [ ] Test on target browsers (Chrome, Firefox, Safari, Edge)
- [ ] Optimize WASM size (< 500KB gzipped)
- [ ] Set up error tracking
- [ ] Document deployment process
- [ ] Create rollback plan
- [ ] Test rollback procedure

## Post-Deployment

### Verification
1. Check all pages load correctly
2. Test drag-and-drop functionality
3. Verify LocalStorage persistence
4. Test export functionality
5. Check browser console for errors
6. Validate security headers
7. Run Lighthouse audit (target > 90 score)

### Monitoring Setup
1. Configure analytics (privacy-respecting)
2. Set up error tracking
3. Monitor performance metrics
4. Track user feedback
5. Monitor server resources

## Updating

```bash
# Update dependencies
cargo update

# Test
cargo test

# Rebuild
trunk build --release

# Deploy
# (use your deployment method)
```

## Support

For deployment issues:
- GitHub Issues: https://github.com/analisaperlengkapan/leptos-studio/issues
- Documentation: See README.md
- Security: See SECURITY.md

---

**Next Steps:**
- Read [SECURITY.md](SECURITY.md) for security best practices
- Check [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
- Review [README.md](README.md) for usage instructions
