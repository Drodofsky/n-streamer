mod message;

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{Space, column, container, row, text},
};

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
        column![self.view_top(), text("Hello World!"),].into()
    }
    fn view_menu(&self) -> Element<'_, Message> {
        text("menu").into()
    }
    fn view_top(&self) -> Element<'_, Message> {
        container(row![
            self.view_menu(),
            Space::with_width(Fill),
            text("19::32")
        ])
        .padding(6)
        .style(container::bordered_box)
        .into()
    }
}
