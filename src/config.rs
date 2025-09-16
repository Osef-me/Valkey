use anyhow::Result;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub discord_token: String,
    pub api_token: String,
    pub api_base_url: String,
    pub guild_id: u64,
    pub scores_channel_id: u64,
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

        let guild_id = env::var("GUILD_ID")
            .expect("GUILD_ID doit être défini dans le fichier .env")
            .parse()
            .expect("GUILD_ID doit être un nombre valide");

        let scores_channel_id = env::var("SCORES_CHANNEL_ID")
            .expect("SCORES_CHANNEL_ID doit être défini dans le fichier .env")
            .parse()
            .expect("SCORES_CHANNEL_ID doit être un nombre valide");

        Ok(Config {
            discord_token,
            api_token,
            api_base_url,
            guild_id,
            scores_channel_id,
        })
    }
}
