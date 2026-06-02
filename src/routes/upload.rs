use crate::auth::jwt;
use crate::error::Result;
use crate::state::AppState;
use crate::storage::file_manager;
use axum::{
    Json,
    extract::{Multipart, State},
};
use tracing::{debug, info, warn};

pub async fn upload(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>> {
    info!("Upload request received");

    let token = get_bearer_token(&headers)?;
    debug!("Bearer token extracted");

    let _claims = jwt::verify_token(&state, token, "upload").map_err(|e| {
        warn!(error = ?e, "JWT verification failed");
        e
    })?;

    info!("JWT verified successfully, scope=upload");

    while let Some(field) = multipart.next_field().await.map_err(|_| {
        warn!("Failed to read next multipart field");
        crate::error::AppError::bad_request("bad multipart")
    })? {
        if field.name() != Some("file") {
            debug!("Skipping non-file field in multipart");
            continue;
        }

        let filename = field
            .file_name()
            .ok_or_else(|| {
                warn!("File field without filename");
                crate::error::AppError::bad_request("missing file name")
            })?
            .to_string();

        info!(filename = %filename, "Processing uploaded file");

        let saved_name = file_manager::save_file(&state, &filename, field).await?;
        info!(saved_name = %saved_name, "File saved to disk");

        file_manager::apply_limit_and_track(&state, &saved_name).await?;
        debug!(saved_name = %saved_name, "File limit applied and tracked");

        let download_url = format!("{}/download/{}", state.public_base_url, saved_name);

        info!(
            action = "file_uploaded",
            filename = %saved_name,
            download_url = %download_url,
            "File successfully uploaded"
        );

        return Ok(Json(UploadResponse {
            file: saved_name,
            download_url,
        }));
    }

    warn!("No file field found in multipart");
    Err(crate::error::AppError::bad_request("file field not found"))
}

fn get_bearer_token(headers: &axum::http::HeaderMap) -> Result<&str> {
    let auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    auth.strip_prefix("Bearer ").ok_or_else(|| {
        warn!("Failed to strip Bearer prefix");
        crate::error::AppError::unauthorized("invalid auth scheme")
    })
}

#[derive(serde::Serialize)]
pub struct UploadResponse {
    file: String,
    download_url: String,
}
