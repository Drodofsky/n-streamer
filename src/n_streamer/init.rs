use std::path::PathBuf;

use iced::Task;

use super::*;

impl NStreamer {
    pub fn init(project_dir: Option<ProjectDirs>) -> (Self, Task<Message>) {
        let mut n_streamer = Self::new();
        n_streamer.project_dir = project_dir;
        let settings = Task::perform(Settings::load(n_streamer.get_project_dir()), |s| {
            Message::Loaded(LoadedMessage::Settings(s))
        });

        (n_streamer, settings)
    }
    pub fn init_second_stage(&mut self) -> Task<Message> {
        if self.settings.media_path().is_none()
            || self.settings.media_path() == Some(&PathBuf::new())
        {
            self.apply_result_and(get_default_media_dir(), |s, path| {
                s.settings.set_media_path(path)
            });
        }
        self.update_theme()
    }
}
