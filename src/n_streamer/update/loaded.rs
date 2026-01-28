use std::sync::Arc;

use iced_video_player::Video;

use crate::n_streamer::db::add_episodes;

use super::*;

impl NStreamer {
    pub fn loaded(&mut self, loaded: LoadedMessage) -> Task<Message> {
        match loaded {
            LoadedMessage::Settings(s) => self.loaded_settings(s),
            LoadedMessage::LiveStream(l) => self.loaded_live_stream(l),
            LoadedMessage::Schedule(s) => self.loaded_schedule(s),
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
    fn loaded_schedule(&mut self, schedule: Result<AnalyzedSchedule, Error>) -> Task<Message> {
        self.apply_result_and_return_task(schedule, |this, s| {
            if let Some(db) = &this.db {
                let connection = db.connect();
                Task::perform(add_episodes(connection, s.episodes), |e| {
                    Message::DB(DBMessage::EpisodesAdded(e))
                })
            } else {
                Task::none()
            }
        })
    }
}
