mod n_streamer;
use n_streamer::*;
fn main() -> iced::Result {
    iced::application(NStreamer::init, NStreamer::update, NStreamer::view)
        .subscription(NStreamer::subscription)
        .theme(NStreamer::theme)
        .exit_on_close_request(false)
        .run()
}

// TODO
// open questions: why does the live stream crash
// why does the live stream apply the audio settings at start
// TODO error handling for video mutation
