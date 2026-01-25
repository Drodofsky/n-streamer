mod components;
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
pub use components::*;
pub use error::*;
pub use message::*;
pub use ui_utils::*;
pub use user_interactions::*;
pub use utils::*;

pub struct NStreamer {
    clock: Clock,
    user_interactions: Vec<UserInteraction>,
    settings: Settings,
    theme: iced::Theme,
}

impl NStreamer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for NStreamer {
    fn default() -> Self {
        NStreamer {
            clock: Clock::default(),
            user_interactions: Vec::new(),
            settings: Settings::default(),
            theme: iced::Theme::Dark,
        }
    }
}
