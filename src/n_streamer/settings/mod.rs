mod exit;
mod locations;
mod theme;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::pick_list::Status as PickListStatus;
use iced::widget::pick_list::Style as PickListStyle;
use iced::{
    Background, Color, Element, Task,
    widget::{button, pick_list},
    window,
};
use std::fmt;

use crate::n_streamer::{NStreamer, message::Message};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingItem {
    Exit,
    Locations,
    Theme,
}

impl fmt::Display for SettingItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingItem::Exit => write!(f, "Exit"),
            SettingItem::Theme => write!(f, "Theme"),
            SettingItem::Locations => write!(f, "Locations"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Settings;

impl Settings {
    pub fn view(&self) -> Element<'_, Message> {
        let options = [
            SettingItem::Locations,
            SettingItem::Theme,
            SettingItem::Exit,
        ];
        let selected: Option<SettingItem> = None;
        pick_list(options, selected, Message::SettingSelected)
            .placeholder("Settings")
            .style(|theme, status| {
                to_pick_list_style(button::primary(theme, to_button_status(status)))
            })
            .into()
    }
}
fn to_pick_list_style(button_style: iced::widget::button::Style) -> PickListStyle {
    PickListStyle {
        text_color: button_style.text_color,
        placeholder_color: button_style.text_color,
        background: button_style
            .background
            .unwrap_or(Background::Color(Color::default())),
        border: button_style.border,
        handle_color: button_style.text_color,
    }
}

fn to_button_status(status: PickListStatus) -> ButtonStatus {
    match status {
        PickListStatus::Active => ButtonStatus::Active,
        PickListStatus::Hovered => ButtonStatus::Hovered,
        PickListStatus::Opened { .. } => ButtonStatus::Pressed,
    }
}

impl NStreamer {
    pub(crate) fn apply_settings_menu(&mut self, setting_item: SettingItem) -> Task<Message> {
        match setting_item {
            SettingItem::Exit => window::latest().map(|id| match id {
                Some(id) => Message::ExitRequest(id),
                None => Message::Tick,
            }),
            SettingItem::Locations => {
                self.user_interaction = Some(Box::new(|s| s.view_locations_popup()));
                Task::none()
            }
            SettingItem::Theme => {
                self.user_interaction = Some(Box::new(|s| s.view_theme_popup()));
                Task::none()
            }
        }
    }
}
