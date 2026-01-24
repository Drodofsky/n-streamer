use std::time::Duration;

use iced::{Subscription, system, window};

use super::*;
impl NStreamer {
    pub fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick);
        let close =
            window::close_requests().map(|id| Message::Window(WindowMessage::ExitRequest(id)));
        let theme =
            system::theme_changes().map(|_| Message::Window(WindowMessage::OnSystemThemeUpdate));
        Subscription::batch([tick, close, theme])
    }
}
