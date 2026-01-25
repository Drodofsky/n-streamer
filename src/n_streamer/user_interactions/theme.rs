use iced::{
    Element, Length,
    widget::{container, row},
};

use super::*;
impl NStreamer {
    pub(crate) fn view_theme_popup(&self) -> Element<'_, Message> {
        create_pop_up(
            "Select Theme".to_string(),
            container(
                row![
                    text_button("Light")
                        .width(Length::FillPortion(1))
                        .on_press(Message::Settings(SettingsMessage::UpdateTheme(
                            Theme::Light
                        ))),
                    text_button("Dark")
                        .width(Length::FillPortion(1))
                        .on_press(Message::Settings(SettingsMessage::UpdateTheme(Theme::Dark))),
                    text_button("System")
                        .width(Length::FillPortion(1))
                        .on_press(Message::Settings(SettingsMessage::UpdateTheme(
                            Theme::System
                        ))),
                ]
                .spacing(SPACING)
                .padding(PADDING),
            ),
        )
        .into()
    }
}
