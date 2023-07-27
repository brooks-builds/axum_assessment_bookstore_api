use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    config::AppConfig,
    db::author_queries::{get_author_by_id, insert_author},
    types::{
        author::{Author, CreateAuthorJson},
        ResponseObject,
    },
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

pub async fn get_one_author(
    state: State<AppConfig>,
    Path(id): Path<i32>,
) -> Result<Json<ResponseObject<Author>>, Json<ResponseObject<()>>> {
    let author = match get_author_by_id(id, &state.db).await {
        Ok(author) => author,
        Err(error) => {
            tracing::error!("Error getting author by id: {error}");
            return Err(Json(ResponseObject::new_internal_error(
                "There was an error getting the author",
            )));
        }
    };

    Ok(Json(ResponseObject::new_ok(author)))
}
