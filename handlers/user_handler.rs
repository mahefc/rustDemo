use axum::{extract::{State, Path, rejection::JsonRejection}, Json, http::StatusCode};
use crate::models::user_model::{User,UpdateUser};
use crate::services::user_service;
use uuid::Uuid;
use crate::AppState;
use crate::utils::common::AppError;


pub async fn health() -> String {
    "OK".into()
}


pub async fn create_user(
    State(state): State<AppState>,
    payload: Result<Json<User>, JsonRejection>
) -> Result<(StatusCode, Json<User>), AppError> {
    
    let payload = AppError::extract_json(payload)?;

    let user = AppError::internal_result(user_service::create_user(&state.db, payload).await)?;

    Ok((StatusCode::CREATED, Json(user)))

}


pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<(StatusCode, Json<User>), AppError>  {
    let user = AppError::internal_result(
        user_service::get_user_by_id(&state.db, user_id).await
    )?;

    Ok((StatusCode::OK, Json(user)))
}


pub async fn update_user(
    State(state): State<AppState>,
    payload: Result<Json<UpdateUser>, JsonRejection>
) -> Result<(StatusCode, Json<User>), AppError> {

    let payload = AppError::extract_json(payload)?;

    let user = AppError::internal_result(user_service::update_user(&state.db, payload).await)?;

    Ok((StatusCode::OK, Json(user)))
}
