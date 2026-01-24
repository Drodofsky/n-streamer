use iced::Element;

use super::*;
impl NStreamer {
    pub fn view(&self) -> Element<'_, Message> {
        iced::widget::text("Hello world").into()
    }
}
