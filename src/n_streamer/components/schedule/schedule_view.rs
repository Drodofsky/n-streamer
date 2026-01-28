use super::*;
use chrono::{DateTime, Local, TimeDelta};
#[derive(Debug, Clone)]
pub struct ScheduleView {
    pub program_id: i64,
    pub program_title: String,
    pub episode_id: i64,
    pub episode_title: Option<String>,
    pub schedule: DateTime<Local>,
    pub period: TimeDelta,
}

impl Str for ScheduleView {
    fn get_str(&self) -> String {
        if let Some(ep_title) = &self.episode_title {
            format!("{} {}", self.program_title, ep_title)
        } else {
            self.program_title.clone()
        }
    }
}
