use crate::config::MAX_FILES;
use crate::error::AppError;
use crate::state::{AppState, FileMeta};
use crate::utils::sanitize::sanitize_filename;
use time::OffsetDateTime;
use tokio::{fs, io::AsyncWriteExt};

use tracing::{error, info, debug};

pub async fn save_file(
    state: &AppState,
    filename: &str,
    mut field: axum::extract::multipart::Field<'_>,
) -> Result<String, AppError> {
    debug!(
        "sanitize_filename({:?}) → {:?}",
        filename,
        sanitize_filename(filename)
    );
    let safe_name =
        sanitize_filename(filename).ok_or_else(|| AppError::bad_request("bad filename"))?;

    let path = state.upload_dir.join(&safe_name);
    info!("Saving to: {:?}", path);

    let mut file = fs::File::create(&path).await.map_err(|e| {
        error!("Failed to create file: {}", e);
        AppError::internal("cannot create file")
    })?;

    let mut total_bytes = 0;
    while let Some(chunk) = field.chunk().await.map_err(|e| {
        error!("Error reading field chunk: {}", e.status());
        if is_size_error(&e) {
            AppError::too_large()
        } else {
            AppError::bad_request("bad request")
        }
    })? {
        total_bytes += chunk.len();
        file.write_all(&chunk).await.map_err(|e| {
            error!("Error writing to file: {}", e);
            AppError::internal("write failed")
        })?;
    }
    debug!("Saved {} bytes", total_bytes);

    Ok(safe_name)
}

pub async fn apply_limit_and_track(state: &AppState, filename: &str) -> Result<(), AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let mut q = state.files.lock().await;

    if let Some(pos) = q.iter().position(|m| m.name == filename) {
        q.remove(pos);
    }

    q.push_back(FileMeta {
        name: filename.to_string(),
        created_at_unix: now,
    });

    while q.len() > MAX_FILES {
        if let Some(oldest) = q.pop_front() {
            let _ = fs::remove_file(state.upload_dir.join(&oldest.name)).await;
        }
    }

    Ok(())
}

fn is_size_error(e: &axum::extract::multipart::MultipartError) -> bool {
    let text = e.status().canonical_reason().unwrap().to_lowercase();
    text.contains("length limit")
        || text.contains("payload too large")
        || text.contains("content length")
}