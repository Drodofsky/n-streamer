use std::path::PathBuf;

use directories::ProjectDirs;

use super::*;

impl NStreamer {
    pub fn get_project_dir(&self) -> Result<ProjectDirs, Error> {
        if let Some(project_dir) = &self.project_dir {
            return Ok(project_dir.clone());
        }
        ProjectDirs::from("dev", "Drodofsky", "n-streamer")
            .ok_or(Error::FileSystem("Failed to get ProjectDirs".to_string()))
    }

    pub fn get_default_media_dir(&self) -> Result<PathBuf, Error> {
        if let Some(project_dir) = &self.project_dir {
            return Ok(project_dir.data_local_dir().to_owned());
        }
        Ok(ProjectDirs::from("dev", "Drodofsky", "n-streamer")
            .ok_or(Error::FileSystem("Failed to get ProjectDirs".to_string()))?
            .data_local_dir()
            .to_owned())
    }
}
