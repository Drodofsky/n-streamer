use std::sync::Arc;

use super::*;
use iced::window::Id as WindowId;
use iced_video_player::Video;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Window(WindowMessage),
    Result(Result<(), Error>),
    Loaded(LoadedMessage),
    Settings(SettingsMessage),
    MenuButtonPressed(Center),
    DB(DBMessage),
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    ExitRequest(WindowId),
    Exit(WindowId),
    CloseUserInteraction,
    OnSystemThemeUpdate,
    ApplySystemTheme(iced::Theme),
}

#[derive(Debug, Clone)]
pub enum LoadedMessage {
    Settings(Result<Settings, Error>),
    LiveStream(Result<Arc<Video>, Error>),
    Schedule(Result<AnalyzedSchedule, Error>),
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    UpdateTheme(Theme),
    SettingSelected(SettingItem),
    NewStreamUrl(String),
    NewMediaPath(String),
    MaybeNewMediaPath(Option<String>),
    OpenMediaPathBrowser,
    SaveAndCloseSettings,
    SetVolume(f32),
    SetMuted(bool),
}

#[derive(Debug, Clone)]
pub enum DBMessage {
    Started(Result<Database, Error>),
    Initialized(Result<(), Error>),
}
