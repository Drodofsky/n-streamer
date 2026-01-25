mod loaded;
mod settings;
mod theme;
mod window;
pub use super::*;
use iced::Task;

impl NStreamer {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.clock.update();
                Task::none()
            }
            Message::Window(wm) => self.update_window(wm),
            Message::Result(res) => self.apply_result_and_return(res, Task::none()),
            Message::Loaded(l) => self.loaded(l),
            Message::Settings(s) => self.update_setting(s),
            Message::MenuButtonPressed(Center::LiveStream) => {
                self.clear_user_interaction();

                if let Some(url) = self.settings.stream_url() {
                    self.center = Center::LiveStream;

                    self.live_stream.live_stream_button_pressed(url)
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
        }
    }
}
