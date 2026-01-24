use iced::{
    Element,
    Length::{self, FillPortion},
    widget::{container, row},
    window::Id,
};

use super::*;
impl NStreamer {
    pub(crate) fn view_exit_popup(&self, id: Id) -> Element<'_, Message> {
        create_pop_up(
            "Close N-Streamer".into(),
            container(
                row![
                    text_button("yes")
                        .width(Length::FillPortion(1))
                        .on_press(Message::Window(WindowMessage::Exit(id))),
                    text_button("no")
                        .width(FillPortion(1))
                        .on_press(Message::Window(WindowMessage::CloseUserInteraction))
                ]
                .spacing(SPACING)
                .padding(PADDING),
            ),
        )
        .into()
    }
}
