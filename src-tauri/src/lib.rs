mod google_tasks;
mod models;
mod oauth;
mod storage;

use google_tasks::{mock_task_lists, mock_tasks};
use models::{SessionStatus, Task, TaskList};
use oauth::{
    build_authorize_url, build_client, complete_flow, persist_tokens, pick_port, pkce_pair,
    redirect_url, session_status,
};
use serde::Serialize;
use std::sync::Arc;
use storage::TokenStorage;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("network error: {0}")]
    Network(String),
    #[error("storage error: {0}")]
    Storage(String),
    #[error("config error: {0}")]
    Config(String),
    #[error("oauth error: {0}")]
    OAuth(String),
    #[error("unknown error")]
    Unknown,
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Clone)]
pub struct AppState {
    http: Arc<reqwest::Client>,
}

impl Default for AppState {
    fn default() -> Self {
        let http = reqwest::Client::builder()
            .user_agent("Yarukoto/0.1.0")
            .build()
            .expect("http client");
        Self {
            http: Arc::new(http),
        }
    }
}

impl AppState {
    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }
}

#[derive(Debug, Serialize)]
pub struct AuthStartResponse {
    pub authorization_url: String,
}

#[derive(Debug, Serialize)]
pub struct AuthEventPayload {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn health_check() -> &'static str {
    "ok"
}

#[tauri::command]
pub async fn list_task_lists() -> Result<Vec<TaskList>, String> {
    mock_task_lists().await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn list_tasks() -> Result<Vec<Task>, String> {
    mock_tasks().await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn start_oauth(app: tauri::AppHandle) -> Result<AuthStartResponse, String> {
    let port = pick_port().map_err(|err| err.to_string())?;
    let redirect = redirect_url(port);
    let client = build_client(&redirect).map_err(|err| err.to_string())?;
    let storage = TokenStorage::new(&app).map_err(|err| err.to_string())?;

    let (pkce_challenge, pkce_verifier) = pkce_pair();
    let (auth_url, csrf_state) = build_authorize_url(&client, pkce_challenge);

    let app_handle = app.clone();
    let storage_handle = storage.clone();
    tauri::async_runtime::spawn(async move {
        let payload =
            match complete_flow(client, port, csrf_state.secret().to_string(), pkce_verifier).await
            {
                Ok(tokens) => match persist_tokens(&storage_handle, &tokens).await {
                    Ok(_) => AuthEventPayload {
                        success: true,
                        message: "Google アカウントに接続しました".into(),
                    },
                    Err(err) => AuthEventPayload {
                        success: false,
                        message: format!("トークン保存に失敗しました: {err}"),
                    },
                },
                Err(err) => AuthEventPayload {
                    success: false,
                    message: format!("OAuth エラー: {err}"),
                },
            };

        let _ = app_handle.emit_all("auth://completed", &payload);
    });

    Ok(AuthStartResponse {
        authorization_url: auth_url.to_string(),
    })
}

#[tauri::command]
pub async fn current_session(app: tauri::AppHandle) -> Result<SessionStatus, String> {
    session_status(&app).await.map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            health_check,
            list_task_lists,
            list_tasks,
            start_oauth,
            current_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
