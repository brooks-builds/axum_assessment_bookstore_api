use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::AppConfig, db::author_queries::insert_author, types::author::CreateAuthorJson,
};

pub async fn create_author(
    state: State<AppConfig>,
    Json(new_author): Json<CreateAuthorJson>,
) -> Result<impl IntoResponse, StatusCode> {
    let author = insert_author(new_author, &state.db)
        .await
        .map_err(|error| {
            tracing::error!("Error inserting author: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok((StatusCode::CREATED, Json(author)))
}
