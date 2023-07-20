use axum::http::StatusCode;

pub async fn create_author() -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::CREATED)
}
