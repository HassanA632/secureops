use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Serialize)]
struct DatabaseHealthResponse {
    status: &'static str,
    database: &'static str,
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/database", get(database_health_check))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "secureops-api",
    })
}

async fn database_health_check(State(state): State<AppState>) -> Json<DatabaseHealthResponse> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .expect("database health check failed");

    Json(DatabaseHealthResponse {
        status: "ok",
        database: "connected",
    })
}
