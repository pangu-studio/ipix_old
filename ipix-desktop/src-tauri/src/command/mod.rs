use chrono::Utc;
use tauri::AppHandle;
use uuid::Uuid;

use ipix_rs::biz::{repo::MediaRepository, Model};
use ipix_rs::constant;

#[tauri::command]
pub async fn init(app_handle: AppHandle, env: String) -> Result<(), String> {
    //TODO check env if dev using dev.db and debug log level else if prod prod.db
    println!("init env: {}", env);

    let appdir = app_handle.path_resolver().app_dir().unwrap();

    constant::app_data_path(String::from(appdir.to_str().unwrap()));
    constant::init_logger(1);
    match constant::run_migrations().await {
        Ok(_) => {
            info!("migrations success");
            Ok(())
        }
        Err(err) => {
            error!("migrations failed: {}", err);
            Err(err.to_string())
        }
    }
}
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
    match MediaRepository::find(&id).await {
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
