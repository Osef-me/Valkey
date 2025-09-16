use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{
        application::{Command, Interaction},
        gateway::Ready,
    },
    prelude::GatewayIntents,
};
use tracing::{error, info};
use tokio::sync::mpsc;

use crate::api::Api;
use crate::config::Config;
use crate::embed::create_score_embed;
use crate::handlers::commands::register;
use crate::models::ScoreReplay;

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
                            .content("❌ Unknown command"),
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
        match Command::set_global_commands(&ctx.http, vec![register()]).await {
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
    config: Config,
    score_receiver: mpsc::UnboundedReceiver<ScoreReplay>,
}

impl Bot {
    pub async fn new(config: Config, score_receiver: mpsc::UnboundedReceiver<ScoreReplay>) -> anyhow::Result<Self> {
        // Créer l'API
        let api = Api::new(config.api_base_url.clone(), config.api_token.clone());

        // Créer le client avec les intents nécessaires
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let client = Client::builder(&config.discord_token, intents)
            .event_handler(BotHandler { 
                api,
            })
            .await?;

        Ok(Bot { 
            client,
            config,
            score_receiver,
        })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!("Bot starting...");

        // Démarrer la tâche de gestion des scores
        let config = self.config.clone();
        let http = self.client.http.clone();
        let mut score_receiver = std::mem::replace(&mut self.score_receiver, mpsc::unbounded_channel().1);
        
        tokio::spawn(async move {
            while let Some(score) = score_receiver.recv().await {
                if let Err(e) = Self::publish_score(&http, &config, &score).await {
                    error!("Erreur lors de la publication du score: {}", e);
                }
            }
        });

        if let Err(why) = self.client.start().await {
            error!("Error starting client: {:?}", why);
            return Err(anyhow::anyhow!("Startup error: {:?}", why));
        }

        Ok(())
    }

    async fn publish_score(
        http: &serenity::http::Http,
        config: &Config,
        score: &ScoreReplay,
    ) -> anyhow::Result<()> {
        let channel_id = serenity::model::id::ChannelId::new(config.scores_channel_id);
        
        let embed = create_score_embed(score);
        let builder = serenity::builder::CreateMessage::new().embed(embed);
        
        channel_id.send_message(http, builder).await?;
        
        info!("Score published for user {} (ID: {})", 
              score.user.username.as_deref().unwrap_or("Unknown"), 
              score.user.discord_id);
        
        Ok(())
    }
}
