use iced::{
    Alignment::Center,
    Element,
    Length::{self, FillPortion},
    widget::{Space, button, column, container, row, text},
    window::Id,
};

use crate::{
    n_streamer::{
        NStreamer,
        message::Message,
        ui_utils::{PADDING, SPACING},
    },
    pop_up, text_btn,
};

impl NStreamer {
    pub(crate) fn view_exit_popup(&self, id: Id) -> Element<'_, Message> {
        pop_up!(
            container(column![
                text("Close NStreamer"),
                row![
                    text_btn!("yes")
                        .width(Length::FillPortion(1))
                        .on_press(Message::Exit(id)),
                    text_btn!("no")
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
