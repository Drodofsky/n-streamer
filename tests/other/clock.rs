use crate::execute_tasks;
use chrono::Local;
use iced_test::simulator;
use n_streamer::*;
#[tokio::test]
async fn clock() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/other/clock");

    let task = n_streamer.update(Message::Tick);
    execute_tasks(task, &mut n_streamer).await;
    let time = Local::now();
    let mut ui = simulator(n_streamer.view());
    ui.find(time.format("%H:%M").to_string()).unwrap();
}
