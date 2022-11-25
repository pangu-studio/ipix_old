use ipix_rs::biz::model::{account::StorageAccount, Model};
#[tauri::command]
pub async fn create_storate_account(data: StorageAccount) -> Result<(), String> {
    info!("create_media_repo: {:?}", data);
    let account: &mut StorageAccount = &mut data.clone();
    account.save().await.map_err(|err| err.to_string())
}
#[tauri::command]
pub async fn list_all_storage_account() ->Result<Vec<StorageAccount>,String> {
    StorageAccount::find_all().await.map_err(|err| err.to_string())
}
