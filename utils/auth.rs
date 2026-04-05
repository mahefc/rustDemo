use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::middleware::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|err| AppError::internal(format!("Failed to hash password: {err}")))
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, AppError> {
    verify(password, hashed)
        .map_err(|err| AppError::internal(format!("Failed to verify password: {err}")))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, username: String) -> Self {
        let now = Utc::now();
        Self {
            user_id: user_id.to_string(),
            username,
            exp: (now + Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn generate_token(claims: &Claims, secret: &str) -> Result<String, AppError> {
    encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|e| AppError::internal(format!("Token generation failed: {}", e)))
}

pub fn verify_token(token: &str, secret: &str) -> Result<TokenData<Claims>, AppError> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map_err(|e| AppError::unauthorized(format!("Invalid token: {}", e)))
}

pub fn extract_token_from_header(auth_header: &str) -> Result<String, AppError> {
    auth_header.strip_prefix("Bearer ").map(String::from).ok_or_else(|| AppError::unauthorized("Missing or invalid Authorization header"))
}
