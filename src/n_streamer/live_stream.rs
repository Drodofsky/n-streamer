use std::sync::Arc;

use iced::{Element, Task, widget};
use iced_video_player::{Video, VideoPlayer};
use reqwest::Url;

use crate::n_streamer::{error::Error, message::Message};

#[derive(Default)]
pub struct LiveStream {
    is_loading: bool,
    video: Option<Arc<Video>>,
}

impl LiveStream {
    fn view_live(&self) -> Option<Element<'_, Message>> {
        self.video
            .as_ref()
            .map(|video| VideoPlayer::new(video).into())
    }
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(live_view) = self.view_live() {
            live_view
        } else {
            widget::text("Loading ...").into()
        }
    }

    pub fn new_live_stream(&mut self, video: Result<Arc<Video>, Error>) {
        self.is_loading = false;
        match video {
            Ok(video) => {
                self.video = Some(video);
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }
    pub fn live_stream_button_pressed(&mut self) -> Task<Message> {
        if self.video.is_none() {
            if !self.is_loading {
                self.is_loading = true;

                Self::get_live_stream_task()
            } else {
                Task::none()
            }
        } else {
            Task::none()
        }
    }

    async fn init_live_stream() -> Result<Arc<Video>, Error> {
        let uri = include_str!("../../stream_url.txt");
        Ok(Arc::new(Video::new(&Url::parse(uri)?)?))
    }

    fn get_live_stream_task() -> Task<Message> {
        Task::perform(Self::init_live_stream(), Message::NewLiveStream)
    }
}
