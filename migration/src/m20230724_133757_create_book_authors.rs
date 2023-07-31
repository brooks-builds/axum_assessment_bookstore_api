use sea_orm_migration::prelude::*;

use crate::{m20230724_130406_create_authors::Authors, m20230724_133748_create_books::Books};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(BookAuthors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BookAuthors::AuthorId).integer().not_null())
                    .col(ColumnDef::new(BookAuthors::BookId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(BookAuthors::AuthorId)
                            .col(BookAuthors::BookId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_author_author")
                            .from(BookAuthors::Table, BookAuthors::AuthorId)
                            .to(Authors::Table, Authors::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book_author_book")
                            .from(BookAuthors::Table, BookAuthors::BookId)
                            .to(Books::Table, Books::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(BookAuthors::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum BookAuthors {
    Table,
    AuthorId,
    BookId,
}
