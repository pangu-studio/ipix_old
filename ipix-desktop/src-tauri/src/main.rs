#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;
extern crate serde;
extern crate simplelog;

pub mod command;
pub mod storage;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use crate::command::account::{
    create_storate_account, delete_storage_account, list_all_storage_account,
    remove_storage_account,update_storate_account
};
use crate::command::oss::{upload_file, upload_token};
use crate::command::repo::{create_media_repository, find_media_repository, list_all_media_repository, update_media_repository};
use ipix_rs::constant;
use std::fs;
use tauri::api::path;
use tauri::async_runtime;
use tauri_plugin_store::PluginBuilder;
fn main() {
    //init ipix-rs lib
    init_lib();
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![
            upload_token,
            upload_file,
            create_media_repository,
            update_media_repository,
            list_all_media_repository,
            find_media_repository,
            create_storate_account,
            list_all_storage_account,
            delete_storage_account,
            remove_storage_account,
            update_storate_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_lib() {
    let mut p = path::data_dir().unwrap();
    p = p.join("iPix");
    let dbpath = p.as_path().as_os_str().to_str().unwrap();
    fs::create_dir_all(p.clone()).unwrap();
    println!("data dir {:?}", dbpath);
    constant::app_data_path(dbpath.to_string());
    constant::init_logger(0);
    match async_runtime::block_on(constant::run_migrations()) {
        Ok(_) => {
            println!("success");
        }
        Err(err) => {
            println!("error: {:?}", err)
        }
    };
}
