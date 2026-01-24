use super::*;

impl NStreamer {
    pub fn update_window(&mut self, message: WindowMessage) -> Task<Message> {
        match message {
            WindowMessage::ExitRequest(id) => {
                if self
                    .user_interactions
                    .iter()
                    .any(|u| u.priority() == Priority::Exit)
                {
                    return Task::none();
                }
                self.add_user_interaction(
                    Box::new(move |s| s.view_exit_popup(id)),
                    super::Priority::Exit,
                );
                Task::none()
            }
            WindowMessage::Exit(id) => iced::window::close(id),
            WindowMessage::CloseUserInteraction => {
                self.close_user_interaction();
                Task::none()
            }
            WindowMessage::UpdateTheme(theme) => {
                self.close_user_interaction();
                let t1 = self.settings.set_theme(theme);
                let t2 = self.update_theme();
                Task::batch([t1, t2])
            }
            WindowMessage::ApplyTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
            WindowMessage::OnSystemThemeUpdate => self.update_theme(),
        }
    }
}
