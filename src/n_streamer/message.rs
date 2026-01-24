use super::*;
use iced::window::Id as WindowId;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Window(WindowMessage),
    SettingSelected(SettingItem),
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    ExitRequest(WindowId),
    Exit(WindowId),
    CloseUserInteraction,
    UpdateTheme(Theme),
    OnSystemThemeUpdate,
    ApplyTheme(iced::Theme),
}
