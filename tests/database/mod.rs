use directories::ProjectDirs;
use turso::{Builder, Database};

mod episode;
mod init;
mod start;

pub async fn create_db(project_dir: ProjectDirs) -> Database {
    let path = project_dir.data_local_dir();
    tokio::fs::create_dir_all(path).await.unwrap();
    let path = path.join("db.sqlite");
    let path = path.to_str().unwrap();
    Builder::new_local(path).build().await.unwrap()
}
