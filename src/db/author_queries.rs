use eyre::Result;
use sea_orm::ActiveModelTrait;

use crate::types::author::{CreateAuthorJson, Author};

pub async fn insert_author(create_author: CreateAuthorJson) -> Result<Author> {
    let mut new_author = entity::authors::ActiveModel::default();
    new_author.name = create_author.name;
    let created_author = new_author.save()
    todo!()
}