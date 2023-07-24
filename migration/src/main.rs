use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    if let Err(error) = dotenvy::dotenv() {
        eprintln!("Error loading env variables: {error}");
        return;
    }
    cli::run_cli(migration::Migrator).await;
}
