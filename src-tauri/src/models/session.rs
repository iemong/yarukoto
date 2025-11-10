use oauth2::basic::BasicTokenType;
use oauth2::{EmptyExtraTokenFields, StandardTokenResponse};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<u64>,
    pub token_type: String,
}

impl StoredToken {
    pub fn is_valid(&self) -> bool {
        match self.expires_at {
            Some(exp) => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::ZERO)
                    .as_secs();
                now + 60 < exp
            }
            None => true,
        }
    }
}

impl From<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> for StoredToken {
    fn from(response: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>) -> Self {
        let expires_at = response
            .expires_in()
            .and_then(|delta| SystemTime::now().checked_add(delta))
            .and_then(|instant| instant.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs());

        Self {
            access_token: response.access_token().secret().to_string(),
            refresh_token: response
                .refresh_token()
                .map(|token| token.secret().to_string()),
            expires_at,
            token_type: response.token_type().as_ref().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SessionStatus {
    pub is_authenticated: bool,
}
