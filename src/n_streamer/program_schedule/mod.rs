pub mod analyzed_schedule;
pub mod parsed_schedule;
pub mod title;

use chrono::Local;
use iced::{
    Element,
    Length::Fill,
    Task, Theme,
    widget::{self, Column, mouse_area, row, scrollable, space, text},
};
use turso::Connection;

use crate::n_streamer::{
    db,
    error::Error,
    message::Message,
    program_schedule::{
        analyzed_schedule::{AnalyzedEpisode, AnalyzedSchedule},
        parsed_schedule::ScheduleRequest,
    },
    ui_utils::{PADDING, SPACING},
};

#[derive(Debug, Default)]
pub struct ProgramSchedule {
    hovered_episode: usize,
    episodes: Vec<AnalyzedEpisode>,
    connection: Option<Connection>,
}

impl ProgramSchedule {
    pub fn view(&self) -> Element<'_, Message> {
        let episodes = self
            .episodes
            .iter()
            .enumerate()
            .fold(Column::new(), |c, (id, e)| {
                c.push(
                    mouse_area(
                        widget::container(
                            row![
                                text(e.program_title.as_str()).style(move |theme: &Theme| {
                                    if self.hovered_episode == id {
                                        let mut style = widget::text::default(theme);
                                        style.color =
                                            Some(theme.extended_palette().background.strong.text);
                                        style
                                    } else {
                                        widget::text::default(theme)
                                    }
                                }),
                                space().width(Fill),
                            ]
                            .padding(PADDING)
                            .spacing(SPACING),
                        )
                        .style(move |theme: &Theme| {
                            if self.hovered_episode == id {
                                widget::container::transparent(theme)
                                    .background(theme.extended_palette().background.strong.color)
                            } else {
                                widget::container::transparent(theme)
                            }
                        }),
                    )
                    .on_enter(Message::ScheduleElementEntered(id)),
                )
            });
        scrollable(episodes.padding(PADDING).width(Fill)).into()
    }
    pub fn set_connectoin(&mut self, connection: Connection) {
        self.connection = Some(connection);
    }
    pub fn set_schedule(&mut self, episodes: Vec<AnalyzedEpisode>) {
        self.episodes = episodes;
    }

    pub fn set_hovered_episode(&mut self, id: usize) {
        self.hovered_episode = id;
    }

    pub fn update(&mut self) -> Result<Option<Task<Message>>, Error> {
        if !self.episodes.is_empty() {
            return Ok(None);
        }

        if let Some(connection) = self.connection.clone() {
            let time = Local::now().to_string();

            let get_episodes_task = Task::perform(
                db::get_episodes(connection.clone(), time),
                Message::LoadedEpisodes,
            );

            Ok(Some(get_episodes_task))
        } else {
            Ok(None)
        }
    }

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
}
