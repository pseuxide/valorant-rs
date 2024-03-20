use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PenaltiesResponse {
    pub subject: String,
    pub penalties: Vec<Value>,
    pub version: i64,
}
