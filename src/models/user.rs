use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub discord_id: i64,
    pub username: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub roles: Option<Value>,
}
