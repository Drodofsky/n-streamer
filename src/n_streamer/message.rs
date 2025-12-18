use std::sync::Arc;

use iced::{widget::image, window::Id};
use iced_video_player::Video;
use turso::Database;

use crate::n_streamer::{
    Center,
    config::{Config, Theme},
    db::{EpisodeId, EpisodeView},
    error::Error,
    program_schedule::analyzed_schedule::AnalyzedEpisode,
    settings::SettingItem,
    ui_utils::{ScrollListMessage, ScrollListOwner},
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
    LoadedEpisodes(Result<Vec<EpisodeView>, Error>),
    ListElementEntered(ScrollListOwner, usize),
    CurrentEpisode(Result<Option<AnalyzedEpisode>, Error>),
    LoadImage(String, Result<Option<image::Handle>, Error>),
    Plus(ScrollListOwner, EpisodeView),
    AddVideoToDownloadQueue(EpisodeId),
    LoadedDownloadQueue(Result<Vec<EpisodeView>, Error>),
    RemoveEpisodeFromDownloadQueue(EpisodeId),
}

impl ScrollListMessage<EpisodeView> for Message {
    fn plus(owner: ScrollListOwner, item: EpisodeView) -> Self {
        Message::Plus(owner, item)
    }
    fn list_element_entered(owner: ScrollListOwner, id: usize) -> Self {
        Message::ListElementEntered(owner, id)
    }
}
