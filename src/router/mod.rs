mod author;

use author::create_author;
use axum::{routing::post, Router};

use crate::config::AppConfig;

pub fn create_router(state: AppConfig) -> Router {
    Router::new()
        .route("/authors", post(create_author))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
