use axum::{Json, extract::Path, http::StatusCode};
use uuid::Uuid;
use crate::models::User;
use crate::services::user_service;

pub async fn get_users() -> Result<Json<Vec<User>>, StatusCode> {
    match user_service::get_users().await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_user(Json(user): Json<User>) -> Result<Json<User>, StatusCode> {
    match user_service::create_user(user).await {
        Ok(created) => Ok(Json(created)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_user(Json(user): Json<User>) -> Result<Json<User>, StatusCode> {
    match user_service::update_user(user).await {
        Ok(updated) => Ok(Json(updated)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_user(Path(id): Path<Uuid>) -> Result<Json<String>, StatusCode> {
    match user_service::delete_user(id).await {
        Ok(_) => Ok(Json("User deleted".to_string())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_user(Path(id): Path<Uuid>) -> Result<Json<User>, StatusCode> {
    match user_service::get_user(id).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn health() -> Json<String> {
    Json("OK".to_string())
}
