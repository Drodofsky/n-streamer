use super::*;
use iced::window::Id as WindowId;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Window(WindowMessage),
    SettingSelected(SettingItem),
    Result(Result<(), Error>),
    Loaded(LoadedMessage),
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    ExitRequest(WindowId),
    Exit(WindowId),
    CloseUserInteraction,
    UpdateTheme(Theme),
    OnSystemThemeUpdate,
    ApplySystemTheme(iced::Theme),
}

#[derive(Debug, Clone)]
pub enum LoadedMessage {
    Settings(Result<Settings, Error>),
}
