use serenity::builder::CreateEmbed;
use bigdecimal::BigDecimal;
use crate::models::ScoreReplay;

pub fn create_score_embed(score: &ScoreReplay) -> CreateEmbed {
    let beatmapset = score.beatmapset_complete.beatmapset.as_ref();
    let beatmap = score.beatmapset_complete.beatmap.first()
        .and_then(|b| b.beatmap.as_ref());
    let msd = score.beatmapset_complete.beatmap.first()
        .and_then(|b| b.msd.first());

    let unknown = "Unknown".to_string();
    let title = beatmapset.map(|bs| &bs.title).unwrap_or(&unknown);
    let creator = beatmapset.map(|bs| &bs.creator).unwrap_or(&unknown);
    let difficulty = beatmap.map(|b| &b.difficulty).unwrap_or(&unknown);
    let rate = &score.score.rate;
    
    // Calculate effective BPM (BPM * rate)
    let effective_bpm = beatmap
        .map(|b| &b.bpm * rate)
        .unwrap_or_else(|| BigDecimal::from(0));

    // Get cover URL
    let cover_url = beatmapset
        .and_then(|bs| bs.cover_url.as_ref())
        .map(|url| url.as_str())
        .unwrap_or("");

    // Calculate Perfect/Marvelous ratio
    let perfect_marvelous_ratio = if score.score_metadata.count_300 > 0 {
        score.score_metadata.count_geki as f64 / score.score_metadata.count_300 as f64
    } else {
        0.0
    };

    // Create osef.me link
    let osef_link = if let (Some(beatmapset_osu_id), Some(beatmap_osu_id)) = (
        beatmapset.and_then(|bs| bs.osu_id),
        beatmap.and_then(|b| b.osu_id)
    ) {
        format!("https://osef.me/beatmapsets/{}/{}", beatmapset_osu_id, beatmap_osu_id)
    } else {
        "".to_string()
    };

    let mut embed = CreateEmbed::new()
        .title(format!("{} - {} | {} | Rate: {:.1}", title, creator, difficulty, rate))
        .color(0x5865F2); // Discord blurple color

    // Add osef.me link if available
    if !osef_link.is_empty() {
        embed = embed.url(&osef_link);
    }

    // Player name (bigger, no "Player:" prefix)
    let player_name = format!(
        "**{}**",
        score.user.username.as_deref().unwrap_or("Unknown")
    );
    embed = embed.field("", player_name, false);

    // Score and accuracy on same row
    let score_accuracy = format!(
        "**Score:** {:?} | **Acc:** {:.2}%",
        score.score_metadata.score,
        score.score_metadata.accuracy
    );
    embed = embed.field("", score_accuracy, false);

    // Beatmap stats at top
    let beatmap_stats = format!(
        "**BPM:** {:.0} | **OD:** {} | **HP:** {} | **Max Combo:** {}",
        effective_bpm,
        beatmap.map(|b| &b.od).unwrap_or(&BigDecimal::from(0)),
        beatmap.map(|b| &b.hp).unwrap_or(&BigDecimal::from(0)),
        score.score_metadata.max_combo
    );
    embed = embed.field("", beatmap_stats, false);

    // Hits without colors and 2 per row with ratio on separate line
    let hits_info = format!(
        "**Ratio M:P:** {:.1}:1 | **Pause:** {}\n**Marv:** {} | **Perf:** {}\n**Great:** {} | **Good:** {}\n**Bad:** {} | **Miss:** {}",
        perfect_marvelous_ratio,
        score.score_metadata.pause_count,
        score.score_metadata.count_geki,
        score.score_metadata.count_300,
        score.score_metadata.count_katu,
        score.score_metadata.count_100,
        score.score_metadata.count_50,
        score.score_metadata.count_miss
    );
    embed = embed.field("", hits_info, false);

    // MSD Ratings - only main_patterns and overall
    if let Some(msd_data) = msd {
        let main_patterns = msd_data.main_pattern.as_deref().unwrap_or("None");
        let zero_bd = BigDecimal::from(0);
        let overall = msd_data.overall.as_ref().unwrap_or(&zero_bd);
        
        // Parse the JSON string to extract the patterns
        let patterns_display = if main_patterns != "None" {
            // Remove brackets and quotes from the JSON array
            main_patterns
                .trim_start_matches('[')
                .trim_end_matches(']')
                .replace('"', "")
                .replace(',', ", ")
        } else {
            "None".to_string()
        };
        
        let msd_info = format!(
            "**Overall:** {:.2} | **Patterns:** {}",
            overall, patterns_display
        );
        embed = embed.field("", msd_info, false);
    }

    // Add cover image at the bottom if available
    if !cover_url.is_empty() {
        embed = embed.image(cover_url);
    }

    embed
}
