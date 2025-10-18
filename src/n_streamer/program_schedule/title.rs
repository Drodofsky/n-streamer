use chrono::Local;
use iced::Task;
use turso::Connection;

use crate::n_streamer::{
    db, error::Error, message::Message, program_schedule::analyzed_schedule::AnalyzedEpisode,
};

#[derive(Debug, Default)]
pub struct Title {
    current_episode: Option<AnalyzedEpisode>,
    connection: Option<Connection>,
}

impl Title {
    pub fn get_current_episode(&self) -> Option<&str> {
        self.current_episode
            .as_ref()
            .map(|e| e.program_title.as_str())
    }
    pub fn set_current_episode(&mut self, episode: Option<AnalyzedEpisode>) {
        self.current_episode = episode;
    }
    pub fn set_connectoin(&mut self, connection: Connection) {
        self.connection = Some(connection);
    }
    pub fn update(&mut self) -> Result<Option<Task<Message>>, Error> {
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
            let time = Local::now().to_string();

            let current_episode_task = Task::perform(
                db::get_current_episodes(connection.clone(), time.clone()),
                Message::CurrentEpisode,
            );

            Ok(Some(current_episode_task))
        } else {
            Ok(None)
        }
    }
}
