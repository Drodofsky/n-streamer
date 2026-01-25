mod loaded;
mod settings;
mod theme;
mod window;
pub use super::*;
use iced::Task;

impl NStreamer {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.clock.update();
                Task::none()
            }
            Message::Window(wm) => self.update_window(wm),
            Message::Result(res) => self.apply_result_and_return(res, Task::none()),
            Message::Loaded(l) => self.loaded(l),
            Message::Settings(s) => self.update_setting(s),
        }
    }
}
