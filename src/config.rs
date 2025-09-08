use anyhow::Result;
use std::env;

pub struct Config {
    pub discord_token: String,
    pub api_token: String,
    pub api_base_url: String,
}

impl Config {
    pub async fn load() -> Result<Self> {
        dotenv::dotenv().ok();

        let discord_token =
            env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN doit être défini dans le fichier .env");

        let api_token =
            env::var("API_TOKEN").expect("API_TOKEN doit être défini dans le fichier .env");

        let api_base_url =
            env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());

        Ok(Config {
            discord_token,
            api_token,
            api_base_url,
        })
    }
}
