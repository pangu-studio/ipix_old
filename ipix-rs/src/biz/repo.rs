
use chrono::{DateTime,Utc};
use sqlx::FromRow;
use async_trait::async_trait;

use crate::biz::Model;
use crate::errors::Error;
use crate::constant::db_conn_pool;
#[derive(Debug, Clone,FromRow,serde::Serialize)]
pub struct MediaRepository {
    // ...
    pub id: String,
    pub name: String,
    pub description: String,
    pub create_time: DateTime<Utc>,
    pub deleted: bool
}

impl MediaRepository {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            create_time: Utc::now(),
            deleted: false
        }
    }

    pub async fn find_all() -> Result<Vec<MediaRepository>, Error> {
        let conn = db_conn_pool().await?;
        let repos = sqlx::query_as::<_, MediaRepository>("select * from m_repos where deleted = 0")
            .fetch_all(conn)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(repos)
    }
    // pub fn delete(&mut self) {
    //     self.deleted = true;
    // }
    // pub fn save(&self) -> Result<(), Error> {
    //     // ...
    // }
    // pub fn find(id: &str) -> Result<Self, Error> {
    //     // ...
    // }
}
#[async_trait]
impl Model<MediaRepository> for MediaRepository {
    async fn delete(&mut self)->Result<(), Error> {
        self.deleted = true;
        let pool = db_conn_pool().await?;
        sqlx::query(r#"
        DELETE FROM media_repos WHERE id = ?"#)
        .bind(self.id.to_owned()).execute(pool)
        .await
        .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }
     async fn save(&self) -> Result<(), Error> {
        if self.id.is_empty() || self.name.is_empty() {
            return Err(Error::InvalidParams("repo-> id,name cannot be empty".to_string()));
        }
        let pool = db_conn_pool().await?;
    
        let sql = r#"
        INSERT INTO m_repos (id, name, description, create_time, deleted)
        VALUES (?1, ?2, ?3, ?4, ?5);
        "#;
        sqlx::query(sql)
            .bind(self.id.to_owned())
            .bind(self.name.to_owned())
            .bind(self.description.to_owned())
            .bind(self.create_time.timestamp())
            .bind(self.deleted)
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
        // ...
    }
    async fn update(&self) -> Result<(), Error> {
        if self.id.is_empty() || self.name.is_empty() {
            return Err(Error::InvalidParams("repo-> id,name cannot be empty".to_string()));
        }
        let pool = db_conn_pool().await?;
    
        let sql = r#"
        UPDATE m_repos SET (name, description, create_time, deleted)
        VALUES (?1, ?2, ?3, ?4, ?5);
        "#;
        sqlx::query(sql)
            .bind(self.id.to_owned())
            .bind(self.name.to_owned())
            .bind(self.description.to_owned())
            .bind(self.create_time.timestamp())
            .bind(self.deleted)
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
        // ...
    }
    async fn find(id: &str) -> Result<Self, Error> {
        // ...
        let pool = db_conn_pool().await?;
       sqlx::query_as::<_, MediaRepository>(r#"
        SELECT * FROM m_repos WHERE id = ?"#).bind(id).fetch_one(pool).await.or_else(|err| Err(Error::Database(err)))
    }
}