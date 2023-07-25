use entity::authors;
use eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, TryIntoModel};

use crate::types::author::{Author, CreateAuthorJson};

pub async fn insert_author(
    create_author: CreateAuthorJson,
    db: &DatabaseConnection,
) -> Result<Author> {
    let author = authors::ActiveModel {
        name: sea_orm::ActiveValue::Set(create_author.name),
        ..Default::default()
    };

    let created_author = author.save(db).await?.try_into_model()?;
    Ok(Author {
        id: created_author.id,
        name: created_author.name,
    })
}

pub async fn get_author_by_id(id: i32, db: &DatabaseConnection) -> Result<Option<Author>> {
    Ok(authors::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into))
}
