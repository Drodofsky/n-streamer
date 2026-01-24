mod components;
mod init;
mod message;
mod subscription;
mod theme;
mod ui_utils;
mod update;
mod view;
pub use components::*;
pub use message::Message;
pub use ui_utils::*;
#[derive(Default)]
pub struct NStreamer {
    clock: Clock,
}

impl NStreamer {
    pub fn new() -> Self {
        Self::default()
    }
}
