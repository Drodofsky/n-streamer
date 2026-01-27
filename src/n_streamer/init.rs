use std::path::PathBuf;

use iced::Task;

use crate::n_streamer::db::start_db;

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
        // init media path
        if self.settings.media_path().is_none()
            || self.settings.media_path() == Some(&PathBuf::new())
        {
            self.apply_result_and(get_default_media_dir(), |s, path| {
                s.settings.set_media_path(path)
            });
        }
        let t1 = self.update_theme();
        let t2 = Task::perform(start_db(self.settings.clone()), |db| {
            Message::DB(DBMessage::Started(db))
        });
        Task::batch([t1, t2])
    }
}
