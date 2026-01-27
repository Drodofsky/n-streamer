use crate::execute_tasks;
use directories::ProjectDirs;
use n_streamer::*;
use tokio::fs::try_exists;

#[tokio::test]
async fn start() {
    let dir = "n_streamer_tests/database/start";
    let project_dir = ProjectDirs::from_path(dir.into()).unwrap();
    let (mut n_streamer, task) = NStreamer::init(Some(project_dir.clone()));
    execute_tasks(task, &mut n_streamer).await;
    let ex = try_exists(project_dir.data_local_dir().join("db.sqlite"))
        .await
        .unwrap();
    assert_eq!(ex, true);
}
