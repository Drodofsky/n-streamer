use iced::window::Id;

use crate::n_streamer::settings::SettingItem;
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    CloseRequest(Id),
    ClosePopUp,
    Close(Id),
    SettingSelected(SettingItem),
}
