use eyre::Result;

#[derive(Clone)]
pub struct AppConfig {
    pub address: [u8; 4],
    pub port: u16,
}

impl AppConfig {
    pub async fn new() -> Result<Self> {
        let address = [127, 0, 0, 1];
        let port = 3000;

        Ok(Self { address, port })
    }
}
