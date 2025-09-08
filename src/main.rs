pub mod api;
pub mod bot;
pub mod config;
pub mod handlers;
pub mod models;

use crate::bot::Bot;
use crate::config::Config;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialiser tracing
    tracing_subscriber::fmt::init();

    println!("🤖 Démarrage du bot Discord...");

    // Charger la configuration
    let config = Config::load().await?;
    println!("📋 Configuration chargée");

    // Créer et démarrer le bot
    let mut bot = Bot::new(config).await?;
    bot.start().await?;

    Ok(())
}
