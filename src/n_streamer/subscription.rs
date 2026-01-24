use iced::Subscription;

use super::*;
impl NStreamer {
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([])
    }
}
