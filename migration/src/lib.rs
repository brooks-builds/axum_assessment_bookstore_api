pub use sea_orm_migration::prelude::*;

mod m20230724_130406_create_authors;
mod m20230724_133748_create_books;
mod m20230724_133757_create_book_authors;
mod m20230725_132431_seed_authors;
mod m20230725_135133_seed_books;
mod m20230725_140752_seed_book_authors;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230724_130406_create_authors::Migration),
            Box::new(m20230724_133748_create_books::Migration),
            Box::new(m20230724_133757_create_book_authors::Migration),
            Box::new(m20230725_132431_seed_authors::Migration),
            Box::new(m20230725_135133_seed_books::Migration),
            Box::new(m20230725_140752_seed_book_authors::Migration),
        ]
    }
}
