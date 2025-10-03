use serde::*;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRequest {
    pub item: Schedule,
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub start_date: String,
    pub end_date: String,
    pub episodes: Vec<Episode>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub program_id: i64,
    pub program_title: String,
    pub episode_id: i64,
    pub episode_title: Option<String>,
    pub suspend_flg: bool,
    pub schedule: String,
    pub period: String,
    pub rebroadcast_flg: Option<bool>,
    pub bilingual_flg: Option<bool>,
    pub english_flg: Option<bool>,
}
