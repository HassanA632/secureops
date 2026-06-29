mod app;
mod config;
mod state;

use std::net::SocketAddr;

use config::Config;
use sqlx::postgres::PgPoolOptions;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("secureops_api=debug,tower_http=debug")
        .init();

    let config = Config::from_env()?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let state = AppState { db };
    let app = app::build_app(state);

    let address: SocketAddr = format!("{}:{}", config.api_host, config.api_port).parse()?;

    tracing::info!("SecureOps API running on http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
