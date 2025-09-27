use iced::window::Id;
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    CloseRequest(Id),
    ClosePopUp,
    Close(Id),
}
