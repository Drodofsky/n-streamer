use std::sync::Arc;

use iced::{Element, Task, widget};
use iced_video_player::{Video, VideoPlayer};
use reqwest::Url;

use super::*;
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

    pub fn new_live_stream(&mut self, video: Result<Arc<Video>, Error>) -> Result<(), Error> {
        self.is_loading = false;
        self.video = Some(video?);
        Ok(())
    }
    pub fn live_stream_button_pressed(&mut self, uri: &str) -> Task<Message> {
        if self.video.is_none() {
            if !self.is_loading {
                self.is_loading = true;

                Task::perform(Self::init_live_stream(uri.to_string()), |l| {
                    Message::Loaded(LoadedMessage::LiveStream(l))
                })
            } else {
                Task::none()
            }
        } else {
            Task::none()
        }
    }

    async fn init_live_stream(uri: String) -> Result<Arc<Video>, Error> {
        Ok(Arc::new(Video::new(&Url::parse(&uri)?)?))
    }
    /// Set the volume multiplier of the audio.
    /// `0.0` = 0% volume, `1.0` = 100% volume.
    ///
    /// This uses a linear scale, for example `0.5` is perceived as half as loud.
    pub fn set_volume(&mut self, volume: f32) {
        if let Some(video) = &mut self.video {
            if let Some(v) = Arc::get_mut(video) {
                v.set_volume(f64::from(volume) / 100.);
            } else {
                println!("could not make mutable")
            }
        }
    }
    pub fn set_muted(&mut self, muted: bool) {
        if let Some(video) = &mut self.video {
            if let Some(v) = Arc::get_mut(video) {
                v.set_muted(muted);
            } else {
                println!("could not make mutable")
            }
        }
    }
}
