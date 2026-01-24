use super::*;
use iced::Task;

impl NStreamer {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.clock.update();
                Task::none()
            }
        }
    }
}
