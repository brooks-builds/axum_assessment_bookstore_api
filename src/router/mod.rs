mod author;

use crate::config::AppConfig;
use author::{create_author, get_all_authors, get_one_author, update_author};
use axum::{
    routing::{get, post, put},
    Router,
};

pub fn create_router(state: AppConfig) -> Router {
    Router::new()
        .route("/authors", post(create_author))
        .route("/authors", get(get_all_authors))
        .route("/authors/:id", get(get_one_author))
        .route("/authors/:id", put(update_author))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
