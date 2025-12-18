use chrono::{DateTime, Local, TimeDelta};
use turso::{Connection, Row};

use crate::n_streamer::{
    error::Error,
    program_schedule::{
        analyzed_program_info::AnalyzedProgramInfo, analyzed_schedule::AnalyzedEpisode,
    },
    ui_utils::Str,
};

impl Str for EpisodeView {
    fn get_str(&self) -> String {
        if let Some(ep_title) = &self.episode_title {
            format!("{} {}", self.program_title, ep_title)
        } else {
            self.program_title.clone()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EpisodeView {
    pub program_id: i64,
    pub program_title: String,
    pub episode_id: i64,
    pub episode_title: Option<String>,
    pub schedule: DateTime<Local>,
    pub period: TimeDelta,
    pub genre: Option<String>,
    pub logo_link: Option<String>,
    pub synopsis: Option<String>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EpisodeId {
    pub program_id: i64,
    pub episode_id: i64,
    pub schedule: DateTime<Local>,
}

impl From<EpisodeView> for EpisodeId {
    fn from(value: EpisodeView) -> Self {
        EpisodeId {
            program_id: value.program_id,
            episode_id: value.episode_id,
            schedule: value.schedule,
        }
    }
}
impl From<&EpisodeView> for EpisodeId {
    fn from(value: &EpisodeView) -> Self {
        EpisodeId {
            program_id: value.program_id,
            episode_id: value.episode_id,
            schedule: value.schedule,
        }
    }
}

pub(crate) async fn init_db(connection: Result<Connection, turso::Error>) -> Result<(), Error> {
    let connection = connection?;
    connection.query("PRAGMA journal_mode=WAL;", ()).await?;
    connection
        .execute(include_str!("../db/create_table_episode.sql"), ())
        .await?;
    connection
        .execute(include_str!("../db/create_table_program.sql"), ())
        .await?;
    connection
        .execute(include_str!("../db/create_table_downloading.sql"), ())
        .await?;
    connection
        .execute(include_str!("../db/create_table_downloaded.sql"), ())
        .await?;
    connection
        .execute(include_str!("../db/create_table_download_queue.sql"), ())
        .await?;
    connection
        .execute(include_str!("../db/create_table_subscriptions.sql"), ())
        .await?;

    Ok(())
}

fn row_to_episode_view(row: Row) -> Result<EpisodeView, Error> {
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
    let schedule = DateTime::parse_from_str(schedule, "%Y-%m-%d %H:%M:%S %:z")?;
    let period = TimeDelta::seconds(*period);
    let genre = row.get_value(6)?;
    let genre = genre.as_text().map(|e| e.to_string());
    let logo_link = row.get_value(7)?;
    let logo_link = logo_link.as_text().map(|e| e.to_string());
    let synopsis = row.get_value(8)?;
    let synopsis = synopsis.as_text().map(|e| e.to_string());

    let episode = EpisodeView {
        program_id: *program_id,
        program_title: program_title.to_string(),
        episode_id: *episode_id,
        episode_title,
        schedule: schedule.with_timezone(&Local),
        period,
        genre,
        logo_link,
        synopsis,
    };

    Ok(episode)
}

fn row_to_episode(row: Row) -> Result<AnalyzedEpisode, Error> {
    let error = Error::Database("Failed to load episode".to_string());

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
    let rebroadcast_flg = row.get_value(7)?;
    let rebroadcast_flg = rebroadcast_flg.as_integer().map(|f| *f != 0);
    let bilingual_flg = row.get_value(8)?;
    let bilingual_flg = bilingual_flg.as_integer().map(|f| *f != 0);
    let english_flg = row.get_value(9)?;
    let english_flg = english_flg.as_integer().map(|f| *f != 0);

    let episode = AnalyzedEpisode {
        program_id: *program_id,
        program_title: program_title.to_string(),
        episode_id: *episode_id,
        episode_title,
        schedule: schedule.with_timezone(&Local),
        period,
        suspend_flg: *suspend_flg != 0,
        rebroadcast_flg,
        bilingual_flg,
        english_flg,
    };

    Ok(episode)
}

pub(crate) async fn get_current_episodes(
    connection: Connection,
    after: String,
) -> Result<Option<AnalyzedEpisode>, Error> {
    let mut rows = connection
        .query(include_str!("../db/get_current_episode.sql"), [after])
        .await?;
    if let Some(row) = rows.next().await? {
        let episode = row_to_episode(row)?;
        return Ok(Some(episode));
    }

    Ok(None)
}

pub(crate) async fn get_episode_views(
    connection: Connection,
    after: String,
) -> Result<Vec<EpisodeView>, Error> {
    let mut rows = connection
        .query(include_str!("../db/get_episode_view.sql"), [after])
        .await?;
    let mut episodes = Vec::new();
    while let Some(row) = rows.next().await? {
        let episode = row_to_episode_view(row)?;
        episodes.push(episode);
    }

    Ok(episodes)
}

pub(crate) async fn get_download_queue_views(
    connection: Connection,
) -> Result<Vec<EpisodeView>, Error> {
    let params: [&str; 0] = [];
    let mut rows = connection
        .query(include_str!("../db/get_download_queue.sql"), params)
        .await?;
    let mut episodes = Vec::new();
    while let Some(row) = rows.next().await? {
        let episode = row_to_episode_view(row)?;
        episodes.push(episode);
    }

    Ok(episodes)
}

pub(crate) async fn add_episode_to_download_queue(
    connection: Result<Connection, turso::Error>,
    episode: EpisodeId,
) -> Result<(), Error> {
    connection?
        .execute(
            include_str!("../db/add_episode_to_download_queue.sql"),
            [
                Some(episode.program_id.to_string()),
                Some(episode.episode_id.to_string()),
                Some(episode.schedule.to_string()),
            ],
        )
        .await?;

    Ok(())
}


pub(crate) async fn remove_episode_from_download_queue(
    connection: Result<Connection, turso::Error>,
    episode: EpisodeId,
) -> Result<(), Error> {
    println!("remove");
    connection?
        .execute(
            include_str!("../db/remove_episode_from_download_queue.sql"),
            [
                Some(episode.program_id.to_string()),
                Some(episode.episode_id.to_string()),
                Some(episode.schedule.to_string()),
            ],
        )
        .await?;

    Ok(())
}

pub(crate) async fn add_episodes(
    connection: Connection,
    episodes: Vec<AnalyzedEpisode>,
) -> Result<(), Error> {
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
pub(crate) async fn add_programs(
    connection: Connection,
    programs: Vec<Result<AnalyzedProgramInfo, Error>>,
) -> Result<(), Error> {
    for program in programs.into_iter() {
        let program = program?;
        connection
            .execute(
                include_str!("../db/add_program.sql"),
                [
                    Some(program.program_master_id.to_string()),
                    Some(program.program_name),
                    program.genre,
                    program.logo_link,
                    program.link,
                    program.synopsis,
                ],
            )
            .await?;
    }

    Ok(())
}
