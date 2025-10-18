use serde::*;
use serde_json::Value;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramInfoRequest {
    pub item: Item,
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub program_master_id: i64,
    pub program_name: String,
    pub genre: Option<String>,
    pub logo_link: Option<String>,
    pub broadcast_time_show_flg: i64,
    pub broadcast_time_description: Option<String>,
    pub rebroadcast_time_show_flg: i64,
    pub rebroadcast_time_description: Option<String>,
    pub bilingual_flg: i64,
    pub english_flg: i64,
    pub link: Option<String>,
    pub synopsis: Option<String>,
    // Ignore
    pub schedules: Vec<Value>,
    // Ignore
    pub episodes: Vec<Value>,
}
