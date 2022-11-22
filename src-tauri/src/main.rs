#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod storage;
// pub mod menu;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! pix 0!", name)
}
#[tauri::command]
fn upload_token(key: &str) -> String {
    format!("{}", storage::qiniu::upload_token(key).unwrap())
}
#[tauri::command]
fn upload_file(
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

use tauri_plugin_store::PluginBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![greet, upload_token, upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}