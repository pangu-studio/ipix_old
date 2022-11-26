pub mod account;
pub mod repo;
use crate::errors::Error;
use async_trait::async_trait;

//Model trait define
#[async_trait]
pub trait Model<T, ID> {
    //save model to db
    async fn save(&mut self) -> Result<ID, Error>;
    //update model to db
    async fn update(&self) -> Result<(), Error>;
    //find model by id from db
    async fn find(id: ID) -> Result<T, Error>;
    //delete model
    async fn remove(&mut self) -> Result<(), Error>;
    //delete model
    async fn delete(id: ID) -> Result<(), Error>;
    fn table_name() -> String;
}
