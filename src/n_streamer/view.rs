use crate::{
    button_text,
    n_streamer::{
        Center, NStreamer,
        message::Message,
        ui_utils::{PADDING, SPACING},
    },
    primary_text,
};
use iced::{
    Alignment, Element,
    Length::{self, Fill},
    widget::{column, container, row, text},
};
impl NStreamer {
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(interaction) = &self.user_interaction {
            let mut col = column![];
            col = col.push(self.view_top());
            col = col.push(interaction(self));
            return col.into();
        }
        column![self.view_top(), self.view_center()].into()
    }
    fn view_current_program(&self) -> Element<'_, Message> {
        self.title
            .get_current_episode()
            .map(|e| primary_text!(e))
            .unwrap_or(primary_text!(""))
            .width(Fill)
            .into()
    }

    fn view_top(&self) -> Element<'_, Message> {
        container(
            row![
                self.settings.view(),
                button_text!("Program Schedule")
                    .on_press(Message::MenuButtonPressed(Center::ProgramSchedule)),
                button_text!("Watch Live").on_press(Message::MenuButtonPressed(Center::LiveStream)),
                self.view_current_program(),
                button_text!("Manage Downloads")
                    .on_press(Message::MenuButtonPressed(Center::Downloads)),
                button_text!("Library").on_press(Message::MenuButtonPressed(Center::Library)),
                self.time.view()
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
            Center::LiveStream => self.life_stream.view(),
            Center::ProgramSchedule => self.program_schedule.view(),
            _ => text("Hello World!").into(),
        };
        container(center).center(Length::Fill).into()
    }
}
