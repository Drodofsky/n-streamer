use std::sync::Arc;

use iced::window::Id;
use iced_video_player::Video;

use crate::n_streamer::{error::Error, settings::SettingItem};
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    ExitRequest(Id),
    ClosePopUp,
    Exit(Id),
    SettingSelected(SettingItem),
    NewLiveStream(Result<Arc<Video>, Error>),
    WatchLive,
}
