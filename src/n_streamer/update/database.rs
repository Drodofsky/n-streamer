use crate::n_streamer::db::init_db;

use super::*;

impl NStreamer {
    pub fn update_database(&mut self, message: DBMessage) -> Task<Message> {
        match message {
            DBMessage::Started(db) => {
                self.apply_result_and(db, |this, db| this.db = Some(db));
                if let Some(db) = &self.db {
                    let res = db.connect();
                    return Task::perform(init_db(res), |e| Message::DB(DBMessage::Initialized(e)));
                }
                Task::none()
            }
            DBMessage::Initialized(e) => {
                self.apply_result(e);
                Task::perform(get_analyzed_schedule(), |a| {
                    Message::Loaded(LoadedMessage::Schedule(a))
                })
            }
            DBMessage::EpisodesAdded(e) => {
                self.apply_result(e);
                Task::none()
            }
        }
    }
}
