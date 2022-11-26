use ipix_rs::biz::model::repo::MediaRepository;
use ipix_rs::biz::model::Model;

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn create_media_repo(data: MediaRepository) -> Result<String, String> {
    let repo: &mut MediaRepository = &mut data.clone();
    info!("create_media_repo: {:?}", repo);
    let res = repo.save().await;
    match res {
        Ok(id) => {
            info!("create media success: {:?},id: {}", repo, id);
            Ok(id)
        }
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
