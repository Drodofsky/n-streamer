mod init;
mod message;
mod subscription;
mod theme;
mod update;
mod view;

pub use message::Message;

pub struct NStreamer;

impl NStreamer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NStreamer {
    fn default() -> Self {
        NStreamer
    }
}
