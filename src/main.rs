use axum_assessment_bookstore_api::{config::AppConfig, run};

#[tokio::main]
async fn main() {
    let config = AppConfig::new();
    match run(config).await {
        Ok(_) => println!("Server shut down"),
        Err(error) => eprintln!("Server crashed with error: {error}"),
    }
}
