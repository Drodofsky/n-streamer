mod live_stream;
mod message;
mod settings;
mod time;
use std::time::Duration;

mod config;
mod db;
mod error;
mod program_schedule;
mod ui_utils;
mod update;
mod utils;
mod view;
use crate::n_streamer::{
    config::Config, error::Error, live_stream::LiveStream, program_schedule::ProgramSchedule,
};
use iced::{Subscription, Task, window};

use message::Message;
use turso::{Builder, Connection, Database};

use crate::n_streamer::{settings::Settings, time::Time, ui_utils::DynView};

pub struct NStreamer {
    settings: Settings,
    time: Time,
    theme: iced::Theme,
    user_interaction: Option<DynView<Self, Message>>,
    life_stream: LiveStream,
    center: Center,
    program_schedule: ProgramSchedule,
    config: Config,
    db: Option<Database>,
}

impl Default for NStreamer {
    fn default() -> Self {
        Self {
            theme: iced::Theme::Dark,
            settings: Settings::default(),
            time: Time::default(),
            user_interaction: None,
            life_stream: LiveStream::default(),
            center: Center::default(),
            program_schedule: ProgramSchedule::default(),
            config: Config::default(),
            db: None,
        }
    }
}

impl NStreamer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init() -> (Self, Task<Message>) {
        let n_streamer = Self::new();
        let config = Task::perform(Config::load(), Message::ConfigLoaded);

        let task = Task::batch([config]);
        (n_streamer, task)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick);
        let long_tick = iced::time::every(Duration::from_secs(3600)).map(|_| Message::LongTick);
        let close = window::close_requests().map(Message::ExitRequest);
        Subscription::batch([tick, close, long_tick])
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }
    fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    async fn init_db(config: Config) -> Result<Database, Error> {
        let error = Error::Config("Failed to get media path".to_string());
        if let Some(path) = config.media_path() {
            let path = path.join("db.sqlite");
            let path = path.to_str().ok_or(error)?;
            Ok(Builder::new_local(path).build().await?)
        } else {
            Err(error)
        }
    }
    fn set_database(&mut self, db: Database) {
        self.db = Some(db);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Center {
    #[default]
    ProgramSchedule,
    LiveStream,
    Downloads,
    Library,
}

async fn download_schedule(connection: Result<Connection, turso::Error>) -> Result<(), Error> {
    let schedule = ProgramSchedule::get_analyzed_schedule().await?;
    db::add_episodes(connection, schedule.episodes).await?;

    Ok(())
}
