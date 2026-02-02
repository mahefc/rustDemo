// Helper module moved from services to utils: response handling and error management
use serde_json::{json, Value, to_value};
use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Response format type with built-in status codes
#[derive(Clone, Copy)]
pub enum ResponseFormat {
    DataFetch,
    DataSingle,
    Created,
    Updated,
    Deleted,
}

impl ResponseFormat {
    fn success_status(&self) -> StatusCode {
        match self {
            ResponseFormat::DataFetch => StatusCode::OK,
            ResponseFormat::DataSingle => StatusCode::OK,
            ResponseFormat::Created => StatusCode::CREATED,
            ResponseFormat::Updated => StatusCode::OK,
            ResponseFormat::Deleted => StatusCode::OK,
        }
    }

    fn error_status(&self) -> StatusCode {
        match self {
            ResponseFormat::DataFetch => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseFormat::DataSingle => StatusCode::NOT_FOUND,
            ResponseFormat::Created => StatusCode::BAD_REQUEST,
            ResponseFormat::Updated => StatusCode::BAD_REQUEST,
            ResponseFormat::Deleted => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn is_wrapped(&self) -> bool {
        matches!(self, ResponseFormat::Created | ResponseFormat::Updated | ResponseFormat::Deleted)
    }
}

pub fn handle_response<T: serde::Serialize>(
    result: Result<T, String>,
    format: ResponseFormat,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    result
        .map(|data| {
            let data_value = to_value(data).unwrap_or(Value::Null);
            let status_code = format.success_status();
            if format.is_wrapped() {
                (
                    status_code,
                    axum::Json(json!({ "status": "success", "data": data_value }))
                )
            } else {
                (status_code, axum::Json(data_value))
            }
        })
        .map_err(|e| (format.error_status(), e))
}

pub fn success_response<T: serde::Serialize>(data: T, status_code: StatusCode) -> (StatusCode, axum::Json<Value>) {
    (
        status_code,
        axum::Json(json!({ "status": "success", "data": data }))
    )
}

pub fn data_response<T: serde::Serialize>(data: T, status_code: StatusCode) -> (StatusCode, axum::Json<T>) {
    (status_code, axum::Json(data))
}

pub fn error_response(message: String, status_code: StatusCode) -> (StatusCode, axum::Json<Value>) {
    (
        status_code,
        axum::Json(json!({ "status": "error", "message": message }))
    )
}

pub fn validate_payload(payload: &Value, required_fields: &[&str]) -> Result<(), String> {
    for field in required_fields {
        if payload.get(field).is_none() {
            return Err(format!("Missing required field: {}", field));
        }
    }
    Ok(())
}
