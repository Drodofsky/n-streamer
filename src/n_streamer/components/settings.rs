use std::fmt;

pub use super::*;
use iced::{
    Element, Task,
    widget::{button, pick_list},
    window,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingItem {
    Exit,
}

impl fmt::Display for SettingItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingItem::Exit => write!(f, "Exit"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Settings;

impl Settings {
    pub fn view(&self) -> Element<'_, Message> {
        let options = [SettingItem::Exit];
        let selected: Option<SettingItem> = None;
        pick_list(options, selected, Message::SettingSelected)
            .placeholder("Settings")
            .style(|theme, status| {
                to_pick_list_style(button::primary(theme, to_button_status(status)))
            })
            .into()
    }
}

impl Settings {
    pub(crate) fn update(&mut self, setting_item: SettingItem) -> Task<Message> {
        match setting_item {
            SettingItem::Exit => window::latest().map(|id| match id {
                Some(id) => Message::Window(WindowMessage::ExitRequest(id)),
                None => Message::Tick,
            }),
        }
    }
}
