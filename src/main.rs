pub mod api;
pub mod bot;
pub mod config;
pub mod embed;
pub mod handlers;
pub mod models;

use crate::bot::Bot;
use crate::config::Config;
use crate::api::create_server;
use anyhow::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialiser tracing
    tracing_subscriber::fmt::init();

    println!("🤖 Démarrage du bot Discord...");

    // Charger la configuration
    let config = Config::load().await?;
    println!("📋 Configuration chargée");

    // Créer le channel pour communiquer entre le serveur et le bot
    let (score_sender, score_receiver) = tokio::sync::mpsc::unbounded_channel();

    // Créer le serveur Axum
    let app = create_server(config.clone(), score_sender).await?;
    println!("🌐 Serveur Axum créé");

    // Démarrer le serveur Axum
    let listener = TcpListener::bind("0.0.0.0:3005").await?;
    println!("🚀 Serveur Axum démarré sur le port 3000");
    
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    
    // Créer et démarrer le bot
    let mut bot = Bot::new(config, score_receiver).await?;
    bot.start().await?;

    Ok(())
}
