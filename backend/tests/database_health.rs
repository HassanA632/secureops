use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[path = "../src/app.rs"]
mod app;

#[path = "../src/state.rs"]
mod state;

use state::AppState;

#[tokio::test]
async fn database_health_check_returns_ok_when_database_is_available() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://secureops:secureops@localhost:5432/secureops".to_string());

    let db = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("database must be running for this test");

    let app = app::build_app(AppState { db });

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health/database")
                .body(Body::empty())
                .expect("failed to build request"),
        )
        .await
        .expect("request failed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("failed to read response body")
        .to_bytes();

    let json: Value = serde_json::from_slice(&body).expect("response body was not valid JSON");

    assert_eq!(json["status"], "ok");
    assert_eq!(json["database"], "connected");
}
