use crate::models::User;
use async_trait::async_trait;
use core::fmt::Display;
use diesel::ConnectionError;

#[derive(Debug)]
pub enum Error {
    ConnectionError(ConnectionError),
}

impl Display for Error {
    fn fmt(
        &self,
        _: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for Error {}

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn create(&self, user: User) -> Result<User, Error>;
    async fn list(&self) -> Result<Vec<User>, Error>;
}
