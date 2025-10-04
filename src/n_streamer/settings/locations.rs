use iced::{
    Element,
    Length::FillPortion,
    widget::{column, row, text, text_input},
};
use rfd::AsyncFileDialog;

use crate::{
    button_text, hl,
    n_streamer::{
        NStreamer,
        message::Message,
        ui_utils::{PADDING, SPACING},
    },
    pop_up,
};

impl NStreamer {
    pub(crate) fn view_locations_popup(&self) -> Element<'_, Message> {
        pop_up!(
            "Configure Locations",
            column![
                text("Streaming URL"),
                text_input("", self.config.stream_url().unwrap_or(""))
                    .on_input(Message::NewStreamUrl),
                text("Media folder"),
                text_input(
                    "",
                    self.config
                        .media_path()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                )
                .on_input(Message::NewMediaPath),
                row![
                    button_text!("Browse")
                        .width(FillPortion(1))
                        .on_press(Message::OpenMediaPathBrowser)
                ],
                hl!(),
                row![
                    button_text!("ok")
                        .width(FillPortion(1))
                        .on_press(Message::SaveAndClosePopup)
                ]
            ]
            .padding(PADDING)
            .spacing(SPACING)
        )
        .into()
    }
}

impl super::Settings {
    pub(crate) async fn browse_media_path() -> Option<String> {
        AsyncFileDialog::new()
            .set_can_create_directories(true)
            .set_title("Choose Media Folder")
            .pick_folder()
            .await
            .and_then(|h| h.path().to_str().map(|s| s.to_string()))
    }
}
