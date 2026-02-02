use axum::{
    Json, 
    extract::Path, 
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use serde_json::{json, Value};
use crate::services::user_service;
use crate::utils::{helpers,logger};
use helpers::ResponseFormat;

/// Fetch all users - plain data response
pub async fn get_users() -> Result<impl IntoResponse, (StatusCode, String)> {
    logger::log_info("GET_USERS", "Fetching all users");
    let result = user_service::get_users()
        .await
        .map_err(|e| {
            let err = format!("Failed to fetch users: {}", e);
            logger::log_error("GET_USERS", &err);
            err.to_string()
        });
    
    helpers::handle_response(result, ResponseFormat::DataFetch)
}

/// Create a new user - response with status wrapper
pub async fn create_user(Json(payload): Json<Value>) -> Result<impl IntoResponse, (StatusCode, String)> {
    logger::log_info("CREATE_USER", "Received payload");
    let result = user_service::create_user(&payload).await;
    
    if let Err(ref e) = result {
        logger::log_error("CREATE_USER", e);
    } else {
        logger::log_info("CREATE_USER", "User created successfully");
    }
    
    helpers::handle_response(result, ResponseFormat::Created)
}

/// Update user by ID - response with status wrapper
pub async fn update_user(
    Path(id): Path<Uuid>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    logger::log_info("UPDATE_USER", &format!("User ID: {}, Payload: {:?}", id, payload));
    let result = user_service::update_user_partial(id, &payload).await;
    
    if let Err(ref e) = result {
        logger::log_error("UPDATE_USER", e);
    } else {
        logger::log_info("UPDATE_USER", "Updated successfully");
    }
    
    helpers::handle_response(result, ResponseFormat::Updated)
}

/// Delete user by ID - response with status wrapper
pub async fn delete_user(Path(id): Path<Uuid>) -> Result<impl IntoResponse, (StatusCode, String)> {
    logger::log_info("DELETE_USER", &format!("Deleting user: {}", id));
    let result = user_service::delete_user(id)
        .await
        .map(|_| json!({"message": "User deleted"}));
    
    if let Err(ref e) = result {
        logger::log_error("DELETE_USER", e);
    } else {
        logger::log_info("DELETE_USER", "User deleted successfully");
    }
    
    helpers::handle_response(result, ResponseFormat::Deleted)
}

/// Get single user by ID - plain data response
pub async fn get_user(Path(id): Path<Uuid>) -> Result<impl IntoResponse, (StatusCode, String)> {
    logger::log_info("GET_USER", &format!("Fetching user: {}", id));
    let result = user_service::get_user(id).await;
    
    if let Err(ref e) = result {
        logger::log_error("GET_USER", e);
    }
    
    helpers::handle_response(result, ResponseFormat::DataSingle)
}