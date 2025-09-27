use iced::window::Id;

use crate::n_streamer::menu::MenuItem;
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    CloseRequest(Id),
    ClosePopUp,
    Close(Id),
    MenuSelected(MenuItem),
}
