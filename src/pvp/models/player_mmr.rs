use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerMMRResponse {
    pub version: i64,
    pub subject: String,
    pub new_player_experience_finished: bool,
    pub queue_skills: HashMap<String, QueueSkill>,
    pub latest_competitive_update: LatestCompetitiveUpdate,
    pub is_leaderboard_anonymized: bool,
    pub is_act_rank_badge_hidden: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QueueSkill {
    pub total_games_needed_for_rating: i32,
    pub total_games_needed_for_leaderboard: i32,
    pub current_season_games_needed_for_rating: i32,
    #[serde(rename = "SeasonalInfoBySeasonID")]
    pub seasonal_info_by_season_id: HashMap<String, SeasonalInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SeasonalInfo {
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    pub number_of_wins: i32,
    pub number_of_wins_with_placements: i32,
    pub number_of_games: i32,
    pub rank: i32,
    pub capstone_wins: i32,
    pub leaderboard_rank: i32,
    pub competitive_tier: i32,
    pub ranked_rating: i32,
    pub wins_by_tier: Option<HashMap<String, i32>>,
    pub games_needed_for_rating: i32,
    pub total_wins_needed_for_rank: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LatestCompetitiveUpdate {
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "MapID")]
    pub map_id: String,
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    pub match_start_time: i64,
    pub tier_after_update: i32,
    pub tier_before_update: i32,
    pub ranked_rating_after_update: i32,
    pub ranked_rating_before_update: i32,
    pub ranked_rating_earned: i32,
    pub ranked_rating_performance_bonus: i32,
    pub competitive_movement: String,
    pub afk_penalty: i32,
}
