use crate::common::{external_request, ExternalResponse};
use axum::{Json, extract::Path};
use serde_json::{json, Value};
use reqwest::{Method, header::HeaderMap};
use axum::http::StatusCode;

pub async fn health() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "status": "OK" })))
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

pub async fn get_external() -> Result<ExternalResponse, StatusCode> {
    // Example headers
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "Bearer mytoken".parse().unwrap());

    // Example body
    let body = json!({
        "query": "example",
        "limit": 5
    });

    external_request(
        Method::POST, // using POST to send body
        "https://jsonplaceholder.typicode.com/posts",
        None,
        Some(body),
    ).await
}
