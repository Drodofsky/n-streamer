mod exit;
use std::fmt;

use iced::{Background, Element, Task, widget::pick_list, window};

use crate::n_streamer::{NStreamer, message::Message};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingItem {
    Exit,
    Todo,
}

impl fmt::Display for SettingItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingItem::Exit => write!(f, "Exit"),
            SettingItem::Todo => write!(f, "TODO"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Settings;

impl Settings {
    pub fn view(&self) -> Element<'_, Message> {
        let options = [SettingItem::Exit, SettingItem::Todo];
        let selected: Option<SettingItem> = None;
        pick_list(options, selected, Message::SettingSelected)
            .placeholder("Settings")
            .style(|theme, status| {
                let mut p = pick_list::default(theme, status);
                p.placeholder_color = theme.palette().text;
                p.background = Background::Color(theme.palette().primary);
                p
            })
            .into()
    }
}

impl NStreamer {
    pub(crate) fn apply_settings_menu(&mut self, setting_item: SettingItem) -> Task<Message> {
        match setting_item {
            SettingItem::Exit => window::get_latest().map(|id| match id {
                Some(id) => Message::ExitRequest(id),
                None => Message::Tick,
            }),
            SettingItem::Todo => {
                todo!()
            }
        }
    }
}
