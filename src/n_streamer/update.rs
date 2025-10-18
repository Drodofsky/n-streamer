use iced::{Task, window};

use crate::n_streamer::{
    Center, NStreamer, config::Config, db, download_schedule, message::Message, settings::Settings,
    utils::get_default_media_dir,
};

impl NStreamer {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                let mut tasks = Vec::new();
                self.time.update();
                let res = self
                    .program_schedule
                    .update(self.config.media_path().map(|p| p.to_path_buf()));
                self.apply_result_and(res, |_, t| {
                    if let Some(t) = t {
                        tasks.push(t);
                    }
                });

                let res = self.title.update();
                self.apply_result_and(res, |_, t| {
                    if let Some(t) = t {
                        tasks.push(t);
                    }
                });

                Task::batch(tasks)
            }
            Message::LongTick => {
                if let Some(db) = &self.db {
                    let connection = db.connect();
                    Task::perform(
                        download_schedule(
                            connection,
                            self.config.media_path().map(|p| p.to_path_buf()),
                        ),
                        Message::Result,
                    )
                } else {
                    Task::none()
                }
            }
            Message::LoadedEpisodes(e) => {
                self.apply_result_and(e, |s, e| s.program_schedule.set_schedule(e));
                Task::none()
            }
            Message::ScheduleElementEntered(id) => {
                self.program_schedule.set_hovered_episode(id);
                Task::none()
            }
            Message::CurrentEpisode(e) => {
                self.apply_result_and(e, |s, e| s.title.set_current_episode(e));
                Task::none()
            }
            Message::ExitRequest(id) => {
                self.add_user_interaction(
                    Box::new(move |s| s.view_exit_popup(id)),
                    super::Priority::Exit,
                );
                Task::none()
            }
            Message::Exit(id) => window::close(id),
            Message::ClosePopUp => {
                self.close_user_interaction();
                Task::none()
            }
            Message::SettingSelected(m) => self.apply_settings_menu(m),
            Message::NewLiveStream(live_stream) => {
                let res = self.life_stream.new_live_stream(live_stream);
                self.apply_result(res);
                Task::none()
            }
            Message::MenuButtonPressed(Center::LiveStream) => {
                self.clear_user_interaction();

                if let Some(url) = self.config.stream_url() {
                    self.center = Center::LiveStream;

                    self.life_stream.live_stream_button_pressed(url)
                } else {
                    self.add_user_interaction(
                        Box::new(|s| {
                            s.view_error_popup(
                                "Please configure a streaming url in settings.".to_string(),
                            )
                        }),
                        super::Priority::Error,
                    );
                    Task::none()
                }
            }
            Message::MenuButtonPressed(c) => {
                self.clear_user_interaction();
                self.center = c;
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
                    let connection1 = db.connect();
                    let connection2 = db.connect();

                    Task::perform(db::init_db(connection1), Message::DbInitialized).chain(
                        Task::perform(
                            download_schedule(
                                connection2,
                                self.config.media_path().map(|p| p.to_path_buf()),
                            ),
                            Message::Result,
                        ),
                    )
                } else {
                    Task::none()
                }
            }
            Message::DbInitialized(result) => {
                self.apply_result(result);
                if let Some(db) = &self.db {
                    let connection = db.connect().map_err(|e| e.into());
                    self.apply_result_and(connection, |s, con| {
                        s.program_schedule.set_connectoin(con.clone());
                        s.title.set_connectoin(con);
                    });
                }
                Task::none()
            }
            Message::UpdateTheme(theme) => {
                self.close_user_interaction();
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
                self.close_user_interaction();
                Task::perform(Config::save(self.config.clone()), Message::Result)
            }
            Message::LoadImage(url, image) => {
                self.apply_result_and(image, |s, image| {
                    s.program_schedule.add_image(url.clone(), image)
                });
                Task::none()
            }
            Message::Plus(_episode_view) => {
                self.add_user_interaction(
                    Box::new(move |s| s.view_download_popup()),
                    super::Priority::Task,
                );
                Task::none()
            }
        }
    }
}
