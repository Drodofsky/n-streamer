use iced::{
    Element,
    Length::{self, FillPortion},
    widget::row,
};

use crate::{
    button_text,
    n_streamer::{
        NStreamer,
        message::Message,
        ui_utils::{PADDING, SPACING},
    },
    pop_up,
};

impl NStreamer {
    pub(crate) fn view_download_popup(&self) -> Element<'_, Message> {
        pop_up!(
            "Add episode to download queue",
            row![
                button_text!("yes").width(Length::FillPortion(1)),
                button_text!("Subscribe").width(Length::FillPortion(1)),
                button_text!("Close")
                    .width(FillPortion(1))
                    .on_press(Message::ClosePopUp)
            ]
            .spacing(SPACING)
            .padding(PADDING)
        )
        .into()
    }
}
