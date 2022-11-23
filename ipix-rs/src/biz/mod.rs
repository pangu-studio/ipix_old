pub mod repo;

use crate::errors::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Model<T> {
     async fn save(&self) -> Result<(), Error>;
     async fn update(&self) -> Result<(), Error>;
     async fn find(id: &str) -> Result<T, Error>;
     async fn delete(&mut self) -> Result<(), Error>;
}
