mod components;
mod db;
mod error;
mod init;
mod message;
mod subscription;
mod theme;
mod ui_utils;
mod update;
mod user_interactions;
mod utils;
mod view;
use std::path::PathBuf;

pub use components::*;
use directories::ProjectDirs;
pub use error::*;
pub use message::*;
use turso::Database;
pub use ui_utils::*;
pub use user_interactions::*;

pub struct NStreamer {
    clock: Clock,
    user_interactions: Vec<UserInteraction>,
    settings: Settings,
    theme: iced::Theme,
    live_stream: LiveStream,
    center: Center,
    db: Option<Database>,
    schedule: ProgramSchedule,
    // only for testing
    project_dir: Option<ProjectDirs>,
}

impl NStreamer {
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn set_project_dir(&mut self, path: impl Into<PathBuf>) {
        self.project_dir = ProjectDirs::from_path(path.into());
    }
}

impl Default for NStreamer {
    fn default() -> Self {
        NStreamer {
            clock: Clock::default(),
            user_interactions: Vec::new(),
            settings: Settings::default(),
            theme: iced::Theme::Dark,
            live_stream: LiveStream::default(),
            center: Center::default(),
            project_dir: None,
            db: None,
            schedule: ProgramSchedule::default(),
        }
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
