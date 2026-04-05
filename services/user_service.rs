use sqlx::{PgPool, Error,types::Json};
use uuid::Uuid;
use crate::{
    models::user_model::{User, UpdateUser, CreateUserRequest, LoginRequest},
    utils::auth::{hash_password, verify_password}
};

pub async fn create_user(pool: &PgPool, payload: CreateUserRequest) -> Result<User, Error> {
    let hashed_password = hash_password(&payload.password)
        .map_err(|_| Error::RowNotFound)?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
    .bind(payload.username)
    .bind(payload.email)
    .bind(hashed_password)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE id = $1
        "#
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_user(pool: &PgPool, payload: UpdateUser) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET username = COALESCE($1->>'username', username),
        updated_at = NOW()
        WHERE id = COALESCE(($1->>'id')::uuid, id)
        RETURNING *
        "#
    )
    .bind(Json(payload)) 
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn login_user(pool: &PgPool, payload: LoginRequest) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE username = $1
        "#
    )
    .bind(payload.username)
    .fetch_one(pool)
    .await?;

    // Verify password
    let valid_password = verify_password(&payload.password, &user.password)
        .map_err(|_| Error::RowNotFound)?;

    if !valid_password {
        return Err(Error::RowNotFound);
    }

    Ok(user)
}
