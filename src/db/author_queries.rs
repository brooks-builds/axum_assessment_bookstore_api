use crate::models::author::{Author, CreateAuthor, AtomicUpdateAuthor};
use entity::authors::Entity as AuthorEntity;
use entity::books::Entity as BookEntity;
use eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TryIntoModel, IntoActiveModel, ModelTrait};

pub async fn insert_author(db: &DatabaseConnection, author: CreateAuthor) -> Result<Author> {
    let mut new_author = <entity::authors::ActiveModel as std::default::Default>::default();

    new_author.name = Set(author.name);

    let created_author = new_author.save(db).await?.try_into_model()?.into();

    Ok(created_author)
}

pub async fn get_author_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Author>> {
    let authors_with_books = AuthorEntity::find_by_id(id)
        .find_with_related(BookEntity)
        .all(db)
        .await?;
    let Some(author_with_books)= authors_with_books.first() else {
        return Ok(None);
    };

    Ok(Some(author_with_books.into()))
}

pub async fn get_all_authors(db: &DatabaseConnection) -> Result<Vec<Author>> {
    let authors = AuthorEntity::find()
        .find_with_related(BookEntity)
        .all(db)
        .await?
        .iter()
        .map(Into::into)
        .collect::<Vec<Author>>();

    Ok(authors)
}

pub async fn update_author(db: &DatabaseConnection, id: i32, author: AtomicUpdateAuthor) -> Result<()> {
    let Some(db_author) = AuthorEntity::find_by_id(id).one(db).await? else {
        return Ok(()); 
    };
    let mut db_author = db_author.into_active_model();

    db_author.name = Set(author.name);
    db_author.save(db).await?;

    Ok(())
}

pub async fn delete_author(db: &DatabaseConnection, id: i32) -> Result<()> {
    let Some(author) = AuthorEntity::find_by_id(id).one(db).await? else {
        return Ok(());
    };
    
    author.delete(db).await?;
    
    Ok(())
}
