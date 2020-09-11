use crate::models::User;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("connection error")]
    ConnectionError(mobc::Error<mobc_diesel::Error>),
    #[error("database error")]
    DatabaseError(diesel::result::Error),
}

impl From<mobc::Error<mobc_diesel::Error>> for Error {
    fn from(error: mobc::Error<mobc_diesel::Error>) -> Self {
        Error::ConnectionError(error)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Error::DatabaseError(error)
    }
}

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn create(&self, user: User) -> Result<User, Error>;
    async fn list(&self) -> Result<Vec<User>, Error>;
}
