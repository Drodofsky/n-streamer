use crate::execute_tasks;
use iced::widget::Id;
use iced_test::simulator;
use n_streamer::*;

#[tokio::test]
async fn no_url() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/live_stream/no_url");

    let mut ui = simulator(n_streamer.view());

    let _ = ui.click("Watch Live").unwrap();
    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }
    let mut ui = simulator(n_streamer.view());

    ui.find(Id::new("error_no_streaming_url_configured"))
        .unwrap();
}

#[tokio::test]
async fn invalid_url() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/live_stream/invalid_url");

    let task = n_streamer.update(Message::Settings(SettingsMessage::NewStreamUrl(
        "not_a_valid_url".into(),
    )));
    execute_tasks(task, &mut n_streamer).await;

    let mut ui = simulator(n_streamer.view());

    let _ = ui.click("Watch Live").unwrap();
    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }
    let mut ui = simulator(n_streamer.view());

    ui.find(Id::new("error_URL")).unwrap();
}

#[tokio::test]
async fn not_a_streaming_url() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/live_stream/not_a_streaming_url");

    let task = n_streamer.update(Message::Settings(SettingsMessage::NewStreamUrl(
        "https://google.com".into(),
    )));
    execute_tasks(task, &mut n_streamer).await;

    let mut ui = simulator(n_streamer.view());

    let _ = ui.click("Watch Live").unwrap();
    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }
    let mut ui = simulator(n_streamer.view());

    ui.find(Id::new("error_video_player")).unwrap();
}

#[tokio::test]
async fn play_live() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/live_stream/play_live");

    let task = n_streamer.update(Message::Settings(SettingsMessage::NewStreamUrl(
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4"
            .into(),
    )));
    execute_tasks(task, &mut n_streamer).await;

    let mut ui = simulator(n_streamer.view());

    let _ = ui.click("Watch Live").unwrap();
    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }
    let mut ui = simulator(n_streamer.view());

    ui.find(Id::new("playing_live")).unwrap();
}
