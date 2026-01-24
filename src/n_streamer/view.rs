use iced::{
    Alignment, Element, Length,
    widget::{column, container, row},
};

use super::*;
impl NStreamer {
    pub fn view(&self) -> Element<'_, Message> {
        column![self.view_top(), self.view_center()].into()
    }
    fn view_top(&self) -> Element<'_, Message> {
        container(
            row![
                text_button("Settings"),
                text_button("Program Schedule"),
                text_button("Watch Live"),
                text_button("current program"),
                text_button("Manage Downloads"),
                text_button("Library"),
                self.clock.view()
            ]
            .spacing(SPACING)
            .align_y(Alignment::Center),
        )
        .padding(PADDING)
        .style(container::bordered_box)
        .into()
    }
    fn view_center(&self) -> Element<'_, Message> {
        container(iced::widget::text("Hello World"))
            .center(Length::Fill)
            .into()
    }
}
