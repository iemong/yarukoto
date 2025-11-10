use crate::{AppError, AppResult};

pub struct OAuthClient;

impl OAuthClient {
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    pub async fn begin_authorization(&self) -> AppResult<()> {
        Ok(())
    }
}
