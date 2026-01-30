use sqlx::{query_as,Error};
use uuid::Uuid;
use crate::models::User;
use crate::services::db::get_pool;

pub async fn get_users() -> Result<Vec<User>, Error> {
    let pool = get_pool().await?;
    query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
}

pub async fn create_user(user: User) -> Result<User, Error> {
    let pool = get_pool().await?;
    query_as::<_, User>(
        "INSERT INTO users (id, firstName, lastName, emailAddress) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(user.id)
    .bind(user.firstName)
    .bind(user.lastName)
    .bind(user.emailAddress)
    .fetch_one(pool)
    .await
}

pub async fn update_user(user: User) -> Result<User, Error> {
    let pool = get_pool().await?;
    query_as::<_, User>(
        "UPDATE users SET firstName=$2, lastName=$3, emailAddress=$4 WHERE id=$1 RETURNING *"
    )
    .bind(user.id)
    .bind(user.firstName)
    .bind(user.lastName)
    .bind(user.emailAddress)
    .fetch_one(pool)
    .await
}

pub async fn delete_user(id: Uuid) -> Result<(), Error> {
    let pool = get_pool().await?;
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user(id: Uuid) -> Result<User, Error> {
    let pool = get_pool().await?;
    query_as::<_, User>("SELECT * FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
}