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
    Plus(ScrollListOrigin, ScheduleView),
    ListElementEntered(ScrollListOrigin, usize),
}

#[derive(Debug, Clone)]
pub enum LoadedMessage {
    Settings(Result<Settings, Error>),
    LiveStream(Result<Arc<Video>, Error>),
    Schedule(Result<AnalyzedSchedule, Error>),
    ScheduleView(Result<Vec<ScheduleView>, Error>),
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
    EpisodesAdded(Result<(), Error>),
}

impl ScrollListMessage<ScheduleView> for Message {
    fn plus(owner: ScrollListOrigin, item: ScheduleView) -> Self {
        Message::Window(WindowMessage::Plus(owner, item))
    }
    fn list_element_entered(owner: ScrollListOrigin, id: usize) -> Self {
        Message::Window(WindowMessage::ListElementEntered(owner, id))
    }
}
