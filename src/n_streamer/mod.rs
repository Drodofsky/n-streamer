mod components;
mod init;
mod message;
mod subscription;
mod theme;
mod ui_utils;
mod update;
mod user_interactions;
mod view;
pub use components::*;
pub use message::*;
pub use ui_utils::*;
pub use user_interactions::*;
#[derive(Default)]
pub struct NStreamer {
    clock: Clock,
    user_interactions: Vec<UserInteraction>,
}

impl NStreamer {
    pub fn new() -> Self {
        Self::default()
    }
}
