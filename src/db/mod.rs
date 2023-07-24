use dotenvy_macro::dotenv;
use eyre::Result;
use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> Result<DatabaseConnection> {
    let database_url = dotenv!("DATABASE_URL").to_owned();
    let db = Database::connect(database_url).await?;

    Ok(db)
}
