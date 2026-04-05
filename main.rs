use axum::{routing::{get, post, put}, Router, serve, middleware::{from_fn_with_state, from_fn}};
use tokio::net::TcpListener;
use std::env;

mod handlers {
    pub mod user_handler;
}
mod models {
    pub mod user_model;
}
mod services {
    pub mod user_service;
}
mod utils {
    pub mod auth;
    pub mod middleware;
}

use crate::{
    utils::middleware::{
        AppState, auth_middleware, logger_middleware, init_tracing, get_pool
    },
    handlers::user_handler
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    
    let state = AppState { 
        db: get_pool().await, 
        jwt_secret: env::var("JWT_SECRET").unwrap_or("$piderM@n".into()),
    };
    
    let port = env::var("PORT").unwrap_or("3000".into());
    let listener = TcpListener::bind(&format!("0.0.0.0:{port}")).await.unwrap();
    serve(listener, routes(state)).await.unwrap();
}

fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(user_handler::health))
        .route("/auth/login", post(user_handler::login))
        .nest("/user", Router::new()
            .route("/", post(user_handler::create_user))
            .route("/", put(user_handler::update_user))
            .route("/{id}", get(user_handler::get_user_by_id))
            .layer(from_fn_with_state(state.clone(), auth_middleware))
        )
        .layer(from_fn(logger_middleware))
        .with_state(state)
}