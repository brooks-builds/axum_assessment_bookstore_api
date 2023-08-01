use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    config::AppConfig,
    db::{
        author_queries::{self, get_author_by_id, insert_author},
        book_queries,
    },
    models::{
        author::{Author, CreateAuthorJson},
        EmptyResponse, ResponseObject,
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
    Ok((
        StatusCode::CREATED,
        Json(ResponseObject::new_created(author)),
    ))
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

pub async fn get_all_authors(
    state: State<AppConfig>,
) -> Result<Json<ResponseObject<Vec<Author>>>, Json<ResponseObject<()>>> {
    match author_queries::get_all_authors(&state.db).await {
        Ok(authors) => Ok(Json(ResponseObject::new_ok(Some(authors)))),
        Err(error) => {
            tracing::error!("Error getting all authors: {error}");
            Err(Json(ResponseObject::new_internal_error(
                "there was an error getting all authors",
            )))
        }
    }
}

pub async fn update_author(
    state: State<AppConfig>,
    Path(id): Path<i32>,
    Json(update_author): Json<CreateAuthorJson>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseObject<EmptyResponse>>)> {
    match author_queries::update_author(&state.db, id, update_author.name).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(error) => {
            if error.root_cause().to_string() == "not_found" {
                Err((StatusCode::OK, Json(ResponseObject::new_ok(None))))
            } else {
                tracing::error!("error updating author: {error}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseObject::new_internal_error("Error updating author")),
                ))
            }
        }
    }
}

pub async fn delete_author(
    state: State<AppConfig>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseObject<EmptyResponse>>)> {
    if let Err(error) = author_queries::delete_author(&state.db, id).await {
        tracing::error!("Error deleting author: {error}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseObject::new_internal_error(
                "There was an error deleting the author",
            )),
        ));
    }

    let books_without_authors = book_queries::get_books_without_authors(&state.db)
        .await
        .map_err(|error| {
            tracing::error!("Error getting books without authors after deleting author: {error}");
            (
                StatusCode::OK,
                Json(ResponseObject::new_ok(Some(EmptyResponse))),
            )
        })?;

    book_queries::delete_many(&state.db, books_without_authors)
        .await
        .map_err(|error| {
            tracing::error!("Error deleting books without authors: {error}");
            (
                StatusCode::OK,
                Json(ResponseObject::new_ok(Some(EmptyResponse))),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
