use crate::{
    models::{Task, TaskList},
    AppError, AppResult,
};

pub async fn mock_task_lists() -> AppResult<Vec<TaskList>> {
    Ok(vec![
        TaskList {
            id: "inbox".into(),
            title: "Inbox".into(),
            task_count: 4,
        },
        TaskList {
            id: "today".into(),
            title: "Today".into(),
            task_count: 2,
        },
    ])
}

pub async fn mock_tasks() -> AppResult<Vec<Task>> {
    Ok(vec![
        Task {
            id: "task-1".into(),
            title: "Rust 側の OAuth 設計".into(),
            completed: false,
            due: None,
            notes: Some("oauth フローの PoC".into()),
        },
        Task {
            id: "task-2".into(),
            title: "TanStack Query ストア設計".into(),
            completed: true,
            due: None,
            notes: None,
        },
    ])
}

#[allow(dead_code)]
pub async fn sync_with_google() -> AppResult<()> {
    Err(AppError::Unknown)
}
