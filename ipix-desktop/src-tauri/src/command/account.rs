use ipix_rs::biz::model::{account::StorageAccount, Delete, Store};
#[tauri::command]
pub async fn create_storate_account(data: StorageAccount) -> Result<i64, String> {
    info!("create_media_repo: {:?}", data);
    let account: &mut StorageAccount = &mut data.clone();
    match account.save().await {
        Ok(id) => {
            info!("create account success: {:?},id: {}", account, id);
            Ok(id)
        }
        Err(err) => Err(err.to_string()),
    }
}
#[tauri::command]
pub async fn list_all_storage_account() -> Result<Vec<StorageAccount>, String> {
    StorageAccount::find_all()
        .await
        .map_err(|err| err.to_string())
}
#[tauri::command]
pub async fn delete_storage_account(id: i64) -> Result<(), String> {
    debug!("delete_storage_account: {}", id);
    StorageAccount::delete(id)
        .await
        .map_err(|err| err.to_string())
}
#[tauri::command]
pub async fn remove_storage_account(id: i64) -> Result<(), String> {
    debug!("remove_storage_account: {}", id);
    //find
    let mut account = StorageAccount::find(id)
        .await
        .map_err(|err| err.to_string())?;
    info!("remove_storage_account: {:?}", account);
    if account.deleted.unwrap_or(false) {
        return Err("already removed".to_string());
    }
    StorageAccount::remove(&mut account)
        .await
        .map_err(|err| err.to_string())
}
#[tauri::command]
pub async fn update_storate_account(data: StorageAccount) -> Result<(), String> {
    info!("update_storate_account: {:?}", data);
    let account: &mut StorageAccount = &mut data.clone();
    match account.update().await {
        Ok(_) => {
            info!("update account success: {:?}", account);
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}
