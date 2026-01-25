mod service;
mod common;

use axum::{routing::get, Router};
use lambda_http::{run, Error};
use service::{health, get_user, get_external};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/user/:id", get(get_user))
        .route("/external", get(get_external))
        .layer(cors);

    run(app).await
}