#![allow(non_snake_case)]

mod services;
mod models;
mod controllers;
use axum::{
    routing::{get, post, put, delete}, 
    Router, 
    middleware::{from_fn, Next},
    http::{Request},
    response::Response,
    body::Body,
};
use services::db::get_pool;
use lambda_http::{run, Error};
use controllers::user_ctrl;
use services::auth;
use tower_http::cors::{CorsLayer, Any};


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    get_pool().await.expect("Failed to connect to database");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let user_routes = Router::new()
        .route("/", get(user_ctrl::get_users))
        .route("/", post(user_ctrl::create_user))
        .route("/", put(user_ctrl::update_user))
        .route("/:id", get(user_ctrl::get_user))
        .route("/:id", delete(user_ctrl::delete_user));

    let app = Router::new()
        .nest("/users", user_routes)
        .layer(from_fn(auth::is_authenticated))
        .layer(cors)
        .layer(from_fn(logging_middleware));

    run(app).await
}


async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let response = next.run(req).await;
    let status = response.status().as_u16();

    eprintln!("Info: '{}' - {}", path, status);

    response
}