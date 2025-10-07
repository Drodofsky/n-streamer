use chrono::{DateTime, Local, TimeDelta};
use turso::Connection;

use crate::n_streamer::{error::Error, program_schedule::analyzed_schedule::AnalyzedEpisode};

pub(crate) async fn init_db(connection: Result<Connection, turso::Error>) -> Result<(), Error> {
    let connection = connection?;
    connection.query("PRAGMA journal_mode=WAL;", ()).await?;
    connection
        .execute(include_str!("../db/create_table_episode.sql"), ())
        .await?;

    Ok(())
}

pub(crate) async fn get_current_episodes(
    connection: Connection,
    after: String,
) -> Result<Option<AnalyzedEpisode>, Error> {
    let mut rows = connection
        .query(include_str!("../db/get_current_episode.sql"), [after])
        .await?;
    let error = Error::Database("Failed to load episode".to_string());
    if let Some(row) = rows.next().await? {
        let program_id = row.get_value(0)?;
        let program_id = program_id.as_integer().ok_or(error.clone())?;
        let program_title = row.get_value(1)?;
        let program_title = program_title.as_text().ok_or(error.clone())?;
        let episode_id = row.get_value(2)?;
        let episode_id = episode_id.as_integer().ok_or(error.clone())?;

        let suspend_flg = row.get_value(4)?;
        let suspend_flg = suspend_flg.as_integer().ok_or(error.clone())?;
        let schedule = row.get_value(5)?;
        let schedule = schedule.as_text().ok_or(error.clone())?;
        let period = row.get_value(6)?;
        let period = period.as_integer().ok_or(error.clone())?;
        let schedule = DateTime::parse_from_str(schedule, "%Y-%m-%d %H:%M:%S %:z")?;
        let period = TimeDelta::seconds(*period);
        // TODO iml every element
        let episode = AnalyzedEpisode {
            program_id: *program_id,
            program_title: program_title.to_string(),
            episode_id: *episode_id,
            schedule: schedule.with_timezone(&Local),
            period: period,
            suspend_flg: *suspend_flg != 0,
            ..Default::default()
        };
        return Ok(Some(episode));
    }

    Ok(None)
}

pub(crate) async fn get_episodes(
    connection: Connection,
    after: String,
) -> Result<Vec<AnalyzedEpisode>, Error> {
    let mut rows = connection
        .query(include_str!("../db/get_episode.sql"), [after])
        .await?;
    let mut episodes = Vec::new();
    let error = Error::Database("Failed to load episode".to_string());
    while let Some(row) = rows.next().await? {
        let program_id = row.get_value(0)?;
        let program_id = program_id.as_integer().ok_or(error.clone())?;
        let program_title = row.get_value(1)?;
        let program_title = program_title.as_text().ok_or(error.clone())?;
        let episode_id = row.get_value(2)?;
        let episode_id = episode_id.as_integer().ok_or(error.clone())?;

        let episode_title = row.get_value(3)?;
        let episode_title = episode_title.as_text().map(|e| e.to_string());

        let suspend_flg = row.get_value(4)?;
        let suspend_flg = suspend_flg.as_integer().ok_or(error.clone())?;
        let schedule = row.get_value(5)?;
        let schedule = schedule.as_text().ok_or(error.clone())?;
        let period = row.get_value(6)?;
        let period = period.as_integer().ok_or(error.clone())?;
        let schedule = DateTime::parse_from_str(schedule, "%Y-%m-%d %H:%M:%S %:z")?;
        let period = TimeDelta::seconds(*period);
        // TODO iml every element
        let episode = AnalyzedEpisode {
            program_id: *program_id,
            program_title: program_title.to_string(),
            episode_id: *episode_id,
            episode_title: episode_title,
            schedule: schedule.with_timezone(&Local),
            period: period,
            suspend_flg: *suspend_flg != 0,
            ..Default::default()
        };
        episodes.push(episode);
    }

    Ok(episodes)
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
