pub struct AppConfig {
    pub address: [u8; 4],
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        let address = [127, 0, 0, 1];
        let port = 3000;

        Self { address, port }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}
