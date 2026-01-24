use iced::window::Id as WindowId;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Window(WindowMessage),
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    ExitRequest(WindowId),
    Exit(WindowId),
    CloseUserInteraction,
}
