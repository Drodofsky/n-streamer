use std::fmt;

pub use super::*;
use iced::{
    Element, Task,
    widget::{button, pick_list},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingItem {
    Exit,
    Theme,
}

impl fmt::Display for SettingItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingItem::Exit => write!(f, "Exit"),
            SettingItem::Theme => write!(f, "Theme"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Settings {
    theme: Option<Theme>,
}

impl Settings {
    pub fn view(&self) -> Element<'_, Message> {
        let options = [SettingItem::Theme, SettingItem::Exit];
        let selected: Option<SettingItem> = None;
        pick_list(options, selected, Message::SettingSelected)
            .placeholder("Settings")
            .style(|theme, status| {
                to_pick_list_style(button::primary(theme, to_button_status(status)))
            })
            .into()
    }
    pub fn set_theme(&mut self, theme: Theme) -> Task<Message> {
        self.theme = Some(theme);
        Task::none()
    }
    pub fn get_theme(&mut self) -> Theme {
        self.theme.unwrap_or(Theme::System)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
    System,
}
