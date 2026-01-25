use serde_json::{json, Value};
use axum::{Json, extract::Path};

pub async fn health() -> Json<Value> {
    let status = json!({ "status": "OK" });
    Json(status)
}

pub async fn get_user(Path(id): Path<i32>) -> Json<Value> {

    let users = vec![
        json!({ "id": 1, "name": "Alice" }),
        json!({ "id": 2, "name": "Bob" }),
    ];

    let user = users.iter().find(|u| u["id"] == id).cloned()
        .unwrap_or_else(|| json!({ "error": "User not found" }));

    Json(user)

}