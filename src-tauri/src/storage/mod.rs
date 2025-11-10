use std::path::PathBuf;

use crate::{AppError, AppResult};

pub struct TokenStorage {
    path: PathBuf,
}

impl TokenStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[allow(dead_code)]
    pub async fn save(&self, _payload: &str) -> AppResult<()> {
        Ok(())
    }
}
