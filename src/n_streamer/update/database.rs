use super::*;

impl NStreamer {
    pub fn update_database(&mut self, message: DBMessage) -> Task<Message> {
        match message {
            DBMessage::Started(db) => {
                self.apply_result_and(db, |this, db| this.db = Some(db));
                Task::none()
            }
        }
    }
}
