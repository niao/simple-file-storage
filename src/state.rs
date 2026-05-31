use crate::utils::sanitize::sanitize_filename;
use std::collections::VecDeque;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct FileMeta {
    pub name: String,
    pub created_at_unix: i64,
}

#[derive(Clone)]
pub struct AppState {
    pub upload_dir: PathBuf,
    pub jwt_secret: Arc<Vec<u8>>,
    pub internal_secret: Arc<String>,
    pub public_base_url: Arc<String>,
    pub files: Arc<Mutex<VecDeque<FileMeta>>>,
}

impl AppState {
    pub fn new_from_env() -> anyhow::Result<Self> {
        let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./data".to_string());
        let jwt_secret =
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_change_me".to_string());
        let internal_secret =
            std::env::var("INTERNAL_SECRET").unwrap_or_else(|_| "internal_dev_secret".to_string());

        let port: u16 = std::env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000);

        let uri_prefix =
            crate::utils::uri::normalize_uri_prefix(std::env::var("URI_PREFIX").ok().as_deref());
        let public_base_url = std::env::var("PUBLIC_BASE_URL").unwrap_or_else(|_| {
            if uri_prefix.is_empty() {
                format!("http://localhost:{port}")
            } else {
                format!("http://localhost:{port}{uri_prefix}")
            }
        });

        Ok(Self {
            upload_dir: PathBuf::from(upload_dir),
            jwt_secret: Arc::new(jwt_secret.into_bytes()),
            internal_secret: Arc::new(internal_secret),
            public_base_url: Arc::new(public_base_url),
            files: Arc::new(Mutex::new(VecDeque::new())),
        })
    }

    pub async fn bootstrap(&self) -> anyhow::Result<()> {
        let mut entries = tokio::fs::read_dir(&self.upload_dir).await?;
        let mut files: Vec<(String, i64)> = Vec::new();

        while let Some(e) = entries.next_entry().await? {
            let meta = e.metadata().await?;
            if !meta.is_file() {
                continue;
            }
            let name = e.file_name().to_string_lossy().to_string();
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            if sanitize_filename(&name).is_some() {
                files.push((name, mtime));
            }
        }

        files.sort_by_key(|(_, t)| *t);

        // Удаляем лишние файлы, если превышено ограничение
        while files.len() > crate::config::MAX_FILES {
            let (name, _) = files.remove(0);
            let _ = tokio::fs::remove_file(self.upload_dir.join(&name)).await;
        }

        let mut q = self.files.lock().await;
        q.clear();
        for (name, t) in files {
            q.push_back(FileMeta {
                name,
                created_at_unix: t,
            });
        }

        Ok(())
    }
}
