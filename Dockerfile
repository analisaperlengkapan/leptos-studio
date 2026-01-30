# Build Stage for Backend
FROM rust:1.85-slim-bookworm AS backend-builder
WORKDIR /app
COPY . .
RUN cd backend && cargo build --release

# Build Stage for Frontend
FROM rust:1.85-slim-bookworm AS frontend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y binaryen
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
COPY . .
# Use the Trunk.toml in root which points to frontend/index.html
RUN trunk build --release

# Final Stage
FROM debian:bookworm-slim
WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/backend/target/release/backend /app/leptos-studio-backend

# Copy frontend assets
COPY --from=frontend-builder /app/dist /app/dist

# Environment variables
ENV DATA_FILE=projects.json
ENV TEMPLATES_FILE=templates.json
ENV GIT_DATA_FILE=git_data.json
ENV ANALYTICS_DATA_FILE=analytics.json
ENV LEPTOS_API_URL=http://localhost:3000
# Backend listens on 3000
EXPOSE 3000
# Frontend assets are served by backend?
# The backend is Axum. Does it serve static files?
# Let's check backend/src/main.rs.

# If backend doesn't serve static files, we need Nginx or modify backend.
# The current backend is just an API.
# Plan: Modify backend/src/main.rs to serve static files from /app/dist if not an API route.

CMD ["/app/leptos-studio-backend"]
