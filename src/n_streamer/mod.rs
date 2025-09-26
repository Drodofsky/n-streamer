mod message;

use iced::{Element, Task, widget::text};

use message::Message;

#[derive(Default)]
pub struct NStreamer {}

impl NStreamer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init() -> (Self, Task<Message>) {
        (Self::new(), Task::none())
    }
    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
    pub fn view(&self) -> Element<'_, Message> {
        text("Hello World!").into()
    }
}
