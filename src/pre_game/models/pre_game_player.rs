use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PreGamePlayerResponse {
    pub subject: String,
    #[serde(rename = "MatchID")]
    pub match_id: String,
    pub version: i64,
}