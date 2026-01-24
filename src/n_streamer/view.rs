use iced::Element;

use super::*;
impl NStreamer {
    pub fn view(&self) -> Element<'_, Message> {
        self.clock.view()
    }
}
