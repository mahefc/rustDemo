use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use validator::{Validate};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Validate, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    
    #[validate(length(min = 3, message="Username must be at least 3 characters"))]
    pub username: String,

    #[validate(email)]
    pub email: String,
    
    #[serde(skip_serializing)]
    pub password: String,
    
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    
    #[serde(default)] 
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub token: String,
}
