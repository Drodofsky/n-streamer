use iced::{
    Alignment, Element,
    widget::{container, row},
};

use super::*;
impl NStreamer {
    pub fn view(&self) -> Element<'_, Message> {
        self.view_top()
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
}
