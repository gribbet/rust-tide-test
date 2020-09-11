use crate::{error::Error, model::User};
use async_trait::async_trait;

#[async_trait]
pub trait Service: 'static + Sync + Send + Clone {
    async fn create_user(&self, user: User) -> Result<User, Error>;
    async fn list_users(&self) -> Result<Vec<User>, Error>;
}
