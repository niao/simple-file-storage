use crate::routes::{download, health, internal, upload};
use crate::state::AppState;
use axum::extract::DefaultBodyLimit;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub fn create_app(state: AppState) -> Router {
    let mut app = Router::new().route("/health", get(health::health));

    let uri_prefix = std::env::var("URI_PREFIX").ok().and_then(|s| {
        let s = s.trim().trim_matches('/');
        if s.is_empty() {
            None
        } else {
            Some(format!("/{}", s))
        }
    });

    let api_routes = Router::new()
        .route("/upload", post(upload::upload))
        .route("/download/{filename}", get(download::download))
        .route("/internal/token", get(internal::internal_token))
        .with_state(state.clone());

    if let Some(prefix) = &uri_prefix {
        tracing::info!(prefix = %prefix, "Nesting API routes under prefix");
        app = app.nest(prefix, api_routes);
    } else {
        tracing::info!("Merging API routes at root");
        app = app.merge(api_routes);
    }

    app.layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(512_000_000))
        .with_state(state)
}
