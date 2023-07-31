pub mod config;
pub mod db;
pub mod models;
mod router;

use std::net::SocketAddr;

use config::AppConfig;
use eyre::Result;
use router::create_router;

pub async fn run(config: AppConfig) -> Result<()> {
    tracing_subscriber::fmt::init();

    let router = create_router(config.clone());
    let address = SocketAddr::from((config.address, config.port));

    tracing::info!("Server running on port {}", config.port);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
