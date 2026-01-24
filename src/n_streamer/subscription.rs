use std::time::Duration;

use iced::Subscription;

use super::*;
impl NStreamer {
    pub fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick);
        Subscription::batch([tick])
    }
}
