use chrono::{DateTime, Local};
use iced::Element;

use crate::{n_streamer::message::Message, primary_text};

#[derive(Debug, Default)]
pub struct Time {
    time: DateTime<Local>,
}

impl Time {
    pub fn view(&self) -> Element<'_, Message> {
        primary_text!(self.time.time().format("%H:%M").to_string()).into()
    }
    pub fn update(&mut self) {
        self.time = Local::now();
    }
}
