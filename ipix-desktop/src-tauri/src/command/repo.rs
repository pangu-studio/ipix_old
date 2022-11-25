use chrono::Utc;
use uuid::Uuid;

use ipix_rs::biz::model::{Model};
use ipix_rs::biz::model::repo::{MediaRepository};

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn create_media_repo(name: String, description: String) -> Result<(), String> {
    let mut repo = MediaRepository::new(Uuid::new_v4().to_string(), name, description);
    repo.create_time = Utc::now();
    info!("create_media_repo: {:?}", repo);
    let res = repo.save().await;
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn find_media_repo(id: String) -> Result<MediaRepository, String> {
    match MediaRepository::find(id).await {
        Ok(repo) => Ok(repo),
        Err(err) => Err(err.to_string()),
    }
}
#[tauri::command]
pub async fn list_all_media_repo() -> Result<Vec<MediaRepository>, String> {
    info!("list all media repo");
    match MediaRepository::find_all().await {
        Ok(repos) => Ok(repos),
        Err(err) => Err(err.to_string()),
    }
}
