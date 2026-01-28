use std::path::PathBuf;

use chrono::{DateTime, Local, NaiveDateTime, NaiveTime, TimeDelta, Timelike, Utc};
use chrono_tz::Asia::Tokyo;
use directories::ProjectDirs;

use super::*;

impl NStreamer {
    pub fn get_project_dir(&self) -> Result<ProjectDirs, Error> {
        if let Some(project_dir) = &self.project_dir {
            return Ok(project_dir.clone());
        }
        ProjectDirs::from("dev", "Drodofsky", "n-streamer")
            .ok_or(Error::FileSystem("Failed to get ProjectDirs".to_string()))
    }

    pub fn get_default_media_dir(&self) -> Result<PathBuf, Error> {
        if let Some(project_dir) = &self.project_dir {
            return Ok(project_dir.data_local_dir().to_owned());
        }
        Ok(ProjectDirs::from("dev", "Drodofsky", "n-streamer")
            .ok_or(Error::FileSystem("Failed to get ProjectDirs".to_string()))?
            .data_local_dir()
            .to_owned())
    }
}

pub fn japan_time_to_utc(japan_time: &str) -> Result<DateTime<Utc>, Error> {
    let schedule_src = NaiveDateTime::parse_from_str(japan_time, "%Y-%m-%d %H:%M:%S")?;
    let tokyo = schedule_src
        .and_local_timezone(Tokyo)
        .single()
        .ok_or(Error::Chrono("failed to convert time".to_string()))?;
    Ok(tokyo.with_timezone(&Utc))
}

pub fn parse_time_to_local(time: &str) -> Result<DateTime<Local>, Error> {
    let dt = DateTime::parse_from_rfc3339(time)?;
    Ok(dt.into())
}

pub fn parse_time_delta(delta: &str) -> Result<TimeDelta, Error> {
    let period_src = NaiveTime::parse_from_str(delta, "%H:%M:%S")?;
    let period = TimeDelta::new(period_src.num_seconds_from_midnight() as i64, 0)
        .ok_or(Error::Chrono("failed to create duration".to_string()))?;
    Ok(period)
}

pub fn time_to_string(time: impl Into<DateTime<Utc>>) -> String {
    time.into()
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

#[cfg(test)]
mod test {
    use std::ops::Add;

    use super::*;
    use chrono::TimeZone;
    #[test]
    fn jpn_to_utc() {
        let time = japan_time_to_utc("2026-01-27 01:40:00").unwrap();
        assert_eq!(time, Utc.with_ymd_and_hms(2026, 1, 26, 16, 40, 0).unwrap())
    }
    #[test]
    fn parse_period() {
        let period = parse_time_delta("01:39:00").unwrap();
        assert_eq!(period, TimeDelta::new(39 * 60 + 60 * 60, 0).unwrap())
    }
    #[test]
    fn time_to_str() {
        let t = time_to_string(Utc.with_ymd_and_hms(2026, 1, 26, 16, 40, 0).unwrap());
        assert_eq!(t, "2026-01-26T16:40:00Z")
    }
    #[test]
    fn parse_dt() {
        let dt = "2026-01-26T16:40:00Z";
        let dt: DateTime<Local> = parse_time_to_local(dt).unwrap();
        let offset_seconds = dt.offset().local_minus_utc();
        let comp = Utc.with_ymd_and_hms(2026, 1, 26, 16, 40, 0).unwrap();
        let comp = comp.add(TimeDelta::new(offset_seconds.into(), 0).unwrap());

        assert_eq!(dt.date_naive(), comp.date_naive());
        assert_eq!(dt.time(), comp.time());
    }
}
