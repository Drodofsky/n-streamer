use std::sync::Arc;

use iced_video_player::Video;

use super::*;

impl NStreamer {
    pub fn loaded(&mut self, loaded: LoadedMessage) -> Task<Message> {
        match loaded {
            LoadedMessage::Settings(s) => self.loaded_settings(s),
            LoadedMessage::LiveStream(l) => self.loaded_live_stream(l),
        }
    }
    fn loaded_live_stream(&mut self, live_stream: Result<Arc<Video>, Error>) -> Task<Message> {
        let res = self.live_stream.new_live_stream(live_stream);
        self.apply_result_and_return(res, Task::none())
    }
    // should only be called once
    fn loaded_settings(&mut self, settings: Result<Settings, Error>) -> Task<Message> {
        self.apply_result_and(settings, |this, s| this.settings = s);
        self.init_second_stage()
    }
}
