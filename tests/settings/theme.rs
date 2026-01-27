use crate::execute_tasks;
use directories::ProjectDirs;
use iced_test::simulator;
use n_streamer::*;

#[tokio::test]
async fn select_white_theme() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/theme/select_white_theme");

    let task = n_streamer.update(Message::Settings(SettingsMessage::SettingSelected(
        SettingItem::Theme,
    )));
    execute_tasks(task, &mut n_streamer).await;
    let mut ui = simulator(n_streamer.view());

    let _ = ui.click("Light").unwrap();
    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }

    let settings = Settings::load(n_streamer.get_project_dir()).await.unwrap();
    assert_eq!(settings.get_theme(), Theme::Light);
    assert_eq!(n_streamer.theme(), iced::Theme::Light);
}

#[tokio::test]
async fn load_light_theme() {
    let dir = "n_streamer_tests/theme/load_light_theme";
    let mut settings = Settings::default();
    let project_dir = ProjectDirs::from_path(dir.into()).unwrap();
    let task = settings.set_theme(Theme::Light, Ok(project_dir.clone()));
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir(dir);
    execute_tasks(task, &mut n_streamer).await;

    let (mut n_streamer, task) = NStreamer::init(Some(project_dir));
    execute_tasks(task, &mut n_streamer).await;
    assert_eq!(n_streamer.theme(), iced::Theme::Light);
}

#[tokio::test]
async fn select_dark_theme() {
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir("n_streamer_tests/theme/select_dark_theme");

    let task = n_streamer.update(Message::Settings(SettingsMessage::SettingSelected(
        SettingItem::Theme,
    )));
    execute_tasks(task, &mut n_streamer).await;
    let mut ui = simulator(n_streamer.view());

    let _ = ui.click("Dark").unwrap();
    for message in ui.into_messages() {
        let task = n_streamer.update(message);
        execute_tasks(task, &mut n_streamer).await;
    }

    let settings = Settings::load(n_streamer.get_project_dir()).await.unwrap();
    assert_eq!(settings.get_theme(), Theme::Dark);
    assert_eq!(n_streamer.theme(), iced::Theme::Dark);
}

#[tokio::test]
async fn load_dark_theme() {
    let dir = "n_streamer_tests/theme/load_dark_theme";
    let mut settings = Settings::default();
    let project_dir = ProjectDirs::from_path(dir.into()).unwrap();
    let task = settings.set_theme(Theme::Dark, Ok(project_dir.clone()));
    let mut n_streamer = NStreamer::default();
    n_streamer.set_project_dir(dir);
    execute_tasks(task, &mut n_streamer).await;

    let (mut n_streamer, task) = NStreamer::init(Some(project_dir));
    execute_tasks(task, &mut n_streamer).await;
    assert_eq!(n_streamer.theme(), iced::Theme::Dark);
}
