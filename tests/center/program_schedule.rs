use crate::*;
use directories::ProjectDirs;
use iced_test::simulator;
use n_streamer::*;

#[tokio::test]
async fn load() {
    let dir = "n_streamer_tests/program_schedule/load";
    let project_dir = ProjectDirs::from_path(dir.into()).unwrap();
    let (mut n_streamer, task) = NStreamer::init(Some(project_dir.clone()));
    execute_tasks(task, &mut n_streamer).await;
    let mut ui = simulator(n_streamer.view());
    ui.find(" ➕ ").unwrap();
}
