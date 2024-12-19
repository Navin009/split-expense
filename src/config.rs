use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;
use mongodb::{options::ClientOptions, Client, Database};
use regex::Regex;
use serde_yaml::Value;
use std::{collections::HashMap, env, fs::File, io::Read};

pub struct AppConfig {
    pub mongodb: Database,
}

impl AppConfig {
    pub async fn new() -> Result<AppConfig, Box<dyn std::error::Error>> {
        dotenv().ok();

        let config = AppConfig::init_app_config().await?;

        let mongodb = AppConfig::init_db(&config).await?;

        Ok(AppConfig { mongodb })
    }

    pub async fn init_db(config: &HashMap<String, Value>) -> mongodb::error::Result<Database> {
        let mongo_uri = format!(
            "mongodb://{}:{}@{}:{}",
            config["database"]["mongodb"]["user"].as_str().unwrap(),
            config["database"]["mongodb"]["password"].as_str().unwrap(),
            config["database"]["mongodb"]["host"].as_str().unwrap(),
            config["database"]["mongodb"]["port"].as_u64().unwrap()
        );

        log::info!("MongoDB URI: {}", mongo_uri);

        let client_options = ClientOptions::parse(&mongo_uri).await?;
        let client = Client::with_options(client_options)?;
        Ok(client.database("url_shortener"))
    }

    pub fn init_logger() {
        Builder::new().filter_level(LevelFilter::Info).init();
    }

    pub async fn init_app_config() -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let mut file = File::open("resources/application.yaml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let processed_contents = AppConfig::process_placeholders(&contents);

        let config: HashMap<String, Value> = serde_yaml::from_str(&processed_contents)?;
        Ok(config)
    }

    fn process_placeholders(contents: &str) -> String {
        // Regex to match placeholders of the form ${VAR:default_value}
        let re = Regex::new(r"\$\{([A-Za-z0-9_]+):([^\}]+)\}").unwrap();

        // Replace each placeholder with its corresponding value
        re.replace_all(contents, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let default_value = &caps[2];

            // Get the value of the environment variable or use the default value
            env::var(var_name).unwrap_or_else(|_| default_value.to_string())
        })
        .to_string()
    }
}
