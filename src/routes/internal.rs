use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};

use tracing::{info, warn};

pub async fn internal_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<TokenQuery>,
) -> Result<Json<TokenResponse>, ApiError> {
    let provided = headers
        .get("x-internal-secret")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if provided != state.internal_secret.as_str() {
        warn!(
            action = "internal_auth_failed",
            "Invalid internal secret attempt"
        );

        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "invalid internal secret",
        ));
    }

    let sub = q.sub.clone().unwrap_or_else(|| "internal".to_string());

    info!(
        action = "internal_token_request",
        sub = %sub,
        "Issuing internal JWT"
    );

    let token = crate::auth::jwt::create_upload_token(&state, Some(sub))
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "jwt encode failed"))?;

    Ok(Json(TokenResponse { token }))
}
#[derive(Deserialize)]
pub struct TokenQuery {
    sub: Option<String>,
}

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
}
