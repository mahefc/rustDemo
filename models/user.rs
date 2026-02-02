use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(default = "Uuid::new_v4", skip_deserializing)]
    pub id: Uuid,
    pub firstName: String,
    pub lastName: String,
    pub emailAddress: Option<String>,
}

impl User {
    /// Extract user data from dynamic JSON payload
    pub fn from_payload(payload: &Value, id: Option<Uuid>) -> Result<Self, String> {
        Ok(User {
            id: id.unwrap_or_else(Uuid::new_v4),
            firstName: payload
                .get("firstName")
                .and_then(|v| v.as_str())
                .ok_or("Missing or invalid firstName")?
                .to_string(),
            lastName: payload
                .get("lastName")
                .and_then(|v| v.as_str())
                .ok_or("Missing or invalid lastName")?
                .to_string(),
            emailAddress: payload
                .get("emailAddress")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
    }
}
