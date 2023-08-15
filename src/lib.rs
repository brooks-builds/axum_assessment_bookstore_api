pub mod config;
mod router;

use config::AppConfig;
use eyre::Result;
use router::create_router;
use std::net::SocketAddr;

pub async fn run(config: AppConfig) -> Result<()> {
    tracing_subscriber::fmt::init();

    let router = create_router();
    let address = SocketAddr::from((config.address, config.port));

    tracing::info!("Server running on port {}", config.port);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
