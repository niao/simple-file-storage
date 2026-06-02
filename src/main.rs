// src/main.rs
pub mod app;
pub mod auth;
pub mod config;
pub mod error;
pub mod routes;
pub mod state;
pub mod storage;
pub mod utils;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        // .json()
        .pretty()
        .init();

    tracing::info!("Starting application bootstrap...");

    let state = state::AppState::new_from_env()?;
    tracing::info!("AppState loaded from environment");

    tokio::fs::create_dir_all(&state.upload_dir).await?;
    tracing::info!(dir = %state.upload_dir.display(), "Upload directory ensured");

    state.bootstrap().await?;
    tracing::info!("Application state bootstrapped");

    let app = app::create_app(state);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let bind_addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!("Listening on {}", bind_addr);

    axum::serve(listener, app).await?;
    Ok(())
}
