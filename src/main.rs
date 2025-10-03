use crate::n_streamer::NStreamer;

mod n_streamer;

fn main() -> iced::Result {
    iced::application(NStreamer::init, NStreamer::update, NStreamer::view)
        .subscription(NStreamer::subscription)
        .theme(NStreamer::theme)
        .exit_on_close_request(false)
        .run()
}
