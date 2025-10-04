use turso::Connection;

use crate::n_streamer::error::Error;

pub(crate) async fn init_db(connection: Result<Connection, turso::Error>) -> Result<(), Error> {
    let connection = connection?;
    if connection
        .execute(include_str!("../db/create_table_episode.sql"), ())
        .await?
        != 0
    {
        return Err(Error::Database("Failed to init tables".to_string()));
    }

    Ok(())
}
