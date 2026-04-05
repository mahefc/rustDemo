use axum::{
    extract::{rejection::JsonRejection, State},
    http::{Request, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::{time::Instant,fmt,env};
use tracing::{error, info, warn};
use tracing_subscriber;
use sqlx::PgPool;
use crate::utils::auth::{Claims, verify_token, extract_token_from_header};

pub struct AppError(StatusCode, String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl AppError {
    pub fn internal<E: ToString>(e: E) -> Self { Self(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) }
    pub fn internal_result<T, E: ToString>(r: Result<T, E>) -> Result<T, Self> { r.map_err(Self::internal) }
    pub fn bad_request(msg: impl Into<String>) -> Self { Self(StatusCode::BAD_REQUEST, msg.into()) }
    pub fn unauthorized(msg: impl Into<String>) -> Self { Self(StatusCode::UNAUTHORIZED, msg.into()) }
    pub fn missing_authorization() -> Self { Self::unauthorized("Missing Authorization header") }
    pub fn extract_json<T>(p: Result<Json<T>, JsonRejection>) -> Result<T, Self> { p.map_err(Self::from).map(|j| j.0) }
}

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self { Self::bad_request(err.body_text()) }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response { 
        (self.0, Json(serde_json::json!({ "status": "error", "message": self.1 }))).into_response() 
    }
}

pub async fn logger_middleware(req: Request<axum::body::Body>, next: Next) -> Response {
    let (method, uri, start) = (req.method().clone(), req.uri().clone(), Instant::now());
    let resp = next.run(req).await;
    let (status, dur) = (resp.status(), start.elapsed());
    if status.is_server_error() { error!(%method, %uri, %status, duration = ?dur); } 
    else if status.is_client_error() { warn!(%method, %uri, %status, duration = ?dur); }
    else { info!(%method, %uri, %status, duration = ?dur); }
    resp
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt_secret: String,
}

impl AppState {
    pub fn verify_and_extract_token(&self, token: &str) -> Result<Claims, AppError> {
        verify_token(token, &self.jwt_secret)
            .map(|d| d.claims)
            .map_err(|e| AppError::internal(e))
    }

    pub fn extract_token_from_header(&self, auth_header: &str) -> Result<String, AppError> {
        extract_token_from_header(auth_header)
    }
}

pub async fn auth_middleware(State(state): State<AppState>, mut req: Request<axum::body::Body>, next: Next) -> Result<Response, AppError> {
    let auth = req.headers().get(AUTHORIZATION)
    .and_then(|v| v.to_str().ok()).ok_or_else(AppError::missing_authorization)?;
    let token = state.extract_token_from_header(auth)?;
    req.extensions_mut().insert(state.verify_and_extract_token(&token)?);
    Ok(next.run(req).await)
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .compact()
        // .without_time()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();
}

pub async fn get_pool() -> PgPool {
    let db_url = env::var("DB_URL").expect("DB_URL must be set");
    PgPool::connect(&db_url).await.expect("Failed to connect to DB")
}