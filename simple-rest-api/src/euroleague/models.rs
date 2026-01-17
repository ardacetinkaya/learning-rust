use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "@code")]
    pub game_code: i32,
    #[serde(rename = "@seasoncode")]
    pub season_code: String,
    #[serde(rename = "@cetdate")]
    pub date: String,
    #[serde(rename = "localclub")]
    pub local: Team,
    #[serde(rename = "roadclub")]
    pub road: Team,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "@code")]
    pub club_code: String,
    #[serde(rename = "@name")]
    pub club_name: String,
    #[serde(rename = "@score")]
    pub score: i32,
}
