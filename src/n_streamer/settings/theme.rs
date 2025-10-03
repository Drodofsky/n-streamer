use iced::{Element, Length, Task, theme::Mode, widget::row};

use crate::{
    button_text,
    n_streamer::{
        NStreamer,
        config::Theme,
        message::Message,
        ui_utils::{PADDING, SPACING},
    },
    pop_up,
};

impl NStreamer {
    pub(crate) fn view_theme_popup(&self) -> Element<'_, Message> {
        pop_up!(
            "Select Theme",
            row![
                button_text!("Light")
                    .width(Length::FillPortion(1))
                    .on_press(Message::UpdateTheme(Theme::Light)),
                button_text!("Dark")
                    .width(Length::FillPortion(1))
                    .on_press(Message::UpdateTheme(Theme::Dark)),
                button_text!("System")
                    .width(Length::FillPortion(1))
                    .on_press(Message::UpdateTheme(Theme::System)),
            ]
            .spacing(SPACING)
            .padding(PADDING)
        )
        .into()
    }
    pub(crate) fn update_theme(&mut self) -> Task<Message> {
        let theme = self.config.theme();

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
                Message::ApplyTheme(theme)
            }),
        }
    }
}
