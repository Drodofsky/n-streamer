use crate::n_streamer::utils::{japan_time_to_utc, parse_time_delta};

use super::*;
use chrono::{DateTime, TimeDelta, Utc};
use serde::*;
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzedEpisode {
    pub program_id: i64,
    pub program_title: String,
    pub episode_id: i64,
    pub episode_title: Option<String>,
    pub suspend_flg: bool,
    pub schedule: DateTime<Utc>,
    pub period: TimeDelta,
    pub rebroadcast_flg: Option<bool>,
    pub bilingual_flg: Option<bool>,
    pub english_flg: Option<bool>,
}
#[derive(Debug, Default, Clone)]
pub struct AnalyzedSchedule {
    pub episodes: Vec<AnalyzedEpisode>,
}

impl TryFrom<Schedule> for AnalyzedSchedule {
    type Error = Error;
    fn try_from(value: Schedule) -> Result<Self, Self::Error> {
        let mut res_eps = Vec::new();
        for episode in value.episodes {
            let Episode {
                program_id,
                program_title,
                episode_id,
                episode_title,
                suspend_flg,
                rebroadcast_flg,
                bilingual_flg,
                english_flg,
                ..
            } = episode;
            let schedule = japan_time_to_utc(&episode.schedule)?;
            let period = parse_time_delta(&episode.period)?;
            res_eps.push(AnalyzedEpisode {
                program_id,
                program_title,
                episode_id,
                episode_title,
                suspend_flg,
                schedule,
                period,
                rebroadcast_flg,
                bilingual_flg,
                english_flg,
            });
        }

        Ok(AnalyzedSchedule { episodes: res_eps })
    }
}

impl fmt::Display for AnalyzedEpisode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let schedule = self.schedule.format("%m/%d(%a) %H:%M");
        if let Some(ep_title) = &self.episode_title {
            write!(f, "[{}]: {}: {}", schedule, self.program_title, ep_title)
        } else {
            write!(f, "[{}]: {}", schedule, self.program_title)
        }
    }
}
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for AnalyzedEpisode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.episode_id == other.episode_id {
            return Some(std::cmp::Ordering::Equal);
        }
        self.schedule.partial_cmp(&other.schedule)
    }
}

impl Ord for AnalyzedEpisode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.episode_id == other.episode_id {
            return std::cmp::Ordering::Equal;
        }
        self.schedule.cmp(&other.schedule)
    }
}
