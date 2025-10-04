use std::sync::Arc;

use iced::window::Id;
use iced_video_player::Video;

use crate::n_streamer::{
    Center,
    config::{Config, Theme},
    error::Error,
    program_schedule::{analyzed_schedule::AnalyzedEpisode, parsed_schedule::Schedule},
    settings::SettingItem,
};
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    ExitRequest(Id),
    ClosePopUp,
    Exit(Id),
    SettingSelected(SettingItem),
    NewLiveStream(Result<Arc<Video>, Error>),
    MenuButtonPressed(Center),
    ScheduleProgramSelected(AnalyzedEpisode),
    NewSchedule(Result<Schedule, Error>),
    ConfigLoaded(Result<Config, Error>),
    UpdateTheme(Theme),
    ApplyTheme(iced::Theme),
    Saved(Result<(), Error>),
    NewStreamUrl(String),
    NewMediaPath(String),
    MaybeNewMediaPath(Option<String>),
    OpenMediaPathBrowser,
    SaveAndClosePopup,
}
