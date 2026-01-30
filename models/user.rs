use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub firstName: String,
    pub lastName: String,
    pub emailAddress: String,
}