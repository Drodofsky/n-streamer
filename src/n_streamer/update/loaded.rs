use super::*;

impl NStreamer {
    pub fn loaded(&mut self, loaded: LoadedMessage) -> Task<Message> {
        match loaded {
            LoadedMessage::Settings(s) => self.loaded_settings(s),
        }
    }
    fn loaded_settings(&mut self, settings: Result<Settings, Error>) -> Task<Message> {
        self.apply_result_and(settings, |this, s| this.settings = s);
        self.init_second_stage()
    }
}
