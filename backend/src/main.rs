use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    path::Path as FilePath,
};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProjectMetadata {
    id: String,
    name: String,
    last_modified: f64,
    component_count: usize,
}

type Store = Arc<RwLock<HashMap<String, serde_json::Value>>>;

fn get_data_file() -> String {
    std::env::var("DATA_FILE").unwrap_or_else(|_| "projects.json".to_string())
}

// Load store synchronously at startup (acceptable blocking)
fn load_store() -> HashMap<String, serde_json::Value> {
    let path = get_data_file();
    if FilePath::new(&path).exists() {
        if let Ok(file) = std::fs::File::open(&path) {
            let reader = std::io::BufReader::new(file);
            if let Ok(map) = serde_json::from_reader(reader) {
                tracing::info!("Loaded projects from {}", path);
                return map;
            }
        }
        tracing::error!("Failed to load projects from {}", path);
    }
    HashMap::new()
}

// Save store asynchronously
async fn save_store(store: &HashMap<String, serde_json::Value>) -> std::io::Result<()> {
    let path = get_data_file();
    let data = serde_json::to_vec_pretty(store)?;
    tokio::fs::write(&path, data).await
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let initial_data = load_store();
    let store = Arc::new(RwLock::new(initial_data));

    // CORS
    // Note: 'Any' origin is used for development convenience.
    // In production, this should be restricted to the specific frontend origin.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/projects", get(list_projects).post(save_project))
        .route("/api/projects/:id", get(get_project).delete(delete_project))
        .layer(cors)
        .with_state(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_projects(State(store): State<Store>) -> Json<Vec<ProjectMetadata>> {
    let store = store.read().await;
    let mut projects: Vec<ProjectMetadata> = store.values().map(|p| {
        let id = p.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled").to_string();
        let last_modified = p.get("last_modified").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let component_count = p.get("layout")
            .and_then(|l| l.as_array())
            .map(|a| a.len())
            .unwrap_or(0);

        ProjectMetadata {
            id,
            name,
            last_modified,
            component_count,
        }
    }).collect();

    // Sort by last modified desc
    projects.sort_by(|a, b| b.last_modified.partial_cmp(&a.last_modified).unwrap_or(std::cmp::Ordering::Equal));

    Json(projects)
}

async fn save_project(
    State(store): State<Store>,
    Json(mut payload): Json<serde_json::Value>,
) -> Result<Json<ProjectMetadata>, StatusCode> {
    // Extract or generate ID
    let id = payload.get("id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Ensure ID is in payload
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("id".to_string(), serde_json::Value::String(id.clone()));
        // Ensure last_modified is updated if not present (though frontend should send it)
        if !obj.contains_key("last_modified") {
             obj.insert("last_modified".to_string(), serde_json::Value::from(0.0));
        }
    }

    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled").to_string();
    let last_modified = payload.get("last_modified").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let component_count = payload.get("layout").and_then(|l| l.as_array()).map(|a| a.len()).unwrap_or(0);

    {
        let mut guard = store.write().await;
        // Insert into memory first
        guard.insert(id.clone(), payload);

        // Try to save to disk
        if let Err(e) = save_store(&guard).await {
            tracing::error!("Failed to save store: {}", e);
            // Rollback in-memory change to keep state consistent?
            // Or keep it and return error?
            // Returning error indicates to client that persistence failed.
            // Removing it ensures consistency.
            guard.remove(&id);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Json(ProjectMetadata {
        id,
        name,
        last_modified,
        component_count,
    }))
}

async fn get_project(
    Path(id): Path<String>,
    State(store): State<Store>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = store.read().await;
    if let Some(project) = store.get(&id) {
        Ok(Json(project.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_project(
    Path(id): Path<String>,
    State(store): State<Store>,
) -> StatusCode {
    let mut guard = store.write().await;

    if let Some(removed_project) = guard.remove(&id) {
        if let Err(e) = save_store(&guard).await {
            tracing::error!("Failed to save store after delete: {}", e);
            // Rollback: put it back
            guard.insert(id, removed_project);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
