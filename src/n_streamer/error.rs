use std::fmt;

use super::*;

#[derive(Debug, Clone)]
pub enum Error {
    FileSystem(String),
    IO(String),
    Config(String),
    Url(String),
    VideoPlayer(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileSystem(e) => {
                write!(f, "file system: {}", e)
            }
            Self::IO(e) => {
                write!(f, "IO: {}", e)
            }
            Self::Config(e) => {
                write!(f, "Config: {}", e)
            }
            Self::Url(e) => {
                write!(f, "URL: {}", e)
            }
            Self::VideoPlayer(e) => {
                write!(f, "video player: {}", e)
            }
        }
    }
}

impl NStreamer {
    pub(crate) fn apply_result_and_return<T, R>(
        &mut self,
        res: Result<T, Error>,
        return_val: R,
    ) -> R {
        self.apply_result(res);
        return_val
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

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value.to_string())
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

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value.to_string())
    }
}

impl From<iced_video_player::Error> for Error {
    fn from(value: iced_video_player::Error) -> Self {
        Self::VideoPlayer(value.to_string())
    }
}
