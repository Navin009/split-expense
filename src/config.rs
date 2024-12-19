use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;
use mongodb::{options::ClientOptions, Client};
use std::env;

pub struct AppConfig {
    pub mongo_client: Client,
}

impl AppConfig {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let mongo_uri = env::var("MONGODB_URI")?;
        let mut client_options = ClientOptions::parse(&mongo_uri).await?;
        client_options.app_name = Some("Splitwise".to_string());

        let mongo_client = Client::with_options(client_options)?;

        Ok(AppConfig { mongo_client })
    }

    pub fn init_logger() {
        Builder::new()
            .filter_level(LevelFilter::Info)
            .init();
    }
}
