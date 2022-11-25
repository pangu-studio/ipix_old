use crate::storage;

#[tauri::command]
pub fn upload_file(
    key: &str,
    access_key: &str,
    secret_key: &str,
    bucket_name: &str,
    prefix: &str,
    window: tauri::Window,
) -> String {
    storage::qiniu::upload_file(key, access_key, secret_key, bucket_name, prefix, window).unwrap()
    // format!("{}", storage::qiniu::upload_token(key).unwrap())
}

#[tauri::command]
pub fn upload_token(key: &str) -> String {
    format!("{}", storage::qiniu::upload_token(key).unwrap())
}
