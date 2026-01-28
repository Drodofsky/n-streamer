use chrono::Local;

use crate::n_streamer::{
    db::{get_schedule_view, init_db},
    utils::time_to_string,
};

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
                if let Some(db) = &self.db {
                    let connection = db.connect();
                    Task::perform(
                        get_schedule_view(connection, time_to_string(Local::now())),
                        |v| Message::Loaded(LoadedMessage::ScheduleView(v)),
                    )
                } else {
                    Task::none()
                }
            }
        }
    }
}
