use chrono::{DateTime, Local};
use iced::{
    Element,
    widget::{container, text},
};

use crate::n_streamer::message::Message;

#[derive(Debug, Default)]
pub struct Time {
    time: DateTime<Local>,
}

impl Time {
    pub fn view(&self) -> Element<'_, Message> {
        container(text(self.time.time().format("%H:%M").to_string())).into()
    }
    pub fn update(&mut self) {
        self.time = Local::now();
    }
}
