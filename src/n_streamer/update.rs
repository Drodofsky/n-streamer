use iced::{Task, window};

use crate::n_streamer::{
    Center, NStreamer, config::Config, db, download_schedule, message::Message, settings::Settings,
    utils::get_default_media_dir,
};

impl NStreamer {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.time.update();
                let res = self.program_schedule.update_current_episode();
                self.apply_result(res);
                Task::none()
            }
            Message::LongTick => {
                if let Some(db) = &self.db {
                    let connection = db.connect();
                    Task::perform(download_schedule(connection), Message::Result)
                } else {
                    Task::none()
                }
            }
            Message::ExitRequest(id) => {
                self.user_interaction = Some(Box::new(move |s| s.view_exit_popup(id)));
                Task::none()
            }
            Message::Exit(id) => window::close(id),
            Message::ClosePopUp => {
                self.user_interaction = None;
                Task::none()
            }
            Message::SettingSelected(m) => self.apply_settings_menu(m),
            Message::NewLiveStream(live_stream) => {
                let res = self.life_stream.new_live_stream(live_stream);
                self.apply_result(res);
                Task::none()
            }
            Message::MenuButtonPressed(Center::LiveStream) => {
                self.user_interaction = None;

                if let Some(url) = self.config.stream_url() {
                    self.center = Center::LiveStream;

                    self.life_stream.live_stream_button_pressed(url)
                } else {
                    self.user_interaction = Some(Box::new(|s| {
                        s.view_error_popup(
                            "Please configure a streaming url in settings.".to_string(),
                        )
                    }));
                    Task::none()
                }
            }
            Message::MenuButtonPressed(c) => {
                self.user_interaction = None;
                self.center = c;
                Task::none()
            }
            Message::NewSchedule(schedule) => {
                let res = self.program_schedule.new_schedule(schedule);
                self.apply_result(res);

                if let Some(db) = &self.db {
                    let connection = db.connect();
                    let episodes = self.program_schedule.schedule();
                    Task::perform(db::add_episodes(connection, episodes), Message::Result)
                } else {
                    Task::none()
                }
            }
            Message::ScheduleProgramSelected(program) => {
                self.program_schedule.select_episode(program);
                Task::none()
            }
            Message::ConfigLoaded(config) => {
                self.apply_result_and(config, Self::set_config);
                if self.config.media_path().is_none() {
                    self.apply_result_and(get_default_media_dir(), |s, path| {
                        s.config.set_media_path(path)
                    });
                }
                let db = Task::perform(Self::init_db(self.config.clone()), Message::DatabaseLoaded);
                Task::batch([self.update_theme(), db])
            }
            Message::DatabaseLoaded(db) => {
                self.apply_result_and(db, Self::set_database);
                if let Some(db) = &self.db {
                    let connection = db.connect();
                    Task::perform(db::init_db(connection), Message::Result)
                } else {
                    Task::none()
                }
            }
            Message::UpdateTheme(theme) => {
                self.user_interaction = None;
                let t1 = self.config.set_theme(theme);

                let t2 = self.update_theme();
                Task::batch([t1, t2])
            }
            Message::ApplyTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::Result(result) => {
                self.apply_result(result);
                Task::none()
            }
            Message::NewStreamUrl(url) => {
                self.config.set_stream_url(url);
                Task::none()
            }
            Message::NewMediaPath(path) => {
                self.config.set_media_path(path.into());
                Task::none()
            }
            Message::MaybeNewMediaPath(path) => {
                if let Some(path) = path {
                    self.config.set_media_path(path.into());
                }
                Task::none()
            }
            Message::OpenMediaPathBrowser => {
                Task::perform(Settings::browse_media_path(), Message::MaybeNewMediaPath)
            }
            Message::SaveAndClosePopup => {
                self.user_interaction = None;
                Task::perform(Config::save(self.config.clone()), Message::Result)
            }
        }
    }
}
