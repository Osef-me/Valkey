use anyhow::Result;
use crate::services::utils::*;
use tracing::{info, error};

pub struct ApiService {
    client: reqwest::Client,
    base_url: String,
}

impl ApiService {
    pub fn new(base_url: String, bearer_token: String) -> Self {
        let client = create_http_client(&bearer_token);
        Self { client, base_url }
    }

    pub async fn register_discord_user(
        &self,
        discord_id: i64,
        username: Option<String>,
    ) -> Result<DiscordRegisterResponse> {
        let request = DiscordRegisterRequest {
            discord_id,
            username: username.clone(),
        };

        let response = self
            .client
            .post(&format!("{}/api/internal/discord_register", self.base_url))
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        
        info!("API Response Status: {}", status);

        if status.is_success() {
            // Essayer de parser avec la structure flexible
            match serde_json::from_str::<crate::services::utils::ApiResponseEnum>(&body) {
                Ok(api_response) => {
                    match api_response {
                        crate::services::utils::ApiResponseEnum::TokenOnly(token) => {
                            // L'API retourne directement le token
                            info!("Token received directly");
                            Ok(DiscordRegisterResponse {
                                user: crate::services::utils::User {
                                    discord_id,
                                    username,
                                    created_at: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                                    roles: Some(serde_json::json!(["user"])),
                                },
                                message: "Client creation token generated".to_string(),
                                token: Some(token),
                            })
                        }
                        crate::services::utils::ApiResponseEnum::TokenMessage(token_response) => {
                            // L'API retourne token + message
                            info!("Token received with message");
                            Ok(DiscordRegisterResponse {
                                user: crate::services::utils::User {
                                    discord_id,
                                    username,
                                    created_at: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                                    roles: Some(serde_json::json!(["user"])),
                                },
                                message: token_response.message,
                                token: Some(token_response.token),
                            })
                        }
                        crate::services::utils::ApiResponseEnum::FullResponse(response) => {
                            // Structure complète
                            Ok(response)
                        }
                        crate::services::utils::ApiResponseEnum::AlternativeResponse(api_response) => {
                            // Structure alternative
                            if let (Some(user), Some(message)) = (api_response.user, api_response.message) {
                                Ok(DiscordRegisterResponse {
                                    user,
                                    message,
                                    token: api_response.token,
                                })
                            } else {
                                anyhow::bail!("Incomplete API response: user or message missing")
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("JSON parsing error: {}", e);
                    anyhow::bail!("API response format error: {}", e)
                }
            }
        } else {
            let error_msg = handle_http_error(status, &body);
            anyhow::bail!(error_msg)
        }
    }
}