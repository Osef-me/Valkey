use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use super::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreReplay {
    pub score: Score,
    pub user: User,
    pub beatmapset_complete: BeatmapsetCompleteExtended,
    pub score_metadata: ScoreMetadata,
    pub score_rating: Option<ScoreRating>,
    pub replay: Option<Replay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub id: i32,
    pub user_id: i64,
    pub beatmap_id: i32,
    pub score_metadata_id: i32,
    pub replay_id: Option<i32>,
    pub rate: BigDecimal,
    pub hwid: Option<String>,
    pub mods: i64,
    pub hash: String,
    pub rank: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeatmapsetCompleteExtended {
    pub beatmapset: Option<BeatmapsetExtended>,
    pub beatmap: Vec<BeatmapCompleteExtended>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeatmapsetExtended {
    pub id: i32,
    pub osu_id: Option<i32>,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub title: String,
    pub title_unicode: Option<String>,
    pub creator: String,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub has_video: bool,
    pub has_storyboard: bool,
    pub is_explicit: bool,
    pub is_featured: bool,
    pub cover_url: Option<String>,
    pub preview_url: Option<String>,
    pub osu_file_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeatmapCompleteExtended {
    pub beatmap: Option<BeatmapExtended>,
    pub msd: Vec<MSDExtended>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeatmapExtended {
    pub id: i32,
    pub osu_id: Option<i32>,
    pub beatmapset_id: Option<i32>,
    pub difficulty: String,
    pub difficulty_rating: BigDecimal,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub max_combo: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: BigDecimal,
    pub cs: BigDecimal,
    pub ar: BigDecimal,
    pub od: BigDecimal,
    pub hp: BigDecimal,
    pub mode: i32,
    pub status: String,
    pub file_md5: String,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MSDExtended {
    pub id: Option<i32>,
    pub beatmap_id: Option<i32>,
    pub overall: Option<BigDecimal>,
    pub stream: Option<BigDecimal>,
    pub jumpstream: Option<BigDecimal>,
    pub handstream: Option<BigDecimal>,
    pub stamina: Option<BigDecimal>,
    pub jackspeed: Option<BigDecimal>,
    pub chordjack: Option<BigDecimal>,
    pub technical: Option<BigDecimal>,
    pub rate: Option<BigDecimal>,
    pub main_pattern: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreMetadata {
    pub id: i32,
    pub skin: Option<String>,
    pub pause_count: i32,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
    pub time_paused: i32,
    pub score: i32,
    pub accuracy: BigDecimal,
    pub max_combo: i32,
    pub perfect: bool,
    pub count_300: i32,
    pub count_100: i32,
    pub count_50: i32,
    pub count_miss: i32,
    pub count_katu: i32,
    pub count_geki: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRating {
    pub id: i32,
    pub score_id: i32,
    pub rating: BigDecimal,
    pub rating_type: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replay {
    pub id: i32,
    pub hash: String,
    pub replay_available: bool,
    pub replay_path: String,
    pub created_at: Option<NaiveDateTime>,
}
