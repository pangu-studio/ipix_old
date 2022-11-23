#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


#[macro_use]
extern crate log;
extern crate simplelog;
extern crate serde;

pub mod command;
pub mod storage;

use ipix_rs::{add};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("abcd {}", add(1, 2));
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

use crate::command::{init,create_media_repo, find_media_repo, list_all_media_repo};
use tauri_plugin_store::PluginBuilder;
fn main() {

    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![
            init,
            greet,
            upload_token,
            upload_file,
            create_media_repo,
            list_all_media_repo,
            find_media_repo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
