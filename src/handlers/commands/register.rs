use anyhow::Result;
use serenity::{
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    model::application::ResolvedOption,
    prelude::Context,
};
use tracing::{info, error};
use crate::api::Api;

pub fn register() -> CreateCommand {
    CreateCommand::new("register")
        .description("Generate a client creation token")
        .dm_permission(false)
}

pub async fn run(
    ctx: &Context,
    _options: &[ResolvedOption<'_>], 
    api_service: &Api,
    user_id: u64,
    username: Option<String>
) -> CreateInteractionResponse {
    info!("Processing register command for user {}", user_id);

    // Call API to generate token
    match api_service.register_discord_user(user_id as i64, username).await {
        Ok(response) => {
            // Send token in private message
            if let Err(e) = send_token_private_message(ctx, user_id, &response.token).await {
                error!("Error sending private message: {:?}", e);
                return CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("✅ Client creation token generated successfully!\n\n❌ Error sending private message. Please check that the bot can send you private messages.")
                );
            }

            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("✅ Client creation token generated successfully!\n\n📬 A private message has been sent to you with your client creation token.")
            )
        }
        Err(e) => {
            error!("Error generating token: {}", e);
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!("❌ Error generating token: {}", e))
            )
        }
    }
}

async fn send_token_private_message(ctx: &Context, user_id: u64, token: &str) -> Result<()> {
    let user_id = serenity::model::id::UserId::new(user_id);
    
    let dm_channel = user_id.create_dm_channel(&ctx.http).await?;
    
    let content = format!(
        "🎉 **Client creation token generated!**\n\nYour client creation token: ||`{}`||\n\n📝 **Instructions** :\n1. Use this token to create your first client\n2. This token is only used for initial client setup\n3. After creating a client, you'll get a proper device token\n4. Keep this token secure\n\n⚠️ **Important** : Keep this token secret and don't share it with anyone!",
        token
    );
    
    dm_channel.say(&ctx.http, content).await?;
    
    Ok(())
}
