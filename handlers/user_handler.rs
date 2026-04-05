use axum::{extract::{State, Path, rejection::JsonRejection}, Json, http::StatusCode};
use uuid::Uuid;
use crate::{
    AppState,
    utils::middleware::AppError,
    utils::auth::{Claims, generate_token},
    services::user_service,
    models::user_model::{User, UpdateUser, CreateUserRequest, LoginRequest, LoginResponse}
};



pub async fn health() -> String {
    "OK".into()
}


pub async fn create_user(
    State(state): State<AppState>,
    payload: Result<Json<CreateUserRequest>, JsonRejection>
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

pub async fn login(
    State(state): State<AppState>,
    payload: Result<Json<LoginRequest>, JsonRejection>
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    
    let payload = AppError::extract_json(payload)?;

    let user = match user_service::login_user(&state.db, payload).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => return Err(AppError::unauthorized("Invalid username or password")),
        Err(err) => return Err(AppError::internal(err)),
    };

    let claims = Claims::new(user.id, user.username.clone());
    let token = generate_token(&claims, &state.jwt_secret)?;

    let response = LoginResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        token,
    };

    Ok((StatusCode::OK, Json(response)))
}
