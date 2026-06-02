use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::Response,
};

use crate::error::ApiError;
use crate::state::AppState;
use crate::utils::sanitize::sanitize_filename;
use tokio::fs;
use tokio_util::io::ReaderStream;

pub async fn download(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> Result<Response, ApiError> {
    tracing::info!(filename = %filename, "Download request received");

    let safe = sanitize_filename(&filename).ok_or_else(|| {
        tracing::warn!(filename = %filename, "Invalid filename detected");
        ApiError::new(StatusCode::BAD_REQUEST, "bad filename")
    })?;

    let path = state.upload_dir.join(&safe);
    tracing::debug!(path = %path.display(), "Resolved file path");

    if !path.exists() {
        tracing::warn!(path = %path.display(), "File not found");
        return Err(ApiError::new(StatusCode::NOT_FOUND, "not found"));
    }

    let file = fs::File::open(&path).await.map_err(|e| {
        tracing::error!(path = %path.display(), error = ?e, "Failed to open file");
        ApiError::new(StatusCode::NOT_FOUND, "not found")
    })?;

    tracing::info!(filename = %safe, "Serving file");

    let stream = ReaderStream::new(file);
    let body = axum::body::Body::from_stream(stream);

    let mut resp = Response::new(body);
    *resp.status_mut() = StatusCode::OK;

    resp.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", safe)
            .parse()
            .unwrap(),
    );

    Ok(resp)
}

// pub async fn download(
//     State(state): State<AppState>,
//     Path(filename): Path<String>,
// ) -> Result<Response, ApiError> {
//     let safe = sanitize_filename(&filename)
//         .ok_or_else(|| ApiError::new(StatusCode::BAD_REQUEST, "bad filename"))?;
//
//     let path = state.upload_dir.join(&safe);
//     if !path.exists() {
//         return Err(ApiError::new(StatusCode::NOT_FOUND, "not found"));
//     }
//
//     let file = fs::File::open(&path)
//         .await
//         .map_err(|_| ApiError::new(StatusCode::NOT_FOUND, "not found"))?;
//
//     let stream = ReaderStream::new(file);
//     let body = axum::body::Body::from_stream(stream);
//
//     let mut resp = Response::new(body);
//     *resp.status_mut() = StatusCode::OK;
//
//     // чтобы браузер скачивал с “нормальным” именем
//     resp.headers_mut().insert(
//         header::CONTENT_DISPOSITION,
//         format!("attachment; filename=\"{}\"", safe)
//             .parse()
//             .unwrap(),
//     );
//
//     Ok(resp)
// }
