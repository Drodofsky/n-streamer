use directories::ProjectDirs;

use super::*;
pub fn get_project_dir() -> Result<ProjectDirs, Error> {
    ProjectDirs::from("dev", "Drodofsky", "n-streamer")
        .ok_or(Error::FileSystem("Failed to get ProjectDirs".to_string()))
}
