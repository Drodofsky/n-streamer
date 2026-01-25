use iced::Task;

use super::*;

impl NStreamer {
    pub fn init() -> (Self, Task<Message>) {
        let n_streamer = Self::new();
        let settings = Task::perform(Settings::load(), |s| {
            Message::Loaded(LoadedMessage::Settings(s))
        });

        (n_streamer, settings)
    }
    pub fn init_second_stage(&mut self) -> Task<Message> {
        self.update_theme()
    }
}
