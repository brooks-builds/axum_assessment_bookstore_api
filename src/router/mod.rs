mod author;

use author::{create_author, get_one_author};
use axum::{
    routing::{get, post},
    Router,
};

use crate::config::AppConfig;

pub fn create_router(state: AppConfig) -> Router {
    Router::new()
        .route("/authors", post(create_author))
        .route("/authors/:id", get(get_one_author))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
