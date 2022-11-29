use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::biz::model::{Delete, Model, Store};
use crate::constant::db_conn_pool;
use crate::errors::Error;
#[derive(Debug, Clone, FromRow, serde::Serialize, serde::Deserialize)]
pub struct MediaRepository {
    // ...
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub addition: Option<String>,
    pub create_time: DateTime<Utc>,
    pub deleted: bool,
}

impl MediaRepository {
    pub fn new(name: String, addition: String, description: String) -> Self {
        Self {
            id: None,
            name,
            description,
            addition: Some(addition),
            create_time: Utc::now(),
            deleted: false,
        }
    }

    // find_all list all repositories.
    pub async fn find_all() -> Result<Vec<MediaRepository>, Error> {
        let conn = db_conn_pool().await?;

        let sql = format!(
            r#"
            SELECT * FROM {} WHERE deleted = 0;
            "#,
            Self::table_name()
        );

        let repos = sqlx::query_as::<_, MediaRepository>(&sql)
            .fetch_all(conn)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        info!("find_all: {:?}", repos);
        Ok(repos)
    }
}
impl Model for MediaRepository {
    fn table_name() -> String {
        return "m_repo".to_string();
    }
}
#[async_trait]
impl Store<MediaRepository, String> for MediaRepository {
    async fn save(&mut self) -> Result<String, Error> {
        //check params
        let id = Uuid::new_v4().to_string();
        if self.name.is_empty() {
            return Err(Error::InvalidParams(
                "repo-> name cannot be empty".to_string(),
            ));
        }
        self.id = Some(id.to_owned());
        self.create_time = Utc::now();

        let pool = db_conn_pool().await?;

        let sql = format!(
            r#"
        INSERT INTO {} (id, name, description, create_time, deleted)
        VALUES (?1, ?2, ?3, ?4, ?5);
        "#,
            Self::table_name()
        );
        sqlx::query(&sql)
            .bind(self.id.to_owned())
            .bind(self.name.to_owned())
            .bind(self.description.to_owned())
            .bind(self.create_time.timestamp())
            .bind(self.deleted)
            .execute(pool)
            .await
            .or_else(|err| Err(Error::Database(err)))?;
        Ok(id)
        // ...
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
        UPDATE {} SET (name, description, create_time, deleted)
        VALUES (?1, ?2, ?3, ?4, ?5);
        "#,
            Self::table_name()
        );
        sqlx::query(&sql)
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
    async fn find(id: String) -> Result<Self, Error> {
        // ...
        let pool = db_conn_pool().await?;
        sqlx::query_as::<_, MediaRepository>(
            format!(
                r#"
        SELECT * FROM {} WHERE id = ? and deleted = 0"#,
                Self::table_name()
            )
            .as_str(),
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .or_else(|err| Err(Error::Database(err)))
    }
    async fn remove(&mut self) -> Result<(), Error> {
        // ...
        let pool = db_conn_pool().await?;
        sqlx::query(
            format!(
                r#"
        UPDATE {} SET deleted = 1 WHERE id = ?"#,
                Self::table_name()
            )
            .as_str(),
        )
        .bind(self.id.to_owned())
        .execute(pool)
        .await
        .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }
}
#[async_trait]
impl Delete<String> for MediaRepository {
    async fn delete(id: String) -> Result<(), Error> {
        let pool = db_conn_pool().await?;
        sqlx::query(
            format!(
                r#"
        DELETE FROM {} WHERE id = ?"#,
                Self::table_name()
            )
            .as_str(),
        )
        .bind(id.to_owned())
        .execute(pool)
        .await
        .or_else(|err| Err(Error::Database(err)))?;
        Ok(())
    }
}
