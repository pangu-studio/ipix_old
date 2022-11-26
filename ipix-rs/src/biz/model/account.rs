use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::biz::model::Model;
use crate::constant::db_conn_pool;
use crate::errors::Error;

#[derive(Debug, Clone, FromRow, serde::Serialize, serde::Deserialize)]
pub struct StorageAccount {
    // ...
    pub id: Option<i64>,
    pub name: String,
    pub app_key: String,
    pub secret: String,
    pub provider: i32,
    pub description: Option<String>,
    pub addition: Option<String>,
    pub create_time: Option<DateTime<Utc>>,
    pub deleted: Option<bool>,
}

impl StorageAccount {
    pub fn new(name: String, app_key: String, secret: String, provider: i32) -> Self {
        Self {
            id: None,
            name,
            app_key,
            secret,
            provider,
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
        SELECT * FROM {} WHERE deleted = 0 ORDER BY id DESC;
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
impl Model<StorageAccount, i64> for StorageAccount {
    fn table_name() -> String {
        "m_storage_account".to_string()
    }
    async fn save(&mut self) -> Result<i64, Error> {
        if self.name.is_empty() {
            return Err(Error::InvalidParams(
                "repo-> id,name cannot be empty".to_string(),
            ));
        }
        let pool = db_conn_pool().await?;
        let dt = Utc::now();
        info!("dt: {}", dt);
        info!("dt: {}", dt.timestamp_millis());
        let id = dt.timestamp_millis();
        self.create_time = Some(dt);
        let sql = format!(
            r#"
        INSERT INTO {} (id, name, app_key, secret, description, addition, create_time)
        VALUES(?1,?2, ?3, ?4, ?5, ?6, ?7);
        "#,
            Self::table_name()
        );
        let id = sqlx::query(&sql)
            .bind(id)
            .bind(self.name.to_owned())
            .bind(self.app_key.to_owned())
            .bind(self.secret.to_owned())
            .bind(self.description.to_owned())
            .bind(self.addition.to_owned())
            .bind(self.create_time.to_owned())
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?
            .last_insert_rowid();

        self.id = Some(id);
        Ok(self.id.unwrap().to_owned())
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
        UPDATE  {} SET name = ?1, app_key = ?2, secret = ?3, provider = ?4, description = ?5, addition = ?6, deleted = ?7 WHERE id = ?8;
        "#,
            Self::table_name()
        );
        sqlx::query(&sql)
            .bind(self.name.to_owned())
            .bind(self.app_key.to_owned())
            .bind(self.secret.to_owned())
            .bind(self.provider.to_owned())
            .bind(self.description.to_owned())
            .bind(self.addition.to_owned())
            .bind(self.deleted.to_owned())
            .bind(self.id.unwrap())
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }

    async fn find(id: i64) -> Result<StorageAccount, Error> {
        let pool = db_conn_pool().await?;
        let sql = format!(
            r#"
            SELECT * FROM {} WHERE id = ? AND deleted = 0;
        "#,
            Self::table_name()
        );
        let account = sqlx::query_as::<_, StorageAccount>(&sql)
            .bind(id)
            .fetch_one(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;

        Ok(account)
    }

    async fn remove(&mut self) -> Result<(), Error> {
        if self.id.is_none() {
            return Err(Error::InvalidParams(
                "account-> id cannot be empty".to_string(),
            ));
        }
        self.deleted = Some(true);
        let pool = db_conn_pool().await?;
        let sql = format!(
            r#"
            UPDATE {} SET deleted = ?1 WHERE id = ?2;
            "#,
            Self::table_name()
        );
        info!("remove sql: {}", sql);
        let res = sqlx::query(&sql)
            .bind(self.deleted.unwrap())
            .bind(self.id.unwrap())
            .execute(pool)
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Database(err)),
        }
    }

    async fn delete(id: i64) -> Result<(), Error> {
        if id == 0 {
            return Err(Error::InvalidParams(
                "account-> id cannot be empty".to_string(),
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
            .bind(id)
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }
}
