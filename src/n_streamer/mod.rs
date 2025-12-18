mod live_stream;
mod message;
mod settings;
mod time;
use std::{collections::HashSet, num::NonZeroI64, path::PathBuf, time::Duration};

mod config;
mod db;
mod download;
mod error;
mod program_schedule;
mod ui_utils;
mod update;
mod utils;
mod view;

use crate::n_streamer::{
    config::Config,
    download::Downloads,
    error::Error,
    live_stream::LiveStream,
    program_schedule::{ProgramSchedule, title::Title},
    utils::download_image_if_not_exists,
};
use iced::{Subscription, Task, futures::future::join_all, window};

use message::Message;
use turso::{Builder, Connection, Database};

use crate::n_streamer::{settings::Settings, time::Time, ui_utils::DynView};

pub struct NStreamer {
    settings: Settings,
    time: Time,
    theme: iced::Theme,
    title: Title,
    user_interactions: Vec<UserInteraction>,
    life_stream: LiveStream,
    center: Center,
    program_schedule: ProgramSchedule,
    downloads: Downloads,
    config: Config,
    db: Option<Database>,
}

impl Default for NStreamer {
    fn default() -> Self {
        Self {
            theme: iced::Theme::Dark,
            settings: Settings,
            time: Time::default(),
            title: Title::default(),
            user_interactions: Vec::new(),
            life_stream: LiveStream::default(),
            center: Center::default(),
            program_schedule: ProgramSchedule::default(),
            downloads: Downloads::default(),
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
    fn get_top_user_interaction(&self) -> Option<&DynView<Self, Message>> {
        self.user_interactions.iter().last().map(|u| &u.view)
    }
    fn add_user_interaction(&mut self, interaction: DynView<Self, Message>, priority: Priority) {
        self.user_interactions.push(UserInteraction {
            view: interaction,
            priority,
        });
        self.user_interactions.sort();
    }
    fn close_user_interaction(&mut self) {
        self.user_interactions
            .remove(self.user_interactions.len() - 1);
    }
    fn clear_user_interaction(&mut self) {
        self.user_interactions.clear();
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

async fn download_schedule(
    connection: Result<Connection, turso::Error>,
    base_path: Option<PathBuf>,
) -> Result<(), Error> {
    let connection = connection?;
    let schedule = ProgramSchedule::get_analyzed_schedule().await?;

    let ids: HashSet<NonZeroI64> = schedule
        .episodes
        .iter()
        .filter_map(|e| NonZeroI64::new(e.program_id))
        .collect();
    let program_info_futures = ids
        .iter()
        .map(|id| ProgramSchedule::get_analyzed_program_info(*id));
    let programs = join_all(program_info_futures).await;
    let download_images = programs
        .iter()
        .filter_map(|p| p.as_ref().ok())
        .filter_map(|p| p.logo_link.as_ref())
        .map(|l| download_image_if_not_exists(l, base_path.clone()));
    for res in join_all(download_images).await.iter() {
        res.clone()?;
    }

    db::add_episodes(connection.clone(), schedule.episodes).await?;
    db::add_programs(connection, programs).await?;
    Ok(())
}

pub struct UserInteraction {
    view: DynView<NStreamer, Message>,
    priority: Priority,
}

impl PartialEq for UserInteraction {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}
impl Eq for UserInteraction {}

impl PartialOrd for UserInteraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for UserInteraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority {
    Exit = 16,
    Error = 8,
    Warn = 4,
    Task = 2,
    Info = 1,
}
