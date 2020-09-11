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
