use super::*;
impl NStreamer {
    pub(crate) fn update_settings(&mut self, setting_item: SettingItem) -> Task<Message> {
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
        }
    }
}
