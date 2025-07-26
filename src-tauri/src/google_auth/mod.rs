//! Google OAuth2 authentication and token management for desktop Tauri app.

mod tokens;
pub use tokens::TokenStorage;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::AppHandle;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use url::Url;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const OAUTH2_SCOPES: [&str; 2] = [
    "https://www.googleapis.com/auth/spreadsheets",
    "https://www.googleapis.com/auth/drive.file",
];

/// Holds the OAuth2 client and token storage.
pub struct GoogleAuth {
    client: BasicClient,
    token_storage: Arc<Mutex<TokenStorage>>,
}

impl GoogleAuth {
    /// New PKCE-based constructor: now includes client_secret for Google OAuth2
    pub fn new(client_id: &str, client_secret: &str, redirect_url: &str, token_path: &str) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new(GOOGLE_AUTH_URL.to_string()).unwrap(),
            Some(TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).unwrap());

        let token_storage = Arc::new(Mutex::new(TokenStorage::new(token_path)));
        Self {
            client,
            token_storage,
        }
    }

    /// Starts the PKCE browser-based OAuth2 flow, launches the browser, and captures the code via a local HTTP server.
    /// Returns the access token on success.
    pub async fn authenticate_via_browser(
        &self,
        _app_handle: AppHandle,
        timeout_secs: u64,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {

        // 1. Pick a random available port and bind listener
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        let redirect_uri = format!("http://localhost:{}/callback", port);

        // 2. Generate PKCE code_verifier using oauth2 crate
        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

        // 3. Build OAuth2 client with dynamic redirect URI (use self.client's client_id)
        let client = self
            .client
            .clone()
            .set_redirect_uri(RedirectUrl::new(redirect_uri.clone()).unwrap());

        // 4. Build auth URL with PKCE params
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(OAUTH2_SCOPES.iter().map(|s| Scope::new(s.to_string())))
            .set_pkce_challenge(pkce_challenge)
            .url();

        // 5. Start server task to handle callbacks
        let server_listener = Arc::new(listener);
        let server_port = port;
        let callback_receiver = {
            let listener = server_listener.clone();
            tokio::spawn(async move {
                loop {
                    match listener.accept().await {
                        Ok((mut stream, _addr)) => {

                            // Read HTTP request with proper buffering
                            let mut buf = vec![0u8; 4096];
                            match tokio::time::timeout(Duration::from_secs(5), async {
                                use tokio::io::AsyncReadExt;
                                stream.read(&mut buf).await
                            })
                            .await
                            {
                                Ok(Ok(n)) if n > 0 => {
                                    let req = String::from_utf8_lossy(&buf[..n]);

                                    // Parse GET /callback?code=...
                                    if req.starts_with("GET /callback?") {
                                        if let Some(line) = req.lines().next() {
                                            if let Some(url_start) = line.find("/callback?") {
                                                // Extract URL path and query string properly
                                                let url_part = &line[url_start..];
                                                let url_end =
                                                    url_part.find(' ').unwrap_or(url_part.len());
                                                let url = format!(
                                                    "http://localhost:{}{}",
                                                    server_port,
                                                    &url_part[..url_end]
                                                );

                                                if let Ok(parsed) = Url::parse(&url) {
                                                    if let Some(code) = parsed
                                                        .query_pairs()
                                                        .find(|(k, _)| k == "code")
                                                        .map(|(_, v)| v.to_string())
                                                    {
                                                        // Respond to browser
                                                        let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Authentication Complete</h1><p>You may close this window.</p></body></html>").await;
                                                        return Ok(code);
                                                    } else {
                                                    }
                                                } else {
                                                }
                                            }
                                        }
                                    } else {
                                    }

                                    // Respond with error for invalid requests
                                    let _ = stream.write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Invalid Request</h1><p>Expected OAuth2 callback.</p></body></html>").await;
                                }
                                Ok(Ok(_)) => {
                                }
                                Ok(Err(_e)) => {
                                }
                                Err(_) => {
                                }
                            }
                        }
                        Err(e) => {
                            return Err(format!("Server accept error: {}", e).into());
                        }
                    }
                }
            })
        };

        // 6. Wait a moment to ensure server is ready
        tokio::time::sleep(Duration::from_millis(100)).await;

        // 7. Open browser
        if let Err(e) = tauri_plugin_opener::open_url(&auth_url.to_string(), None::<&str>) {
            return Err(format!("Failed to open browser: {e}").into());
        }

        // 8. Wait for callback with timeout
        let _start = Instant::now();

        let code = tokio::select! {
            result = callback_receiver => {
                match result {
                    Ok(Ok(code)) => {
                        code
                    }
                    Ok(Err(e)) => {
                        return Err(e);
                    }
                    Err(e) => {
                        return Err(format!("Server task failed: {}", e).into());
                    }
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(timeout_secs)) => {
                return Err("OAuth2 authentication timed out".into());
            }
        };

        // 7. Exchange code for tokens using PKCE (code_verifier, no client_secret)

        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .add_extra_param("redirect_uri", &redirect_uri)
            .request_async(async_http_client)
            .await
            .map_err(|e| {
                format!("Google OAuth2 token exchange failed: {:?}", e)
            })?;


        // 8. Store tokens
        let mut storage = self.token_storage.lock().await;
        let stored = storage.save_from_response(&token_result)?;

        Ok(stored.access_token)
    }

    /// Gets a valid access token, refreshing or initiating OAuth2 PKCE flow as needed.
    pub async fn get_access_token(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut storage = self.token_storage.lock().await;
        if let Some(token) = storage.load()? {
            if !token.is_expired() {
                return Ok(token.access_token.clone());
            }
            // Try to refresh
            if let Some(refresh_token) = &token.refresh_token {
                if let Some(new_token) = storage.refresh(&self.client, refresh_token).await? {
                    return Ok(new_token.access_token.clone());
                }
            }
        }
        // No valid token, start PKCE OAuth2 flow
        // For desktop, this should call authenticate_via_browser and store the result
        // (This is a placeholder; actual integration may differ)
        Err("No valid token and PKCE flow must be initiated via authenticate_via_browser".into())
    }
}
