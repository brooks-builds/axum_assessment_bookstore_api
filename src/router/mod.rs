mod author;

use author::create_author;
use axum::{routing::post, Router};

pub fn create_router() -> Router {
    Router::new().route("/authors", post(create_author))
}
