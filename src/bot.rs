use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{
        application::{Command, Interaction},
        gateway::Ready,
    },
    prelude::GatewayIntents,
};
use tracing::{info, error};

use crate::config::Config;
use crate::handlers::commands::register;
use crate::api::Api;

pub struct BotHandler {
    api: Api,
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        info!("Interaction received: {:?}", interaction.kind());
        
        if let Interaction::Command(command) = interaction {
            info!("Command received: {}", command.data.name);
            info!("Options: {:?}", command.data.options());

            let user_id = command.user.id.get();
            let options = command.data.options();

            let response = match command.data.name.as_str() {
                "register" => {
                    info!("Executing register command");
                    let username = command.user.name.clone();
                    register::run(&ctx, &options, &self.api, user_id, Some(username)).await
                }
                _ => {
                    error!("Unknown command: {}", command.data.name);
                    serenity::builder::CreateInteractionResponse::Message(
                        serenity::builder::CreateInteractionResponseMessage::new()
                            .content("❌ Unknown command")
                    )
                }
            };

            if let Err(why) = command.create_response(&ctx.http, response).await {
                error!("Error sending response: {:?}", why);
            }

        } else {
            info!("Unrecognized interaction: {:?}", interaction);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected and ready!", ready.user.name);
        info!("Bot ID: {}", ready.user.id);

        // Enregistrer les commandes globales
        match Command::set_global_commands(&ctx.http, vec![
            register(),
        ]).await {
            Ok(commands) => {
                info!("Global commands registered successfully");
                for cmd in commands {
                    info!("  - /{}: {}", cmd.name, cmd.description);
                }
            }
            Err(e) => {
                error!("Error registering commands: {:?}", e);
            }
        }
    }
}

pub struct Bot {
    client: Client,
}

impl Bot {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // Créer l'API
        let api = Api::new(config.api_base_url.clone(), config.api_token.clone());
        
        // Créer le client avec les intents nécessaires
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
        
        let client = Client::builder(&config.discord_token, intents)
            .event_handler(BotHandler { 
                 api,
            })
            .await?;
        
        Ok(Bot { client })
    }
    
    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!("Bot starting...");
        
        if let Err(why) = self.client.start().await {
            error!("Error starting client: {:?}", why);
            return Err(anyhow::anyhow!("Startup error: {:?}", why));
        }
        
        Ok(())
    }
}