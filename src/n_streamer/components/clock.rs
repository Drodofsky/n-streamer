use chrono::{DateTime, Local};
use iced::Element;

use super::*;
#[derive(Debug, Default)]
pub struct Clock {
    time: DateTime<Local>,
}

impl Clock {
    pub fn view(&self) -> Element<'_, Message> {
        primary_text(self.time.time().format("%H:%M").to_string()).into()
    }
    pub fn update(&mut self) {
        self.time = Local::now();
    }
}
