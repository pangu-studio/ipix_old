use std::fs::File;
use std::ops::Deref;
use std::sync::Mutex;

use once_cell::sync::OnceCell;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};

use crate::errors::Error;
use crate::errors::Error::DBMigrate;
use crate::errors::Error::Database;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

pub async fn run_migrations() -> Result<(), Error> {
    sqlx::migrate!("./db/migrations")
        .run(db_conn_pool().await?)
        .await
        .or_else(|err| Err(DBMigrate(err)))
}

pub fn app_data_path(path: String) -> &'static Mutex<String> {
    static INSTANCE: OnceCell<Mutex<String>> = OnceCell::new();
    INSTANCE.get_or_init(|| Mutex::new(path))
}

//初始化全局db
pub async fn db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let app_data_path = app_data_path("".to_string()).lock().unwrap().to_string();
    let db_path = app_data_path + "/data.db";
    debug!("db_path: {}", db_path);
    let opts = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await
}

static DB_ONCE_CELL: tokio::sync::OnceCell<Pool<Sqlite>> = tokio::sync::OnceCell::const_new();

pub async fn db_conn_pool() -> Result<&'static Pool<Sqlite>, Error> {
    DB_ONCE_CELL
        .get_or_try_init(db)
        .await
        .or_else(|err| Err(Database(err)))
}

pub fn init_logger(level: i8) {
    let log_file = app_data_path("".to_string())
        .lock()
        .unwrap()
        .deref()
        .to_owned()
        + "/ipix.log";
    println!("log: {}", log_file);
    let ter: Box<TermLogger>;
    if level == 0 {
        ter = TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        );
    } else if level == 1 {
        ter = TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
    } else {
        ter = TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
    }

    CombinedLogger::init(vec![
        ter,
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file).unwrap(),
        ),
    ])
    .unwrap_or(());
}
