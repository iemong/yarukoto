use std::path::PathBuf;

use serde_json::json;
use tauri::AppHandle;
use tokio::fs;

use crate::models::{SessionStatus, StoredToken};
use crate::{AppError, AppResult};

const TOKENS_FILE: &str = "tokens.json";

#[derive(Clone)]
pub struct TokenStorage {
    path: PathBuf,
}

impl TokenStorage {
    pub fn new(app: &AppHandle) -> AppResult<Self> {
        let mut dir = app
            .path()
            .app_config_dir()
            .ok_or_else(|| AppError::Storage("failed to resolve app config directory".into()))?;
        if !dir.exists() {
            std::fs::create_dir_all(&dir)
                .map_err(|err| AppError::Storage(format!("cannot create config dir: {err}")))?;
        }
        dir.push(TOKENS_FILE);
        Ok(Self { path: dir })
    }

    pub async fn save(&self, token: &StoredToken) -> AppResult<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|err| AppError::Storage(format!("cannot create parent dir: {err}")))?;
        }
        let data = serde_json::to_vec_pretty(token)
            .map_err(|err| AppError::Storage(format!("failed to serialize token: {err}")))?;
        fs::write(&self.path, data)
            .await
            .map_err(|err| AppError::Storage(format!("failed to write token file: {err}")))?;
        Ok(())
    }

    pub async fn load(&self) -> AppResult<Option<StoredToken>> {
        match fs::read(&self.path).await {
            Ok(bytes) => {
                let token = serde_json::from_slice(&bytes)
                    .map_err(|err| AppError::Storage(format!("failed to parse token: {err}")))?;
                Ok(Some(token))
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(AppError::Storage(format!(
                "failed to read token file: {err}"
            ))),
        }
    }

    pub async fn clear(&self) -> AppResult<()> {
        if fs::remove_file(&self.path).await.is_err() {
            // ignore missing file
        }
        Ok(())
    }

    pub async fn session_status(&self) -> AppResult<SessionStatus> {
        let token = self.load().await?;
        Ok(SessionStatus {
            is_authenticated: token.map(|t| t.is_valid()).unwrap_or(false),
        })
    }
}

pub async fn debug_dump(app: &AppHandle) -> AppResult<serde_json::Value> {
    let storage = TokenStorage::new(app)?;
    let status = storage.session_status().await?;
    Ok(json!({ "path": storage.path, "status": status.is_authenticated }))
}
