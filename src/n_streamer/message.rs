use super::*;
use iced::window::Id as WindowId;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Window(WindowMessage),
    Result(Result<(), Error>),
    Loaded(LoadedMessage),
    Settings(SettingsMessage),
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
}
