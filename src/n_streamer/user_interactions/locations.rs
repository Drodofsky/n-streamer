use iced::{
    Element,
    Length::FillPortion,
    widget::{column, container, row, text, text_input},
};

use super::*;
impl NStreamer {
    pub(crate) fn view_locations_popup(&self) -> Element<'_, Message> {
        create_pop_up(
            "Configure Locations".to_string(),
            container(
                column![
                    text("Streaming URL"),
                    text_input("", self.settings.stream_url().unwrap_or(""))
                        .on_input(|s| Message::Settings(SettingsMessage::NewStreamUrl(s))),
                    text("Media folder"),
                    text_input(
                        "",
                        self.settings
                            .media_path()
                            .and_then(|p| p.to_str())
                            .unwrap_or("")
                    )
                    .on_input(|s| Message::Settings(SettingsMessage::NewMediaPath(s))),
                    row![
                        text_button("Browse")
                            .width(FillPortion(1))
                            .on_press(Message::Settings(SettingsMessage::OpenMediaPathBrowser))
                    ],
                    hl(1),
                    row![
                        text_button("ok")
                            .width(FillPortion(1))
                            .on_press(Message::Settings(SettingsMessage::SaveAndCloseSettings))
                    ]
                ]
                .padding(PADDING)
                .spacing(SPACING),
            ),
        )
        .into()
    }
}
