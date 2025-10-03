use iced::{
    Element,
    Length::{self, FillPortion},
    widget::{column, container, row, text},
    window::Id,
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
    pub(crate) fn view_exit_popup(&self, id: Id) -> Element<'_, Message> {
        pop_up!(
            container(column![
                text("Close NStreamer"),
                row![
                    button_text!("yes")
                        .width(Length::FillPortion(1))
                        .on_press(Message::Exit(id)),
                    button_text!("no")
                        .width(FillPortion(1))
                        .on_press(Message::ClosePopUp)
                ]
                .spacing(SPACING)
            ])
            .padding(PADDING)
        )
        .into()
    }
}
