use axum::{routing::{get,post,put}, Router,serve};
use tokio::net::TcpListener;
use std::env;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
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
    pub mod common;
}
use crate::handlers::user_handler;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = get_pool().await;
    let state = AppState { db: pool };
    let app = routes(state);
    let port = env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}


pub async fn get_pool() -> PgPool {
    let db_url = env::var("DB_URL").expect("DB_URL must be set");

    PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}

fn routes(state: AppState) -> Router {
    Router::new()
    .route("/", get(user_handler::health))
    .nest("/user", Router::new()
        .route("/", post(user_handler::create_user))
        .route("/", put(user_handler::update_user))
        .route("/{id}", get(user_handler::get_user_by_id))
    )
    .with_state(state)
}
