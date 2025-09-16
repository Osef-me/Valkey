use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreDisplay {
    // Informations de base
    pub id: i32,
    pub user_id: i64,
    pub username: Option<String>,
    pub beatmap_id: i32,
    pub rate: BigDecimal,
    pub mods: i64,
    pub rank: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    
    // Performance
    pub score: i32,
    pub accuracy: BigDecimal,
    pub max_combo: i32,
    pub perfect: bool,
    pub pause_count: i32,
    
    // Hits
    pub count_300: i32,
    pub count_100: i32,
    pub count_50: i32,
    pub count_miss: i32,
    pub count_katu: i32,
    pub count_geki: i32,
}
