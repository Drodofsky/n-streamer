mod message;
mod time;
use std::time::Duration;

use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    Subscription, Task,
    widget::{Space, button, column, container, row, text},
};

use message::Message;

use crate::n_streamer::time::Time;

#[derive(Default)]
pub struct NStreamer {
    time: Time,
}

impl NStreamer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init() -> (Self, Task<Message>) {
        (Self::new(), Task::none())
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.time.update();
                Task::none()
            }
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick)
    }
    pub fn view(&self) -> Element<'_, Message> {
        column![self.view_top(), text("Hello World!"),].into()
    }
    fn view_menu(&self) -> Element<'_, Message> {
        button("menu").into()
    }
    fn view_top(&self) -> Element<'_, Message> {
        container(row![self.view_menu(), Space::with_width(Fill), self.time.view()].align_y(Center))
            .padding(6)
            .style(container::bordered_box)
            .into()
    }
}
