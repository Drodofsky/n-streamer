use std::fmt;

use iced::{Element, Length::FillPortion, widget::row};

use crate::{
    button_text,
    n_streamer::{NStreamer, message::Message, ui_utils::PADDING},
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
    Config(String),
    Database(String),
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
        Self::Config(value.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self::Config(value.to_string())
    }
}

impl From<iced_video_player::Error> for Error {
    fn from(value: iced_video_player::Error) -> Self {
        Self::VideoPlayer(value.to_string())
    }
}

impl From<turso::Error> for Error {
    fn from(value: turso::Error) -> Self {
        Self::Database(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value.to_string())
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
            Self::Config(e) => {
                write!(f, "Config: {}", e)
            }
            Self::Database(e) => {
                write!(f, "Database: {}", e)
            }
        }
    }
}

impl NStreamer {
    pub(crate) fn view_error_popup(&self, message: String) -> Element<'_, Message> {
        pop_up!(
            message.to_string(),
            row![
                button_text!("ok")
                    .style(iced::widget::button::danger)
                    .width(FillPortion(1))
                    .on_press(Message::ClosePopUp)
            ]
            .padding(PADDING)
        )
        .into()
    }
    pub(crate) fn apply_result<T>(&mut self, res: Result<T, Error>) {
        match res {
            Ok(_) => {}
            Err(e) => {
                self.add_user_interaction(
                    Box::new(move |s| s.view_error_popup(e.to_string())),
                    super::Priority::Error,
                );
            }
        }
    }
      pub(crate) fn apply_result_and_defualt<T:Default>(&mut self, res: Result<T, Error>)->T {
        match res {
            Ok(o) => {o}
            Err(e) => {
                self.add_user_interaction(
                    Box::new(move |s| s.view_error_popup(e.to_string())),
                    super::Priority::Error,
                );
                T::default()
            }
        }
    }
    pub(crate) fn apply_result_and<T>(
        &mut self,
        res: Result<T, Error>,
        mut f: impl FnMut(&mut Self, T),
    ) {
        match res {
            Ok(x) => f(self, x),
            Err(e) => {
                self.add_user_interaction(
                    Box::new(move |s| s.view_error_popup(e.to_string())),
                    super::Priority::Error,
                );
            }
        }
    }
}
