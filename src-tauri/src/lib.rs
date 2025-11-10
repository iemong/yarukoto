mod google_tasks;
mod models;
mod oauth;
mod storage;

use google_tasks::{mock_task_lists, mock_tasks};
use models::{Task, TaskList};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("network error: {0}")]
    Network(String),
    #[error("storage error: {0}")]
    Storage(String),
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            health_check,
            list_task_lists,
            list_tasks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
