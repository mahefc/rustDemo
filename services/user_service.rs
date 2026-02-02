use sqlx::{query_as, Error};
use uuid::Uuid;
use serde_json::Value;
use crate::models::User;
use crate::utils::db::get_pool;

pub async fn get_users() -> Result<Vec<User>, Error> {
    let pool = get_pool().await?;
    query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
}

pub async fn create_user(payload: &Value) -> Result<User, String> {
    let user = User::from_payload(payload, None)?;
    let pool = get_pool().await.map_err(|e| e.to_string())?;
    
    query_as::<_, User>(
        r#"INSERT INTO users (id, "firstName", "lastName", "emailAddress") 
        VALUES ($1, $2, $3, $4) 
        RETURNING *"#
    )
    .bind(&user.id)
    .bind(&user.firstName)
    .bind(&user.lastName)
    .bind(&user.emailAddress)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
}

pub async fn update_user_partial(id: Uuid, payload: &Value) -> Result<User, String> {
    let pool = get_pool().await.map_err(|e| format!("DB connection error: {}", e))?;
    
    let allowed_fields = vec!["firstName", "lastName", "emailAddress"];
    let mut updates = Vec::new();
    let mut values = Vec::new();
    let mut param_idx = 2;
    
    if let Some(obj) = payload.as_object() {
        for (key, val) in obj.iter() {
            if allowed_fields.contains(&key.as_str()) {
                if let Some(value_str) = val.as_str() {
                    updates.push(format!("\"{}\"=${}", key, param_idx));
                    values.push(value_str.to_string());
                    param_idx += 1;
                }
            }
        }
    }
    
    if updates.is_empty() {
        return Err("No fields to update".to_string());
    }
    
    let query_str = format!("UPDATE users SET {} WHERE id=$1 RETURNING *", updates.join(", "));
    let mut query = query_as::<_, User>(&query_str).bind(id);
    
    for value in values {
        query = query.bind(value);
    }
    
    query.fetch_one(pool).await.map_err(|e| e.to_string())
}

pub async fn delete_user(id: Uuid) -> Result<(), String> {
    let pool = get_pool().await.map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_user(id: Uuid) -> Result<User, String> {
    let pool = get_pool().await.map_err(|e| e.to_string())?;
    query_as::<_, User>("SELECT * FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
}