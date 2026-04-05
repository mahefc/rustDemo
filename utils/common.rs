use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub struct AppError {
    pub status: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn internal<E: ToString>(error: E) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }

    pub fn internal_result<T, E: ToString>(result: Result<T, E>) -> Result<T, Self> {
        result.map_err(Self::internal)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn extract_json<T>(payload: Result<Json<T>, JsonRejection>) -> Result<T, Self> {
        payload.map_err(Self::from).map(|json| json.0)
    }
}

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self {
        AppError::bad_request(err.body_text())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        let body = Json(json!({
            "status": "error",
            "message": self.message,
        }));
        println!("AppError: {}", self.message);

        (self.status, body).into_response()
    }
}
