mod app;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("secureops_api=debug,tower_http=debug")
        .init();

    let app = app::build_app();
    let address = SocketAddr::from(([0, 0, 0, 0], 8080));

    tracing::info!("SecureOps API running on http://{address}");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind API address");

    axum::serve(listener, app).await.expect("API server failed");
}
