mod message;
mod time;
use std::time::Duration;
mod ui_utils;
use iced::{
    Alignment::Center,
    Element,
    Length::{self, Fill, FillPortion},
    Subscription, Task,
    widget::{Space, button, column, container, row, stack, text},
    window::{self, Id},
};

use message::Message;

use crate::{
    n_streamer::{
        time::Time,
        ui_utils::{PADDING, SPACING},
    },
    pop_up, text_btn,
};

#[derive(Default)]
pub struct NStreamer {
    time: Time,
    user_interaction: Option<Box<dyn Fn(&Self) -> Element<'_, Message>>>,
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
            Message::CloseRequest(id) => {
                self.user_interaction = Some(Box::new(move |s| s.view_close_hover(id)));
                Task::none()
            }
            Message::Close(id) => window::close(id),
            Message::ClosePopUp => {
                self.user_interaction = None;
                Task::none()
            }
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick);
        let close = window::close_requests().map(|id| Message::CloseRequest(id));
        Subscription::batch([tick, close])
    }
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(interaction) = &self.user_interaction {
            let mut col = column![];
            col = col.push(self.view_top());
            col = col.push(stack([self.view_center(), (interaction(&self))]));
            return col.into();
        }
        column![self.view_top(), self.view_center()].into()
    }
    fn view_menu(&self) -> Element<'_, Message> {
        button("menu").into()
    }
    fn view_top(&self) -> Element<'_, Message> {
        container(row![self.view_menu(), Space::with_width(Fill), self.time.view()].align_y(Center))
            .padding(PADDING)
            .style(container::bordered_box)
            .into()
    }
    fn view_center(&self) -> Element<'_, Message> {
        container(text("Hello World!")).center(Length::Fill).into()
    }
    fn view_close_hover(&self, id: Id) -> Element<'_, Message> {
        pop_up!(
            container(column![
                text("Close NStreamer"),
                row![
                    text_btn!("yes")
                        .width(Length::FillPortion(1))
                        .on_press(Message::Close(id)),
                    text_btn!("no")
                        .width(FillPortion(1))
                        .on_press(Message::ClosePopUp)
                ]
                .spacing(SPACING)
            ])
            .padding(PADDING)
        )
        .into()
    }
}
