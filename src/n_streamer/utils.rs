use std::{path::PathBuf, str::FromStr};

use directories::ProjectDirs;
use iced::widget::image;
use tokio::{
    fs::{File, create_dir_all},
    io::{AsyncReadExt, AsyncWriteExt},
};
use url::Url;

use crate::n_streamer::error::Error;

pub fn get_project_dir() -> Result<ProjectDirs, Error> {
    ProjectDirs::from("dev", "Drodofsky", "n-streamer")
        .ok_or(Error::FileSystem("Failed to get ProjectDirs".to_string()))
}
pub fn get_default_media_dir() -> Result<PathBuf, Error> {
    Ok(ProjectDirs::from("dev", "Drodofsky", "n-streamer")
        .ok_or(Error::FileSystem("Failed to get ProjectDirs".to_string()))?
        .data_local_dir()
        .to_owned())
}

pub async fn load_image(
    image_url: String,
    base_path: Option<PathBuf>,
) -> Result<Option<image::Handle>, Error> {
    if image_url.is_empty() {
        return Ok(None);
    }
    let path = base_path.unwrap_or(get_default_media_dir()?);
    let path = path.join(image_url.strip_prefix("/").unwrap_or(&image_url));
    let mut file = match File::open(path).await {
        Ok(f) => f,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Ok(None),
            _ => return Err(e.into()),
        },
    };
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).await?;
    let handle = image::Handle::from_bytes(bytes);
    Ok(Some(handle))
}

pub async fn download_image_if_not_exists(
    url: &str,
    base_path: Option<PathBuf>,
) -> Result<(), Error> {
    if url.is_empty() {
        return Ok(());
    }
    let path = base_path.unwrap_or(get_default_media_dir()?);
    let path = path.join(url.strip_prefix("/").unwrap_or(url));
    let uri = Url::from_str("https://nhkworldpremium.com")?;
    let uri = uri.join(url)?;
    if !path.exists() {
        let image = reqwest::get(uri).await?.bytes().await?;
        if let Some(parent) = path.parent() {
            create_dir_all(parent).await?;
        }
        let mut f = File::create_new(path).await?;
        f.write_all(&image).await?;
    }

    Ok(())
}
