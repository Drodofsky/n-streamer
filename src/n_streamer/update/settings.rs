use super::*;
impl NStreamer {
    pub fn update_setting(&mut self, message: SettingsMessage) -> Task<Message> {
        match message {
            SettingsMessage::SettingSelected(s) => self.on_setting_selected(s),
            SettingsMessage::UpdateTheme(t) => {
                self.close_user_interaction();
                let t1 = self.settings.set_theme(t);
                let t2 = self.update_theme();
                Task::batch([t1, t2])
            }
            SettingsMessage::NewMediaPath(m) => {
                self.settings.set_media_path(m.into());
                Task::none()
            }
            SettingsMessage::NewStreamUrl(u) => {
                self.settings.set_stream_url(u);
                Task::none()
            }
            SettingsMessage::OpenMediaPathBrowser => {
                Task::perform(Settings::browse_media_path(), |p| {
                    Message::Settings(SettingsMessage::MaybeNewMediaPath(p))
                })
            }
            SettingsMessage::SaveAndCloseSettings => {
                self.close_user_interaction();
                Task::perform(Settings::save(self.settings.clone()), Message::Result)
            }
            SettingsMessage::MaybeNewMediaPath(path) => {
                if let Some(path) = path {
                    self.settings.set_media_path(path.into());
                }
                Task::none()
            }
            SettingsMessage::SetMuted(muted) => {
                self.settings.set_muted(muted);
                self.live_stream.set_muted(muted);
                Task::none()
            }
            SettingsMessage::SetVolume(volume) => {
                self.settings.set_volume(volume);
                self.live_stream.set_volume(volume);
                Task::none()
            }
        }
    }
    fn on_setting_selected(&mut self, setting_item: SettingItem) -> Task<Message> {
        match setting_item {
            SettingItem::Exit => iced::window::latest().map(|id| match id {
                Some(id) => Message::Window(WindowMessage::ExitRequest(id)),
                None => Message::Tick,
            }),
            SettingItem::Theme => {
                self.add_user_interaction(
                    Box::new(|s| s.view_theme_popup()),
                    super::Priority::Task,
                );
                Task::none()
            }
            SettingItem::Locations => {
                self.add_user_interaction(
                    Box::new(|s| s.view_locations_popup()),
                    super::Priority::Task,
                );
                Task::none()
            }
            SettingItem::Sound => {
                self.add_user_interaction(
                    Box::new(|s| s.view_sound_popup()),
                    super::Priority::Task,
                );
                Task::none()
            }
        }
    }
}
