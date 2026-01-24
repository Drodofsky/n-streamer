use iced::Task;

use super::*;

impl NStreamer {
    pub fn init() -> (Self, Task<Message>) {
        let mut n_streamer = Self::new();
        let t1 = n_streamer.update_theme();

        (n_streamer, t1)
    }
}
