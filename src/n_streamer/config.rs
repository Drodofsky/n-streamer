use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::n_streamer::{error::Error, utils::get_project_dir};
#[derive(Default, Debug, Clone, Serialize, Deserialize)]

pub struct Config {
    stream_url: Option<String>,
}

impl Config {
    pub async fn load() -> Result<Config, Error> {
        let project_dir = get_project_dir()?;
        std::fs::create_dir_all(project_dir.preference_dir())?;
        let mut file = match File::open(project_dir.preference_dir().join("config.toml")).await {
            Ok(f) => f,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    File::create_new(project_dir.preference_dir().join("config.toml")).await?
                }
                _ => {
                    return Err(Error::from(e));
                }
            },
        };
        let mut config_str = String::new();
        file.read_to_string(&mut config_str).await?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
    pub fn stream_url(&self) -> Option<&str> {
        self.stream_url.as_deref()
    }
}
