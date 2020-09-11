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
pub struct ServiceImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ServiceImpl {
    async fn connection(&self) -> Result<PgConnection, Error> {
        Ok(self.pool.get().await?.into_inner())
    }
}

pub fn create_service(database_url: String) -> ServiceImpl {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager);
    ServiceImpl { pool: pool }
}

#[async_trait]
impl Service for ServiceImpl {
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
