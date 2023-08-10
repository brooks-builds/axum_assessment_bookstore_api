use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    config::AppConfig,
    db::author_queries,
    models::author::{Author, AuthorResponse},
};

use super::ErrorResponse;

pub async fn create_author(
    state: State<AppConfig>,
    Json(new_author): Json<Author>,
) -> Result<(StatusCode, Json<AuthorResponse>), ErrorResponse> {
    let create_author = new_author.try_into().map_err(|error| {
        tracing::error!("Error creating author: {error}");
        (StatusCode::BAD_REQUEST, format!("{error}"))
    })?;
    let created_author = author_queries::insert_author(&state.db, create_author)
        .await
        .map_err(|error| {
            tracing::error!("Error inserting author into database: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error creating the author".to_owned(),
            )
        })?;
    let response = created_author.try_into().map_err(|error| {
        tracing::error!("Error converting created author to response: {error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error creating the author".to_owned(),
        )
    })?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_one_author(
    state: State<AppConfig>,
    Path(id): Path<i32>,
) -> Result<Json<AuthorResponse>, ErrorResponse> {
    let author = author_queries::get_author_by_id(&state.db, id)
        .await
        .map_err(|error| {
            tracing::error!("Error getting author from db: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting the author".to_owned(),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Author not found".to_owned()))?
        .try_into()
        .map_err(|error| {
            tracing::error!("Error converting db author to response author: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting the author".to_owned(),
            )
        })?;

    Ok(Json(author))
}

pub async fn get_all_authors(
    state: State<AppConfig>,
) -> Result<Json<Vec<AuthorResponse>>, ErrorResponse> {
    let authors = author_queries::get_all_authors(&state.db)
        .await
        .map_err(|error| {
            tracing::error!("Error getting all authors: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting all authors".to_owned(),
            )
        })?
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<AuthorResponse>, _>>()
        .map_err(|error| {
            tracing::error!("Error converting get all author into response author: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting all authors".to_owned(),
            )
        })?;

    Ok(Json(authors))
}

pub async fn update_author(
    state: State<AppConfig>,
    Path(id): Path<i32>,
    Json(author): Json<Author>,
) -> Result<StatusCode, ErrorResponse> {
    let author = author.try_into().map_err(|error| {
        tracing::error!("Error converting request author to atomic update author: {error}");
        (StatusCode::BAD_REQUEST, format!("{error}"))
    })?;

    author_queries::update_author(&state.db, id, author)
        .await
        .map_err(|error| {
            tracing::error!("Error updating author: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error updating the author".to_owned(),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_author(
    state: State<AppConfig>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ErrorResponse> {
    author_queries::delete_author(&state.db, id)
        .await
        .map_err(|error| {
            tracing::error!("Error deleting author: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error deleting the author".to_owned(),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
