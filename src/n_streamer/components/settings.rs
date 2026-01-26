use rfd::AsyncFileDialog;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::{Path, PathBuf},
};
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
    Locations,
    Theme,
    Sound,
}

impl fmt::Display for SettingItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingItem::Exit => write!(f, "Exit"),
            SettingItem::Theme => write!(f, "Theme"),
            SettingItem::Locations => write!(f, "Locations"),
            SettingItem::Sound => write!(f, "Sound"),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    theme: Option<Theme>,
    stream_url: Option<String>,
    media_path: Option<PathBuf>,
    volume: Option<f32>,
    muted: Option<bool>,
}

impl Settings {
    pub fn view(&self) -> Element<'_, Message> {
        let options = [
            SettingItem::Sound,
            SettingItem::Locations,
            SettingItem::Theme,
            SettingItem::Exit,
        ];
        let selected: Option<SettingItem> = None;
        pick_list(options, selected, |s| {
            Message::Settings(SettingsMessage::SettingSelected(s))
        })
        .placeholder("Settings")
        .style(|theme, status| to_pick_list_style(button::primary(theme, to_button_status(status))))
        .into()
    }
    pub fn set_theme(&mut self, theme: Theme) -> Task<Message> {
        self.theme = Some(theme);
        Task::perform(Self::save(self.clone()), Message::Result)
    }
    pub fn get_theme(&mut self) -> Theme {
        self.theme.unwrap_or(Theme::System)
    }
    pub fn set_stream_url(&mut self, stream_url: String) {
        self.stream_url = Some(stream_url);
    }
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = Some(volume);
    }
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = Some(muted);
    }
    pub fn volume(&self) -> f32 {
        self.volume.unwrap_or(80.)
    }
    pub fn muted(&self) -> bool {
        self.muted.unwrap_or(false)
    }
    pub fn stream_url(&self) -> Option<&str> {
        self.stream_url.as_deref()
    }
    pub fn media_path(&self) -> Option<&Path> {
        self.media_path.as_deref()
    }
    pub fn set_media_path(&mut self, path: PathBuf) {
        self.media_path = Some(path);
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
    pub(crate) async fn browse_media_path() -> Option<String> {
        AsyncFileDialog::new()
            .set_can_create_directories(true)
            .set_title("Choose Media Folder")
            .pick_folder()
            .await
            .and_then(|h| h.path().to_str().map(|s| s.to_string()))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
}
