use ipix_rs::biz::model::{repo::MediaRepository, Delete, Store};

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn create_media_repo(data: MediaRepository) -> Result<String, String> {
    let repo: &mut MediaRepository = &mut data.clone();
    info!("create_media_repo: {:?}", repo);
    let res = repo.save().await;
    match res {
        Ok(id) => {
            debug!("create media success: {:?},id: {}", repo, id);
            Ok(id)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn find_media_repo(id: String) -> Result<MediaRepository, String> {
    MediaRepository::find(id)
        .await
        .map_err(|err| err.to_string())
}
#[tauri::command]
pub async fn list_all_media_repo() -> Result<Vec<MediaRepository>, String> {
    debug!("list all media repo");
    match MediaRepository::find_all().await {
        Ok(repos) => Ok(repos),
        Err(err) => Err(err.to_string()),
    }
}
#[tauri::command]
pub async fn remove_media_repo(id: String) -> Result<(), String> {
    debug!("remove media repo id: {}", id);
    //find one
    let mut repo = MediaRepository::find(id)
        .await
        .map_err(|err| err.to_string())?;
    MediaRepository::remove(&mut repo)
        .await
        .map_err(|err| err.to_string())
}
#[tauri::command]
pub async fn delete_media_repo(id: String) -> Result<(), String> {
    debug!("delete media repo id: {}", id);
    MediaRepository::delete(id)
        .await
        .map_err(|err| err.to_string())
}
