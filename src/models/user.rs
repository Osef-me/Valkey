use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub discord_id: i64,
    pub username: Option<String>,
    pub created_at: Option<String>,
    pub roles: Option<serde_json::Value>,
}