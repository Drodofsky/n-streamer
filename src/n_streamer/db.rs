use chrono::TimeDelta;
use turso::{Builder, Connection, Row};

use crate::n_streamer::utils::{parse_time_to_local, time_to_string};

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
    connection
        .execute(include_str!("../db/create_table_program.sql"), [0u32; 0])
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
                    Some(time_to_string(episode.schedule)),
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

pub(crate) async fn get_schedule_view(
    connection: Result<Connection, turso::Error>,
    after: String,
) -> Result<Vec<ScheduleView>, Error> {
    let mut rows = connection?
        .query(include_str!("../db/get_schedule.sql"), [after])
        .await?;
    let mut episodes = Vec::new();
    while let Some(row) = rows.next().await? {
        let episode = row_to_schedule_view(row)?;
        episodes.push(episode);
    }

    Ok(episodes)
}

fn row_to_schedule_view(row: Row) -> Result<ScheduleView, Error> {
    let error = Error::Database("Failed to load episode view".to_string());

    let program_id = row.get_value(0)?;
    let program_id = program_id.as_integer().ok_or(error.clone())?;
    let program_title = row.get_value(1)?;
    let program_title = program_title.as_text().ok_or(error.clone())?;
    let episode_id = row.get_value(2)?;
    let episode_id = episode_id.as_integer().ok_or(error.clone())?;
    let episode_title = row.get_value(3)?;
    let episode_title = episode_title.as_text().map(|e| e.to_string());
    let schedule = row.get_value(4)?;
    let schedule = schedule.as_text().ok_or(error.clone())?;
    let period = row.get_value(5)?;
    let period = period.as_integer().ok_or(error.clone())?;
    let schedule = parse_time_to_local(schedule)?;
    let period = TimeDelta::seconds(*period);

    let episode = ScheduleView {
        program_id: *program_id,
        program_title: program_title.to_string(),
        episode_id: *episode_id,
        episode_title,
        schedule,
        period,
    };

    Ok(episode)
}
