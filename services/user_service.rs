use sqlx::{PgPool,Error};
use sqlx::types::Json;
use uuid::Uuid;
use crate::models::user_model::{User,UpdateUser};

pub async fn create_user(pool: &PgPool, payload: User) -> Result<User, Error> {

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        RETURNING *
        "#
    )
    .bind(payload.username)
    .bind(payload.email)
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