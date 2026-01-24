use iced::Task;

use super::*;

impl NStreamer {
    pub fn init() -> (Self, Task<Message>) {
        let n_streamer = Self::new();

        (n_streamer, Task::none())
    }
}
