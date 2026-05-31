use crate::config::MAX_FILES;
use crate::error::AppError;
use crate::state::{AppState, FileMeta};
use crate::utils::sanitize::sanitize_filename;
use time::OffsetDateTime;
use tokio::{fs, io::AsyncWriteExt};

pub async fn save_file(
    state: &AppState,
    filename: &str,
    mut field: axum::extract::multipart::Field<'_>,
) -> Result<String, AppError> {
    let safe_name =
        sanitize_filename(filename).ok_or_else(|| AppError::bad_request("bad filename"))?;

    let path = state.upload_dir.join(&safe_name);
    let mut file = fs::File::create(&path)
        .await
        .map_err(|_| AppError::internal("cannot create file"))?;

    while let Some(chunk) = field
        .chunk()
        .await
        .map_err(|_| AppError::bad_request("bad upload stream"))?
    {
        file.write_all(&chunk)
            .await
            .map_err(|_| AppError::internal("write failed"))?;
    }

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
