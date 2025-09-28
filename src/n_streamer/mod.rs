mod live_stream;
mod message;
mod settings;
mod time;
use std::time::Duration;
mod error;
mod ui_utils;
use crate::{
    button_text,
    n_streamer::{live_stream::LiveStream, ui_utils::SPACING},
    primary_text,
};
use iced::{
    Alignment, Element,
    Length::{self, Fill},
    Subscription, Task,
    widget::{column, container, row, stack, text},
    window,
};
use iced::{
    Background, Color,
    widget::{
        self,
        button::{self, Status},
        container::transparent,
        text::Style,
    },
};

use message::Message;

use crate::n_streamer::{
    settings::Settings,
    time::Time,
    ui_utils::{DynView, PADDING},
};

#[derive(Default)]
pub struct NStreamer {
    settings: Settings,
    time: Time,
    user_interaction: Option<DynView<Self, Message>>,
    life_stream: LiveStream,
    center: Center,
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
            Message::ExitRequest(id) => {
                self.user_interaction = Some(Box::new(move |s| s.view_exit_popup(id)));
                Task::none()
            }
            Message::Exit(id) => window::close(id),
            Message::ClosePopUp => {
                self.user_interaction = None;
                Task::none()
            }
            Message::SettingSelected(m) => self.apply_settings_menu(m),
            Message::NewLiveStream(live_stream) => {
                self.life_stream.new_live_stream(live_stream);
                Task::none()
            }
            Message::WatchLive => {
                self.center = Center::LiveStream;
                self.life_stream.live_stream_button_pressed()
            }
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick);
        let close = window::close_requests().map(Message::ExitRequest);
        Subscription::batch([tick, close])
    }
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(interaction) = &self.user_interaction {
            let mut col = column![];
            col = col.push(self.view_top());
            col = col.push(stack([self.view_center(), (interaction(self))]));
            return col.into();
        }
        column![self.view_top(), self.view_center()].into()
    }

    fn view_top(&self) -> Element<'_, Message> {
        container(
            row![
                self.settings.view(),
                button_text!("Program Schedule").on_press(Message::Tick),
                button_text!("Watch Live").on_press(Message::WatchLive),
                primary_text!("City-Scope").width(Fill),
                button_text!("Manage Downloads").on_press(Message::Tick),
                button_text!("Library").on_press(Message::Tick),
                self.time.view()
            ]
            .spacing(SPACING)
            .align_y(Alignment::Center),
        )
        .padding(PADDING)
        .style(container::bordered_box)
        .into()
    }
    fn view_center(&self) -> Element<'_, Message> {
        let center = match self.center {
            Center::LiveStream => self.life_stream.view(),
            _ => text("Hello World!").into(),
        };
        container(center).center(Length::Fill).into()
    }
}

#[derive(Debug, Default)]
enum Center {
    #[default]
    ProgramSchedule,
    LiveStream,
    Downloads,
    Library,
}
