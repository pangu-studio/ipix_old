use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::biz::model::Model;
use crate::constant::db_conn_pool;
use crate::errors::Error;

#[derive(Debug, Clone, FromRow, serde::Serialize, serde::Deserialize)]
pub struct StorageAccount {
    // ...
    pub id: Option<i32>,
    pub name: String,
    pub app_key: String,
    pub secret: String,
    pub description: Option<String>,
    pub addition: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub deleted: Option<bool>,
}

impl StorageAccount {
    pub fn new(name: String, app_key: String, secret: String) -> Self {
        Self {
            id: None,
            name,
            app_key,
            secret,
            description: None,
            addition: None,
            create_time: None,
            deleted: None,
        }
    }
    pub async fn find_all() -> Result<Vec<StorageAccount>, Error> {
        let conn = db_conn_pool().await?;
        let sql = format!(
            r#"
        SELECT * FROM {} where deleted = 0;
            "#,
            Self::table_name()
        );
        let repos = sqlx::query_as::<_, StorageAccount>(&sql)
            .fetch_all(conn)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        info!("find_all: {:?}", repos);
        Ok(repos)
    }
}
#[async_trait]
impl Model<StorageAccount, i32> for StorageAccount {
    fn table_name() -> String {
        "m_storage_account".to_string()
    }
    async fn save(&mut self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::InvalidParams(
                "repo-> id,name cannot be empty".to_string(),
            ));
        }
        let pool = db_conn_pool().await?;
        self.create_time = Some(Utc::now());
        let sql = format!(
            r#"
        INSERT INTO {} (name, app_key, secret, description, addition, create_time)
        VALUES(?1, ?2, ?3, ?4, ?5, ?6);
        "#,
            Self::table_name()
        );
        sqlx::query(&sql)
            .bind(self.name.to_owned())
            .bind(self.app_key.to_owned())
            .bind(self.secret.to_owned())
            .bind(self.description.to_owned())
            .bind(self.addition.to_owned())
            .bind(self.create_time.to_owned())
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }

    async fn update(&self) -> Result<(), Error> {
        if self.id.is_none() || self.name.is_empty() {
            return Err(Error::InvalidParams(
                "repo-> id,name cannot be empty".to_string(),
            ));
        }
        let pool = db_conn_pool().await?;
        let sql = format!(
            r#"
        DELETE FROM {} WHERE id = ?";
        "#,
            Self::table_name()
        );
        sqlx::query(&sql)
            .bind(self.id.unwrap())
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }

    async fn find(id: i32) -> Result<StorageAccount, Error> {
        let pool = db_conn_pool().await?;
        let account = sqlx::query_as::<_, StorageAccount>(r#""#)
            .bind(id)
            .fetch_one(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;

        Ok(account)
    }
    async fn delete(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
