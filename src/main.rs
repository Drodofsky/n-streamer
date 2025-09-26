use crate::n_streamer::NStreamer;

mod n_streamer;

fn main() -> iced::Result {
    iced::application("NStreamer", NStreamer::update, NStreamer::view).run_with(NStreamer::init)
}
