use iced::window::Id;

use crate::n_streamer::settings::SettingItem;
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    ExitRequest(Id),
    ClosePopUp,
    Exit(Id),
    SettingSelected(SettingItem),
}
