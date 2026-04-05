use axum::{
    Router,
    middleware::from_fn,
    middleware::from_fn_with_state,
    routing::{get, post, put},
};
use std::sync::Arc;

mod utils {
    pub mod auth;
    pub mod common;
    pub mod state;
}

mod repos {
    pub mod folder_r;
}

mod services {
    pub mod folder_s;
}

mod handlers {
    pub mod folder_h;
}

use utils::auth::require_auth;
use utils::common::logger;
use utils::state::AppState;

use handlers::folder_h;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let state = AppState::new().await;
    let app = app(state);

    lambda_http::run(app).await.expect("Lambda runtime failed");
}

fn app(state: Arc<AppState>) -> Router {
    let auth = from_fn_with_state(state.clone(), require_auth);

    let public = Router::new().route("/health", get(|| async { "Server is UP" }));

    let protected = Router::new()
        // Folders
        .route("/folder/create", post(folder_h::create_folder))
        .route("/folder/update", put(folder_h::update_folder))
        .route("/folder/getAll/:tenantId", get(folder_h::get_all_folders))
        .route("/folder/getAll/:tenantId/workspace/:workspaceId", get(folder_h::get_all_folders))
        .route("/folder/template/:templateId",get(folder_h::get_folders_by_template_id))
        .route("/folder/template/:templateId/parentFolder",get(folder_h::get_folder_by_template_id))
        .route("/folder/:projectId/exists",get(folder_h::folder_exists_in_project))
        .route("/folder/:id",get(folder_h::get_folder_by_id).delete(folder_h::delete_folder))
        .layer(auth);

    Router::new()
        .merge(public)
        .merge(protected)
        .layer(from_fn(logger))
        .with_state(state)
}
