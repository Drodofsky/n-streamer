use std::fmt;

use iced::{
    Element,
    Length::FillPortion,
    widget::{column, container, text},
};

use crate::{
    button_text,
    n_streamer::{NStreamer, message::Message, ui_utils::BIG_PADDING},
    pop_up,
};

#[derive(Debug, Clone)]
pub enum Error {
    Api(String),
    Chrono(String),
    Url(String),
    VideoPlayer(String),
    FileSystem(String),
    IO(String),
    Download(String),
    ConfigParsing(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Api(value.to_string())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(value: chrono::ParseError) -> Self {
        Self::Chrono(value.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::ConfigParsing(value.to_string())
    }
}

impl From<iced_video_player::Error> for Error {
    fn from(value: iced_video_player::Error) -> Self {
        Self::VideoPlayer(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(format!("{:#?}", value.kind()))
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api(e) => {
                write!(f, "api: {}", e)
            }
            Self::Chrono(e) => {
                write!(f, "chrono: {}", e)
            }
            Self::Url(e) => {
                write!(f, "URL: {}", e)
            }
            Self::VideoPlayer(e) => {
                write!(f, "video player: {}", e)
            }
            Self::FileSystem(e) => {
                write!(f, "file system: {}", e)
            }
            Self::IO(e) => {
                write!(f, "IO: {}", e)
            }
            Self::Download(e) => {
                write!(f, "Download: {}", e)
            }
            Self::ConfigParsing(e) => {
                write!(f, "Config Parsing: {}", e)
            }
        }
    }
}

impl NStreamer {
    pub(crate) fn view_error_popup(&self, message: &str) -> Element<'_, Message> {
        pop_up!(
            container(column![
                text!("{}", message),
                button_text!("ok")
                    .width(FillPortion(1))
                    .on_press(Message::ClosePopUp)
            ])
            .padding(BIG_PADDING)
        )
        .into()
    }
}
