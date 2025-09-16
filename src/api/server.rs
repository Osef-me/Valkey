use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;

use crate::models::ScoreReplay;
use crate::config::Config;

pub type ScoreSender = mpsc::UnboundedSender<ScoreReplay>;

pub struct ServerState {
    pub score_sender: ScoreSender,
}

pub async fn create_server(_config: Config, score_sender: ScoreSender) -> anyhow::Result<Router> {
    let state = Arc::new(ServerState { score_sender });

    let app = Router::new()
        .route("/scores", post(handle_score))
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(app)
}

async fn handle_score(
    State(state): State<Arc<ServerState>>,
    Json(score): Json<ScoreReplay>,
) -> Result<Json<Value>, StatusCode> {
    tracing::info!("Score reçu: {:?}", score);

    // Envoyer le score au bot Discord
    if let Err(e) = state.score_sender.send(score) {
        tracing::error!("Erreur lors de l'envoi du score: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(json!({
        "status": "success",
        "message": "Score envoyé au bot Discord"
    })))
}
