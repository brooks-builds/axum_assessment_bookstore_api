use axum_assessment_bookstore_api::{config::AppConfig, run};

#[tokio::main]
async fn main() {
    let config = match AppConfig::new().await {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error creating config: {error}");
            return;
        }
    };
    match run(config).await {
        Ok(_) => println!("Server shut down"),
        Err(error) => eprintln!("Server crashed with error: {error}"),
    }
}
