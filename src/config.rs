use crate::db::connect;
use eyre::Result;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppConfig {
    pub address: [u8; 4],
    pub port: u16,
    pub db: DatabaseConnection,
}

impl AppConfig {
    pub async fn new() -> Result<Self> {
        let address = [127, 0, 0, 1];
        let port = 3000;
        let db = connect().await?;

        Ok(Self { address, port, db })
    }
}
