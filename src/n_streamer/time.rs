use chrono::{DateTime, Local};
use iced::{
    Element, Length,
    widget::{container, text, text::LineHeight},
};

use crate::n_streamer::message::Message;

#[derive(Debug, Default)]
pub struct Time {
    time: DateTime<Local>,
}

impl Time {
    pub fn view(&self) -> Element<'_, Message> {
        container(
            text(self.time.time().format("%H:%M").to_string())
                .style(|theme| text::base(theme))
                .width(Length::Shrink)
                .wrapping(text::Wrapping::None)
                .line_height(LineHeight::Relative(1.0)),
        )
        .into()
    }
    pub fn update(&mut self) {
        self.time = Local::now();
    }
}
