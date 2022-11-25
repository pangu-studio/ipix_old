pub mod repo;
pub mod oss;
pub mod account;

use ipix_rs::constant;
use tauri::AppHandle;

#[tauri::command]
pub async fn init_lib(app_handle: AppHandle, env: String) -> Result<(), String> {
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