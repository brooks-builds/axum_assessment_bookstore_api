use axum::{http::StatusCode, Json};

use crate::types::author::CreateAuthorJson;

pub async fn create_author(
    Json(new_author): Json<CreateAuthorJson>,
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::CREATED)
}
