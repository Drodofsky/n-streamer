pub mod analyzed_schedule;
pub mod parsed_schedule;

use chrono::{Local, TimeDelta};
use iced::{Element, Task, widget::text};
use turso::Connection;

use crate::n_streamer::{
    db,
    error::Error,
    message::Message,
    program_schedule::{
        analyzed_schedule::{AnalyzedEpisode, AnalyzedSchedule},
        parsed_schedule::ScheduleRequest,
    },
};

#[derive(Debug, Default)]
pub struct ProgramSchedule {
    current_episode: Option<AnalyzedEpisode>,
    episodes: Vec<AnalyzedEpisode>,
    connection: Option<Connection>,
}

impl ProgramSchedule {
    pub fn view(&self) -> Element<'_, Message> {
        text("TODO").into()
    }
    pub fn schedule(&self) -> &[AnalyzedEpisode] {
        &self.episodes
    }
    pub fn set_connectoin(&mut self, connection: Connection) {
        self.connection = Some(connection);
    }
    pub fn set_schedule(&mut self, episodes: Vec<AnalyzedEpisode>) {
        self.episodes = episodes;
    }
    pub fn get_current_episode(&self) -> Option<&str> {
        self.current_episode
            .as_ref()
            .map(|e| e.program_title.as_str())
    }
    pub fn set_current_episode(&mut self, episode: Option<AnalyzedEpisode>) {
        self.current_episode = episode;
    }

    pub fn update_current_episode(&mut self) -> Result<Option<Task<Message>>, Error> {
        if let Some(ce) = &self.current_episode {
            let now = Local::now();
            if now
                < ce.schedule
                    .checked_add_signed(ce.period)
                    .ok_or(Error::Chrono("failed to calculate time offset".to_string()))?
            {
                return Ok(None);
            }
        }
        if let Some(connection) = self.connection.clone() {
            let last_episode = self
                .current_episode
                .as_ref()
                .map(|c| c.schedule.to_string())
                .unwrap_or(
                    Local::now()
                        .checked_sub_signed(TimeDelta::hours(3))
                        .ok_or(Error::Chrono("failed to calculate time offset".to_string()))?
                        .to_string(),
                );
            Ok(Some(Task::perform(
                db::get_current_episodes(connection.clone(), last_episode),
                Message::CurrentEpisode,
            )))
        } else {
            Ok(None)
        }
    }

    pub async fn get_analyzed_schedule() -> Result<AnalyzedSchedule, Error> {
        let json: ScheduleRequest =
            reqwest::get("https://nhkworldpremium.com/backend/api/v1/front/episodes?lang=en")
                .await?
                .json()
                .await?;
        if json.status != 400 {
            return Err(Error::Api(format!("API: {}", json.status)));
        }
        json.item.try_into()
    }
}
