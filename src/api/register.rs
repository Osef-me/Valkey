use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use crate::api::Api;

// Request structure
#[derive(Debug, Serialize)]
pub struct DiscordRegisterRequest {
    pub discord_id: i64,
    pub username: Option<String>,
}

// Response structure
#[derive(Debug, Deserialize)]
pub struct TokenMessageResponse {
    pub token: String,
    pub message: String,
}

impl Api {

    pub async fn register_discord_user(
        &self,
        discord_id: i64,
        username: Option<String>,
    ) -> Result<TokenMessageResponse> {
        let request = DiscordRegisterRequest {
            discord_id,
            username,
        };

        let response = self
            .get_client()
            .post(&format!("{}/api/internal/discord_register", self.get_base_url()))
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        
        info!("API Response Status: {}", status);

        if status.is_success() {
            match serde_json::from_str::<TokenMessageResponse>(&body) {
                Ok(token_response) => {
                    info!("Token received successfully");
                    Ok(token_response)
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

fn handle_http_error(status: reqwest::StatusCode, body: &str) -> String {
    match status {
        reqwest::StatusCode::CONFLICT => "User already exists".to_string(),
        reqwest::StatusCode::UNAUTHORIZED => "Invalid authentication token".to_string(),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => "Internal server error".to_string(),
        _ => format!("HTTP error {}: {}", status, body),
    }
}
