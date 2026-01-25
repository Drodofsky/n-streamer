use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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
        Task::perform(Self::save(self.clone()), Message::Result)
    }
    pub fn get_theme(&mut self) -> Theme {
        self.theme.unwrap_or(Theme::System)
    }
    pub async fn load() -> Result<Settings, Error> {
        let project_dir = get_project_dir()?;
        std::fs::create_dir_all(project_dir.preference_dir())?;
        let mut file = match File::open(project_dir.preference_dir().join("config.toml")).await {
            Ok(f) => f,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    File::create_new(project_dir.preference_dir().join("config.toml")).await?
                }
                _ => {
                    return Err(Error::from(e));
                }
            },
        };
        let mut config_str = String::new();
        file.read_to_string(&mut config_str).await?;
        let settings: Settings = toml::from_str(&config_str)?;
        Ok(settings)
    }
    pub async fn save(self) -> Result<(), Error> {
        let project_dir = get_project_dir()?;
        let mut file = File::create(project_dir.preference_dir().join("config.toml")).await?;
        let config_str = toml::to_string_pretty(&self)?;
        file.write_all(config_str.as_bytes()).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
}
