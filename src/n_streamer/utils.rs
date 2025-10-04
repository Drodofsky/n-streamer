use std::path::PathBuf;

use directories::ProjectDirs;

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
