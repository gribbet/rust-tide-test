use crate::{
    error::Error,
    model::{CreateUser, User},
    schema,
    schema::users::dsl::users,
    service::Service,
};
use async_trait::async_trait;
use diesel::prelude::*;
use mobc::Pool;
use mobc_diesel::ConnectionManager;

#[derive(Clone)]
pub struct DatabaseService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DatabaseService {
    pub fn new(url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::builder().build(manager);
        DatabaseService { pool: pool }
    }

    async fn connection(&self) -> Result<PgConnection, Error> {
        Ok(self.pool.get().await?.into_inner())
    }
}

#[async_trait]
impl Service for DatabaseService {
    async fn create_user(&self, user: CreateUser) -> Result<User, Error> {
        Ok(diesel::insert_into(schema::users::table)
            .values(&user)
            .get_result(&self.connection().await?)?)
    }

    async fn get_user(&self, user_id: i32) -> Result<Option<User>, Error> {
        use schema::users::dsl::*;
        Ok(users
            .filter(id.eq(user_id))
            .first(&self.connection().await?)
            .optional()?)
    }

    async fn list_users(&self) -> Result<Vec<User>, Error> {
        Ok(users.load::<User>(&self.connection().await?)?)
    }
}
