use reqwest::{Method, header::HeaderMap};
use axum::{Json, response::{IntoResponse, Response},http::{StatusCode}};
use serde_json::Value;

pub enum ExternalResponse {
    Json(Value),
    Text(String),
    Buffer(Vec<u8>)
}

pub async fn external_request(
    method: Method,
    url: &str,
    headers: Option<HeaderMap>,
    body: Option<Value>,
) -> Result<ExternalResponse, StatusCode> {
    let client = reqwest::Client::new();
    let mut request = client.request(method, url);

    if let Some(h) = headers { request = request.headers(h); }

    if let Some(b) = body { request = request.json(&b); }

    let response = request.send().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    // Inspect Content-Type header
    let content_type = response.headers().get(reqwest::header::CONTENT_TYPE)
                               .and_then(|ct| ct.to_str().ok()).unwrap_or("");

    if content_type.contains("application/json") {
        let json = response.json::<Value>().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
        Ok(ExternalResponse::Json(json))
    } else if content_type.contains("text/") {
        let text = response.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
        Ok(ExternalResponse::Text(text))
    } else {
        let bytes = response.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
        Ok(ExternalResponse::Buffer(bytes.to_vec()))
    }
}



impl IntoResponse for ExternalResponse {
    fn into_response(self) -> Response {
        match self {
            ExternalResponse::Json(val) => Json(val).into_response(),
            ExternalResponse::Text(text) => text.into_response(),
            ExternalResponse::Buffer(buf) => buf.into_response(),
        }
    }
}



