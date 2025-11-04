# Deployment Guide

Guide for deploying Leptos Studio to production environments.

## Prerequisites

### Required Tools

```bash
# Install Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown
```

## Build Process

### Development Build

```bash
# Start development server with hot reload
trunk serve

# Access at http://localhost:8080
```

### Production Build

```bash
# Clean previous builds
cargo clean
rm -rf dist/

# Build optimized WASM bundle
trunk build --release

# Output will be in dist/ directory
```

The production build includes:
- Optimized WASM binary
- Minified JavaScript
- Hashed filenames for cache busting
- All static assets

## Deployment Options

### 1. GitHub Pages

The repository includes an automated GitHub Actions workflow that deploys to GitHub Pages on push to main branch.

**Automatic Deployment** (configured in `.github/workflows/ci.yml`):
- Triggers on push to `main` branch
- Runs tests and builds WASM
- Deploys to `gh-pages` branch
- Accessible at `https://yourusername.github.io/leptos-studio/`

**Manual Deployment:**

```bash
# Build with base path
trunk build --release --public-url /leptos-studio/

# Deploy to gh-pages
cd dist
git init
git add .
git commit -m "Deploy to GitHub Pages"
git push -f https://github.com/yourusername/leptos-studio.git main:gh-pages
```

**GitHub Settings:**
1. Go to repository Settings → Pages
2. Source: Deploy from branch `gh-pages`
3. Directory: `/ (root)`
4. Save changes

### 2. Netlify

**Option A: Deploy via Netlify UI**
1. Connect your GitHub repository
2. Build command: `trunk build --release`
3. Publish directory: `dist`
4. Deploy

**Option B: Deploy via CLI**

```bash
# Install Netlify CLI
npm install -g netlify-cli

# Build
trunk build --release

# Deploy
netlify deploy --prod --dir=dist
```

**netlify.toml** (optional):

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
```

### 3. Vercel

```bash
# Install Vercel CLI
npm install -g vercel

# Build
trunk build --release

# Deploy
vercel --prod
```

**vercel.json**:

```json
{
  "buildCommand": "cargo install trunk && rustup target add wasm32-unknown-unknown && trunk build --release",
  "outputDirectory": "dist",
  "installCommand": "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
  "routes": [
    { "src": "/(.*)", "dest": "/index.html" }
  ]
}
```

### 4. Docker

**Dockerfile:**

```dockerfile
# Build stage
FROM rust:1.90-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
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
    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # WASM MIME type
    types {
        application/wasm wasm;
    }

    server {
        listen 80;
        root /usr/share/nginx/html;
        index index.html;

        # Security headers
        add_header X-Frame-Options "DENY" always;
        add_header X-Content-Type-Options "nosniff" always;

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

### 5. Static Hosting (Generic)

For any static hosting provider (Cloudflare Pages, S3, etc.):

1. Build: `trunk build --release`
2. Upload contents of `dist/` directory
3. Configure:
   - Single Page Application routing (redirect to index.html)
   - WASM MIME type: `application/wasm`
   - Enable gzip/brotli compression

## Configuration

### Environment Variables

```bash
# Set public URL for assets
trunk build --release --public-url /your-base-path/
```

### Trunk Configuration

Edit `Trunk.toml` for build customization:

```toml
[build]
target = "index.html"
release = true
dist = "dist"
public_url = "/"

[watch]
ignore = ["./dist"]

[serve]
address = "127.0.0.1"
port = 8080
```

## Optimization

### Build Optimization

Configured in `Cargo.toml`:

```toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
opt-level = "s"         # Optimize for size
strip = true            # Strip debug symbols
panic = "abort"         # Smaller panic handler
```

### WASM Size

- Current WASM size depends on dependencies
- Use `wasm-opt` for further optimization:

```bash
# Install wasm-opt
cargo install wasm-opt

# Optimize WASM file
wasm-opt -Oz -o dist/leptos_studio_bg_opt.wasm dist/leptos_studio_bg.wasm
```

### Caching Strategy

Recommended cache headers:
- `index.html`: `no-cache` (always fetch fresh)
- `*.wasm`, `*.js`, `*.css`: `max-age=31536000, immutable` (cache forever with hash)

## Monitoring

### Performance Metrics

Monitor these metrics:
- First Contentful Paint (FCP): < 2.0s
- Time to Interactive (TTI): < 5.0s
- WASM load time
- Total bundle size

Use browser DevTools or Lighthouse for performance auditing.

### Error Tracking

Add error tracking in production:

```rust
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    // Add your error tracking service here
    
    leptos::mount::mount_to_body(app::App);
}
```

## Troubleshooting

### Common Issues

**WASM fails to load:**
- Check MIME type is `application/wasm`
- Verify HTTPS is enabled
- Check browser console for errors

**Assets not loading:**
- Verify `public-url` matches deployment path
- Check base href in index.html
- Inspect network tab for 404s

**Build failures:**
```bash
cargo clean
rm -rf target/ dist/
trunk build --release
```

**Performance issues:**
- Enable gzip/brotli compression
- Use CDN for static assets
- Check WASM file size
- Profile with browser DevTools

## Production Checklist

Before deploying:

- [ ] Run `cargo test` - all tests pass
- [ ] Run `cargo clippy` - no warnings
- [ ] Build with `--release` flag
- [ ] Test in production-like environment
- [ ] Enable HTTPS
- [ ] Configure security headers (see SECURITY.md)
- [ ] Set up error tracking
- [ ] Test on target browsers
- [ ] Document deployment process
- [ ] Create rollback plan

## CI/CD

The project includes GitHub Actions workflow (`.github/workflows/ci.yml`):
- Runs on push to main/develop branches
- Executes tests and linting
- Builds WASM bundle
- Runs security audit
- Deploys to GitHub Pages (on main branch)

## Support

For deployment issues:
- GitHub Issues: https://github.com/analisaperlengkapan/leptos-studio/issues
- Documentation: README.md
- Security: SECURITY.md
