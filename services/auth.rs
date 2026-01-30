use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, Request},
    middleware::Next,
    response::Response,
    body::Body,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

pub struct AuthenticatedUser(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Check for 'decodedtoken' header
        if let Some(decoded_header) = parts.headers.get("decodedtoken") {
            if let Ok(decoded_str) = decoded_header.to_str() {
                if let Ok(claims) = serde_json::from_str::<Claims>(decoded_str) {
                    return Ok(AuthenticatedUser(claims));
                }
            }
        }

        // 2. Fallback to 'Authorization' header
        let auth_header = parts.headers.get("Authorization")
            .or_else(|| parts.headers.get("authorization"));

        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    match decode_token(token) {
                        Ok(claims) => return Ok(AuthenticatedUser(claims)),
                        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
                    }
                }
            }
        }

        Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))
    }
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWTSECRET").unwrap_or_else(|_| "secret".to_string());
    
    let mut validation = Validation::default();
    validation.validate_exp = false;
    validation.validate_aud = false;
    validation.required_spec_claims.remove("exp");

    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    let token_data = decode::<Claims>(
        token,
        &decoding_key,
        &validation,
    )?;
    
    Ok(token_data.claims)
}

pub async fn is_authenticated(
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let (mut parts, body) = req.into_parts();
    
    // Attempt to extract the user from the request parts
    match AuthenticatedUser::from_request_parts(&mut parts, &()).await {
        Ok(AuthenticatedUser(claims)) => {
            // Reconstruct the request and insert the claims into extensions
            let mut req = Request::from_parts(parts, body);
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(e) => Err(e),
    }
}
