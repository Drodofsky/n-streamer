use crate::{database::create_db, execute_tasks};
use directories::ProjectDirs;
use n_streamer::*;
use turso::Connection;

async fn check_table_exists(connection: &Connection, table_name: &str) {
    let mut q = connection
        .query(
            include_str!("../test_data/sql/table_exists.sql"),
            [table_name],
        )
        .await
        .unwrap();
    let q = q.next().await.unwrap().unwrap().get_value(0).unwrap();
    let b = q.as_integer().unwrap();
    assert_eq!(*b, 1)
}

#[tokio::test]
async fn start() {
    let dir = "n_streamer_tests/database/init";
    let project_dir = ProjectDirs::from_path(dir.into()).unwrap();
    let (mut n_streamer, task) = NStreamer::init(Some(project_dir.clone()));
    execute_tasks(task, &mut n_streamer).await;
    let db = create_db(project_dir).await;
    let connection = db.connect().unwrap();
    check_table_exists(&connection, "episode").await;
}
