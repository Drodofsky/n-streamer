use turso::Connection;

use crate::n_streamer::{error::Error, program_schedule::analyzed_schedule::AnalyzedEpisode};

pub(crate) async fn init_db(connection: Result<Connection, turso::Error>) -> Result<(), Error> {
    let connection = connection?;
    connection
        .execute(include_str!("../db/create_table_episode.sql"), ())
        .await?;

    Ok(())
}

pub(crate) async fn add_episodes(
    connection: Result<Connection, turso::Error>,
    episodes: Vec<AnalyzedEpisode>,
) -> Result<(), Error> {
    let connection = connection?;

    for episode in episodes.into_iter().filter(|e| e.program_title != "TBA") {
        connection
            .execute(
                include_str!("../db/add_episode.sql"),
                [
                    Some(episode.program_id.to_string()),
                    Some(episode.program_title),
                    Some(episode.episode_id.to_string()),
                    episode.episode_title,
                    Some((episode.suspend_flg as u8).to_string()),
                    Some(episode.schedule.to_string()),
                    Some(episode.period.num_seconds().to_string()),
                    episode.rebroadcast_flg.map(|f| (f as u8).to_string()),
                    episode.bilingual_flg.map(|f| (f as u8).to_string()),
                    episode.english_flg.map(|f| (f as u8).to_string()),
                ],
            )
            .await?;
    }

    Ok(())
}
