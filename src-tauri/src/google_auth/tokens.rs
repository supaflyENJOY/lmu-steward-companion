//! Token storage and refresh logic for Google OAuth2.

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, EmptyExtraTokenFields, RefreshToken,
    StandardTokenResponse, TokenResponse,
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<u64>, // unix timestamp
}

impl StoredToken {
    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now + 60 > exp // 60s buffer
        } else {
            false
        }
    }
}

/// TokenStorage now uses PathBuf for better path handling and supports app data dir
pub struct TokenStorage {
    path: PathBuf,
}

impl TokenStorage {
    /// Create a new TokenStorage with a given path
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }


    pub fn load(&self) -> Result<Option<StoredToken>, Box<dyn Error + Send + Sync>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let data = fs::read_to_string(&self.path)?;
        let token: StoredToken = serde_json::from_str(&data)?;
        Ok(Some(token))
    }

    pub fn save(&self, token: &StoredToken) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(parent) = self.path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create token directory: {e}"))?;
            }
        }
        let data = serde_json::to_string(token)?;
        let mut file = fs::File::create(&self.path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn save_from_response(
        &mut self,
        resp: &StandardTokenResponse<EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    ) -> Result<StoredToken, Box<dyn Error + Send + Sync>> {
        let expires_at = resp.expires_in().map(|dur| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now + dur.as_secs()
        });
        let token = StoredToken {
            access_token: resp.access_token().secret().to_string(),
            refresh_token: resp.refresh_token().map(|r| r.secret().to_string()),
            expires_at,
        };
        self.save(&token)?;
        Ok(token)
    }

    pub async fn refresh(
        &mut self,
        client: &BasicClient,
        refresh_token: &str,
    ) -> Result<Option<StoredToken>, Box<dyn Error + Send + Sync>> {
        let token_result = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
            .request_async(async_http_client)
            .await?;
        let stored = self.save_from_response(&token_result)?;
        Ok(Some(stored))
    }
}
