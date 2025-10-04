use std::sync::Arc;

use iced::window::Id;
use iced_video_player::Video;
use turso::Database;

use crate::n_streamer::{
    Center,
    config::{Config, Theme},
    error::Error,
    program_schedule::analyzed_schedule::AnalyzedEpisode,
    settings::SettingItem,
};
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    LongTick,
    ExitRequest(Id),
    ClosePopUp,
    Exit(Id),
    SettingSelected(SettingItem),
    NewLiveStream(Result<Arc<Video>, Error>),
    MenuButtonPressed(Center),
    ConfigLoaded(Result<Config, Error>),
    UpdateTheme(Theme),
    ApplyTheme(iced::Theme),
    NewStreamUrl(String),
    NewMediaPath(String),
    MaybeNewMediaPath(Option<String>),
    OpenMediaPathBrowser,
    SaveAndClosePopup,
    DatabaseLoaded(Result<Database, Error>),
    Result(Result<(), Error>),
    DbInitialized(Result<(), Error>),
    LoadedEpisodes(Result<Vec<AnalyzedEpisode>, Error>),
    CurrentEpisode(Result<Option<AnalyzedEpisode>, Error>),
}
