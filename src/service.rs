use crate::model::User;
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
pub trait Service: 'static + Sync + Send + Clone {
    async fn create_user(&self, user: User) -> Result<User, Error>;
    async fn list_users(&self) -> Result<Vec<User>, Error>;
}
