use chrono::{DateTime, Local};
use iced::{
    Background, Color, Element,
    widget::{
        self,
        button::{self, Status},
        container::transparent,
        text::Style,
    },
};

use crate::{
    n_streamer::{message::Message, ui_utils::PADDING},
    primary_text,
};

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
