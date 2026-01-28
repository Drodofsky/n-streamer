use turso::{Builder, Connection};

use super::*;

pub async fn start_db(settings: Settings) -> Result<Database, Error> {
    let error = Error::Config("Failed to get media path".to_string());
    if let Some(path) = settings.media_path() {
        tokio::fs::create_dir_all(path).await?;

        let path = path.join("db.sqlite");
        let path = path.to_str().ok_or(error)?;
        Ok(Builder::new_local(path).build().await?)
    } else {
        Err(error)
    }
}

pub async fn init_db(connection: Result<Connection, turso::Error>) -> Result<(), Error> {
    let connection = connection?;
    connection
        .query("PRAGMA journal_mode = 'experimental_mvcc';", [0u32; 0])
        .await?;
    connection
        .execute(include_str!("../db/create_table_episode.sql"), [0u32; 0])
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
