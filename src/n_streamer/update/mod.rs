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
            Message::SettingSelected(setting) => self.update_settings(setting),
        }
    }
}
