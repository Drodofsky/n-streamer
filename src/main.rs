use crate::n_streamer::NStreamer;

mod n_streamer;

fn main() -> iced::Result {
    iced::application("NStreamer", NStreamer::update, NStreamer::view)
        .subscription(NStreamer::subscription)
        .exit_on_close_request(false)
        .run_with(NStreamer::init)
}
