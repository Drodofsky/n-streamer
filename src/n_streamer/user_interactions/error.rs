use iced::{
    Element,
    Length::FillPortion,
    widget::{Id, container, row},
};

pub use super::*;

impl NStreamer {
    pub(crate) fn view_error_popup(&self, message: String, id: Id) -> Element<'_, Message> {
        create_pop_up(
            message.to_string(),
            container(
                row![
                    text_button("ok")
                        .style(iced::widget::button::danger)
                        .width(FillPortion(1))
                        .on_press(Message::Window(WindowMessage::CloseUserInteraction))
                ]
                .padding(PADDING),
            )
            .id(id),
        )
        .into()
    }
}
