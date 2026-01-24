use iced::theme::Mode;

use super::*;

impl NStreamer {
    pub(crate) fn update_theme(&mut self) -> Task<Message> {
        let theme = self.settings.get_theme();

        match theme {
            Theme::Dark => {
                self.theme = iced::Theme::Dark;
                Task::none()
            }
            Theme::Light => {
                self.theme = iced::Theme::Light;
                Task::none()
            }
            Theme::System => iced::system::theme().map(|t| {
                let theme = match t {
                    Mode::Dark => iced::Theme::Dark,
                    Mode::Light => iced::Theme::Light,
                    Mode::None => {
                        return Message::Tick;
                    }
                };
                Message::Window(WindowMessage::ApplyTheme(theme))
            }),
        }
    }
}
