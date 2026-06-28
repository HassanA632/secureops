use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

#[path = "../src/app.rs"]
mod app;

#[tokio::test]
async fn health_check_returns_ok() {
    let app = app::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
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

    let json: Value = serde_json::from_slice(&body).expect("response body is not valid JSON");

    assert_eq!(json["status"], "ok");
    assert_eq!(json["service"], "secureops-api");
}
