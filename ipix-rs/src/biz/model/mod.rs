pub mod account;
pub mod repo;
use crate::errors::Error;
use async_trait::async_trait;

//Model trait define
#[async_trait]
pub trait Model<T, ID> {
    //save model to db
    async fn save(&mut self) -> Result<(), Error>;
    //update model to db
    async fn update(&self) -> Result<(), Error>;
    //find model by id from db
    async fn find(id: ID) -> Result<T, Error>;
    //delete model
    async fn delete(&mut self) -> Result<(), Error>;
    fn table_name() -> String;
}
