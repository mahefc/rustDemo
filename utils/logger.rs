use serde_json::json;

/// Log error with details
pub fn log_error(context: &str, error: &str) {
    eprintln!("[ERROR] {} - {}", context, error);
}

/// Log info
pub fn log_info(context: &str, message: &str) {
    eprintln!("[INFO] {} - {}", context, message);
}

/// Create error response
pub fn error_response(error: &str) -> serde_json::Value {
    json!({
        "status": "error",
        "message": error,
        "timestamp": chrono::Local::now().to_rfc3339()
    })
}

/// Log validation error
pub fn log_validation_error(field: &str, reason: &str) {
    eprintln!("[VALIDATION ERROR] Field '{}': {}", field, reason);
}
