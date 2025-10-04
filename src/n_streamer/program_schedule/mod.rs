pub mod analyzed_schedule;
pub mod parsed_schedule;

use chrono::Local;
use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{self, column, container, pick_list, row, text},
};

use crate::n_streamer::{
    error::Error,
    message::Message,
    program_schedule::{
        analyzed_schedule::{AnalyzedEpisode, AnalyzedSchedule},
        parsed_schedule::{Schedule, ScheduleRequest},
    },
    ui_utils::fmt_period,
};

#[derive(Debug, Default)]
pub struct ProgramSchedule {
    is_loading: bool,
    selected_program: Option<AnalyzedEpisode>,
    current_episode: Option<AnalyzedEpisode>,
    schedule: Option<AnalyzedSchedule>,
}

impl ProgramSchedule {
    pub fn view(&self) -> Element<'_, Message> {
        match &self.schedule {
            None => text("Loading ...").into(),
            Some(s) => container(widget::column![
                pick_list(
                    s.episodes.clone(),
                    self.selected_program.clone(),
                    Message::ScheduleProgramSelected,
                ),
                self.view_selected_program()
            ])
            .into(),
        }
    }
    pub fn schedule(&self) -> Vec<AnalyzedEpisode> {
        self.schedule.as_ref().cloned().unwrap_or_default().episodes
    }
    pub fn get_current_episode(&self) -> Option<&str> {
        self.current_episode
            .as_ref()
            .map(|e| e.program_title.as_str())
    }
    pub fn view_selected_program(&self) -> Element<'_, Message> {
        let mut col = column![];
        if let Some(sel_program) = &self.selected_program {
            col = col.push(row![
                text("Title"),
                container(text(&sel_program.program_title)).align_right(Fill)
            ]);
            if let Some(ep) = &sel_program.episode_title {
                col = col.push(row![text("Episode"), container(text(ep)).align_right(Fill)]);
            }
            col = col.push(row![
                text("Program Id"),
                container(text(sel_program.program_id)).align_right(Fill)
            ]);
            col = col.push(row![
                text("Episode Id"),
                container(text(sel_program.episode_id)).align_right(Fill)
            ]);
            col = col.push(row![
                text("Schedule"),
                container(text(
                    sel_program.schedule.format("%m/%d(%a) %H:%M").to_string()
                ))
                .align_right(Fill)
            ]);
            col = col.push(row![
                text("Period"),
                container(text(fmt_period(&sel_program.period))).align_right(Fill)
            ]);
        }

        col.into()
    }

    pub fn update_current_episode(&mut self) -> Result<(), Error> {
        if let Some(ce) = &self.current_episode {
            let now = Local::now();
            if now
                < ce.schedule
                    .checked_add_signed(ce.period)
                    .ok_or(Error::Chrono("failed to calculate time offset".to_string()))?
            {
                return Ok(());
            }
        }
        self.set_current_episode()
    }
    fn set_current_episode(&mut self) -> Result<(), Error> {
        let now = Local::now();
        if let Some(schedule) = &self.schedule {
            self.current_episode = schedule
                .episodes
                .iter()
                .find(|e| {
                    let end = if let Some(end) = e.schedule.checked_add_signed(e.period) {
                        end
                    } else {
                        return false;
                    };
                    e.schedule <= now && end >= now
                })
                .cloned();
        }
        Ok(())
    }

    pub fn update_schedule(&mut self) -> Task<Message> {
        if !self.is_loading {
            self.is_loading = true;

            Self::get_schedule_task()
        } else {
            Task::none()
        }
    }

    fn get_schedule_task() -> Task<Message> {
        Task::perform(Self::get_schedule(), Message::NewSchedule)
    }
    pub fn new_schedule(&mut self, schedule: Result<Schedule, Error>) -> Result<(), Error> {
        self.is_loading = false;
        self.set_new_schedule(schedule?)?;
        Ok(())
    }

    pub fn select_episode(&mut self, episode: AnalyzedEpisode) {
        self.selected_program = Some(episode);
    }

    async fn get_schedule() -> Result<Schedule, Error> {
        let json: ScheduleRequest =
            reqwest::get("https://nhkworldpremium.com/backend/api/v1/front/episodes?lang=en")
                .await?
                .json()
                .await?;
        if json.status != 400 {
            return Err(Error::Api(format!("API: {}", json.status)));
        }
        Ok(json.item)
    }
    fn set_new_schedule(&mut self, schedule: Schedule) -> Result<(), Error> {
        let schedule = AnalyzedSchedule::try_from(schedule)?;
        if self.selected_program.is_none() {
            self.selected_program = schedule.episodes.first().cloned();
        }
        self.schedule = Some(schedule);
        Ok(())
    }
}
