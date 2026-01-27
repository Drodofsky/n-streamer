use crate::execute_tasks;
use directories::ProjectDirs;
use iced::widget::Id;
use iced_test::simulator;
use n_streamer::*;
#[tokio::test]
async fn insert_streaming_url() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/locations/insert_streaming_url");

    let task = n_streamer.update(Message::Settings(SettingsMessage::SettingSelected(
        SettingItem::Locations,
    )));
    execute_tasks(task, &mut n_streamer).await;
    let mut ui = simulator(n_streamer.view());

    let _ = ui.click(Id::new("streaming_url_input")).unwrap();
    let _ = ui.typewrite("https://google.com");

    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }
    let mut ui = simulator(n_streamer.view());

    let _ = ui.click("ok").unwrap();
    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }

    let settings = Settings::load(n_streamer.get_project_dir()).await.unwrap();
    assert_eq!(settings.stream_url(), Some("https://google.com"));
}

#[tokio::test]
async fn insert_media_path() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/locations/insert_media_path");

    let task = n_streamer.update(Message::Settings(SettingsMessage::SettingSelected(
        SettingItem::Locations,
    )));
    execute_tasks(task, &mut n_streamer).await;
    let mut ui = simulator(n_streamer.view());

    let _ = ui.click(Id::new("media_path_input")).unwrap();
    let _ = ui.typewrite("/usr/local/tmp");

    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }
    let mut ui = simulator(n_streamer.view());

    let _ = ui.click("ok").unwrap();
    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }

    let settings = Settings::load(n_streamer.get_project_dir()).await.unwrap();
    assert_eq!(
        settings.media_path().map(|s| s.to_str()).flatten(),
        Some("/usr/local/tmp")
    );
}

#[tokio::test]
async fn load_streaming_url() {
    let url = "https://google.com";
    let dir = "n_streamer_tests/theme/load_streaming_url";
    let project_dir = ProjectDirs::from_path(dir.into()).unwrap();
    let mut settings = Settings::default();
    settings.set_stream_url(url.into());
    settings.save(Ok(project_dir.clone())).await.unwrap();

    let (mut n_streamer, task) = NStreamer::init(Some(project_dir));
    execute_tasks(task, &mut n_streamer).await;
    let task = n_streamer.update(Message::Settings(SettingsMessage::SettingSelected(
        SettingItem::Locations,
    )));
    execute_tasks(task, &mut n_streamer).await;
    let mut ui = simulator(n_streamer.view());
    ui.find(url).unwrap();
}

#[tokio::test]
async fn load_media_path() {
    let media_path = "/tmp/videos";
    let dir = "n_streamer_tests/theme/load_media_path";
    let project_dir = ProjectDirs::from_path(dir.into()).unwrap();
    let mut settings = Settings::default();
    settings.set_media_path(media_path.into());
    settings.save(Ok(project_dir.clone())).await.unwrap();

    let (mut n_streamer, task) = NStreamer::init(Some(project_dir));
    execute_tasks(task, &mut n_streamer).await;
    let task = n_streamer.update(Message::Settings(SettingsMessage::SettingSelected(
        SettingItem::Locations,
    )));
    execute_tasks(task, &mut n_streamer).await;
    let mut ui = simulator(n_streamer.view());
    ui.find(media_path).unwrap();
}
