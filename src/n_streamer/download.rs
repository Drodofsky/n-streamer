use iced::{
    Element,
    Length::{self, FillPortion},
    Task,
    widget::{self, row},
};
use turso::Connection;

use crate::{
    button_text,
    n_streamer::{
        NStreamer,
        db::{self, EpisodeView},
        error::Error,
        message::Message,
        ui_utils::{PADDING, SPACING, ScrollListOwner, fmt_period, view_scroll_list},
    },
    pop_up,
};

#[derive(Debug, Default, Clone)]
pub struct Downloads {
    download_queue: Vec<EpisodeView>,
    hovered_episode: usize,
    connection: Option<Connection>,
}

impl Downloads {
    pub fn view(&self) -> Element<'_, Message> {
        row![self.view_scroll_list(), self.view_hovered_info()].into()
    }
    fn view_scroll_list(&self) -> Element<'_, Message> {
        view_scroll_list(
            &self.download_queue,
            self.hovered_episode,
            ScrollListOwner::DownloadQueue,
            " ➖ "
        )
        .into()
    }
    fn view_hovered_info(&self) -> Element<'_, Message> {
        if let Some(episode) = self.download_queue.get(self.hovered_episode) {
            let mut col = iced::widget::column![];
            if let Some(ep_title) = &episode.episode_title {
                let title = format!("{} {}", episode.program_title, ep_title);
                col = col.push(row![
                    widget::text("Title:"),
                    widget::space().width(Length::Fill),
                    widget::text(title)
                ]);
            } else {
                col = col.push(row![
                    widget::text("Title:"),
                    widget::space().width(Length::Fill),
                    widget::text(episode.program_title.as_str())
                ]);
            }

            col = col.push(row![
                widget::text("Schedule:"),
                widget::space().width(Length::Fill),
                widget::text(episode.schedule.format("%m/%d (%a) %H:%M").to_string())
            ]);
            col = col.push(row![
                widget::text("Duration:"),
                widget::space().width(Length::Fill),
                widget::text(fmt_period(&episode.period))
            ]);
            if let Some(genre) = &episode.genre {
                col = col.push(row![
                    widget::text("Genre:"),
                    widget::space().width(Length::Fill),
                    widget::text(genre)
                ]);
            }

            if let Some(synopsis) = &episode.synopsis {
                col = col.push(row![
                    widget::text("Synopsis:"),
                    widget::space().width(Length::Fill),
                    widget::text(synopsis)
                ]);
            }

            col.padding(PADDING).spacing(SPACING).into()
        } else {
            iced::widget::column![].into()
        }
    }
    pub fn set_connectoin(&mut self, connection: Connection) {
        self.connection = Some(connection);
    }
    pub fn set_download_queue(&mut self, episodes: Vec<EpisodeView>) {
        dbg!(episodes.len());
        self.download_queue = episodes;
    }

    pub fn set_hovered_episode(&mut self, id: usize) {
        self.hovered_episode = id;
    }
    pub fn update(&mut self) -> Result<Task<Message>, Error> {
        if let Some(connection) = self.connection.clone() {
            let get_episodes_task = Task::perform(
                db::get_download_queue_views(connection.clone()),
                Message::LoadedDownloadQueue,
            );
            println!("updatede download queue");

            Ok(get_episodes_task)
        } else {
            Ok(Task::none())
        }
    }
}

impl NStreamer {
    pub(crate) fn view_download_popup(&self, episode_view: &EpisodeView) -> Element<'_, Message> {
        pop_up!(
            "Add episode to download queue",
            row![
                button_text!("yes")
                    .width(Length::FillPortion(1))
                    .on_press(Message::AddVideoToDownloadQueue(episode_view.into())),
                button_text!("Subscribe").width(Length::FillPortion(1)),
                button_text!("Close")
                    .width(FillPortion(1))
                    .on_press(Message::ClosePopUp)
            ]
            .spacing(SPACING)
            .padding(PADDING)
        )
        .into()
    }
    pub(crate) fn view_download_queue_popup(
        &self,
        episode_view: &EpisodeView,
    ) -> Element<'_, Message> {
        pop_up!(
            "Remove episode from download queue",
            row![
                button_text!("yes").width(Length::FillPortion(1)).on_press(Message::RemoveEpisodeFromDownloadQueue(episode_view.into())),
                button_text!("no")
                    .width(FillPortion(1))
                    .on_press(Message::ClosePopUp)
            ]
            .spacing(SPACING)
            .padding(PADDING)
        )
        .into()
    }
}
