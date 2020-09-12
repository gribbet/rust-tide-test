use crate::{
    error::Error,
    model::{CreateUser, User},
};
use async_trait::async_trait;

#[async_trait]
pub trait Service: Sync + Send {
    async fn create_user(&self, user: CreateUser) -> Result<User, Error>;
    async fn get_user(&self, id: i32) -> Result<Option<User>, Error>;
    async fn list_users(&self) -> Result<Vec<User>, Error>;
}
