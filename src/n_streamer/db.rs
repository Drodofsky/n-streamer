use turso::Builder;

use super::*;

pub async fn start_db(settings: Settings) -> Result<Database, Error> {
    let error = Error::Config("Failed to get media path".to_string());
    if let Some(path) = settings.media_path() {
        tokio::fs::create_dir_all(path).await?;

        let path = path.join("db.sqlite");
        let path = path.to_str().ok_or(error)?;
        Ok(Builder::new_local(path).build().await?)
    } else {
        Err(error)
    }
}
