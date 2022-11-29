pub mod account;
pub mod repo;

use crate::errors::Error;
use async_trait::async_trait;

pub trait Model {
    fn table_name() -> String;
}

//Model trait define
#[async_trait]
pub trait Store<M: Model, ID> {
    //save model to db
    async fn save(&mut self) -> Result<ID, Error>;
    //update model to db
    async fn update(&self) -> Result<(), Error>;
    //find model by id from db
    async fn find(id: ID) -> Result<M, Error>;
    //delete model
    async fn remove(&mut self) -> Result<(), Error>;
}

#[async_trait]
pub trait Delete<ID> {
    //delete model
    async fn delete(id: ID) -> Result<(), Error>;
}
