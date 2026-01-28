use iced::{
    Alignment, Element, Length,
    widget::{column, container, row, text},
};

use super::*;
impl NStreamer {
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(interaction) = &self.get_top_user_interaction() {
            let mut col = column![];
            col = col.push(self.view_top());
            col = col.push(interaction(self));
            return col.into();
        }
        column![self.view_top(), self.view_center()].into()
    }
    fn view_top(&self) -> Element<'_, Message> {
        container(
            row![
                self.settings.view(),
                text_button("Program Schedule")
                    .on_press(Message::MenuButtonPressed(Center::ProgramSchedule)),
                text_button("Watch Live").on_press(Message::MenuButtonPressed(Center::LiveStream)),
                text_button("current program").width(Length::Fill),
                text_button("Manage Downloads")
                    .on_press(Message::MenuButtonPressed(Center::Downloads)),
                text_button("Library").on_press(Message::MenuButtonPressed(Center::Library)),
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
        let center = match self.center {
            Center::LiveStream => self.live_stream.view(),
            Center::ProgramSchedule => self.schedule.view(),
            Center::Downloads => text("Hello World!").into(),
            Center::Library => text("Hello World!").into(),
        };
        container(center).center(Length::Fill).into()
    }
}
