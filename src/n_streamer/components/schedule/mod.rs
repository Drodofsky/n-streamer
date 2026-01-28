mod analyzed_schedule;
mod parsed_schedule;
mod schedule_view;
pub use analyzed_schedule::*;
use iced::{Element, widget::text};
pub use parsed_schedule::*;
pub use schedule_view::*;

#[derive(Debug, Default, Clone)]
pub struct ProgramSchedule {
    schedule: Vec<ScheduleView>,
    hovered_id: usize,
}

impl ProgramSchedule {
    pub fn update_schedule(&mut self, new: Vec<ScheduleView>) {
        self.schedule = new;
    }
    pub fn view(&self) -> Element<'_, Message> {
        if self.schedule.is_empty() {
            text("Loading ...").into()
        } else {
            view_scroll_list(
                &self.schedule,
                self.hovered_id,
                ScrollListOrigin::Schedule,
                " ➕ ",
            )
            .into()
        }
    }
    pub fn set_hovered(&mut self, hovered_id: usize) {
        self.hovered_id = hovered_id;
    }
}

pub use super::*;
pub async fn get_analyzed_schedule() -> Result<AnalyzedSchedule, Error> {
    let json: ScheduleRequest =
        reqwest::get("https://nhkworldpremium.com/backend/api/v1/front/episodes?lang=en")
            .await?
            .json()
            .await?;
    if json.status != 400 {
        return Err(Error::Api(format!("API: {}", json.status)));
    }
    json.item.try_into()
}
