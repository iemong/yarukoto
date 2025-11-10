use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use portpicker::pick_unused_port;
use serde::Deserialize;
use tauri::AppHandle;
use tokio::sync::{oneshot, Mutex};

use crate::models::StoredToken;
use crate::storage::TokenStorage;
use crate::{AppError, AppResult};

const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const TASKS_SCOPE: &str = "https://www.googleapis.com/auth/tasks";
const CALLBACK_PATH: &str = "/callback";

pub fn pkce_pair() -> (PkceCodeChallenge, PkceCodeVerifier) {
    PkceCodeChallenge::new_random_sha256()
}

#[derive(Clone)]
struct CallbackState {
    sender: Arc<Mutex<Option<oneshot::Sender<AuthCallback>>>>,
}

#[derive(Debug, Deserialize)]
struct AuthCallback {
    code: String,
    state: String,
}

pub fn pick_port() -> AppResult<u16> {
    pick_unused_port().ok_or_else(|| AppError::OAuth("no available loopback port".into()))
}

pub fn redirect_url(port: u16) -> String {
    format!("http://127.0.0.1:{port}{CALLBACK_PATH}")
}

pub fn build_client(redirect_url: &str) -> AppResult<BasicClient> {
    let client_id =
        std::env::var("CLIENT_ID").map_err(|_| AppError::Config("CLIENT_ID is not set".into()))?;
    let client_secret = std::env::var("CLIENT_SECRET")
        .map_err(|_| AppError::Config("CLIENT_SECRET is not set".into()))?;

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(AUTH_URL.to_string()).map_err(|err| AppError::OAuth(err.to_string()))?,
        Some(TokenUrl::new(TOKEN_URL.to_string()).map_err(|err| AppError::OAuth(err.to_string()))?),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url.to_string())
            .map_err(|err| AppError::OAuth(err.to_string()))?,
    );

    Ok(client)
}

pub fn build_authorize_url(
    client: &BasicClient,
    challenge: PkceCodeChallenge,
) -> (url::Url, CsrfToken) {
    client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(TASKS_SCOPE.to_string()))
        .set_pkce_challenge(challenge)
        .url()
}

pub async fn complete_flow(
    client: BasicClient,
    port: u16,
    expected_state: String,
    pkce_verifier: PkceCodeVerifier,
) -> AppResult<StoredToken> {
    let callback = wait_for_callback(port).await?;

    if callback.state != expected_state {
        return Err(AppError::OAuth("state mismatch".into()));
    }

    let token_response = client
        .exchange_code(AuthorizationCode::new(callback.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|err| AppError::OAuth(format!("token exchange failed: {err}")))?;

    Ok(StoredToken::from(token_response))
}

async fn wait_for_callback(port: u16) -> AppResult<AuthCallback> {
    let (auth_tx, auth_rx) = oneshot::channel();
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let state = CallbackState {
        sender: Arc::new(Mutex::new(Some(auth_tx))),
    };

    let router = Router::new()
        .route(CALLBACK_PATH, get(callback_handler))
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server = axum::Server::bind(&addr).serve(router.into_make_service());

    tauri::async_runtime::spawn(async move {
        let _ = server
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .await;
    });

    let callback = auth_rx
        .await
        .map_err(|_| AppError::OAuth("authorization channel closed".into()))?;
    let _ = shutdown_tx.send(());
    Ok(callback)
}

async fn callback_handler(
    State(state): State<CallbackState>,
    Query(query): Query<AuthCallback>,
) -> Html<&'static str> {
    if let Some(sender) = state.sender.lock().await.take() {
        let _ = sender.send(query);
    }

    Html("<h1>認証が完了しました</h1><p>アプリに戻ってログイン状態を確認してください。</p>")
}

pub async fn persist_tokens(storage: &TokenStorage, tokens: &StoredToken) -> AppResult<()> {
    storage.save(tokens).await
}

pub async fn session_status(app: &AppHandle) -> AppResult<crate::models::SessionStatus> {
    let storage = TokenStorage::new(app)?;
    storage.session_status().await
}
