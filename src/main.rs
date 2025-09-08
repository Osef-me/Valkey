pub mod bot;
pub mod config;
pub mod models;
pub mod api;
pub mod handlers;

use anyhow::Result;
use crate::config::Config;
use crate::bot::Bot;

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