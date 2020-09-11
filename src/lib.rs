#[macro_use]
extern crate diesel;
extern crate async_trait;

pub mod models;
pub mod schema;
pub mod services;

use crate::{
    models::User,
    schema::users::dsl::users,
    services::{Error, UserService},
};
use async_trait::async_trait;
use diesel::prelude::*;
use mobc::Pool;
use mobc_diesel::ConnectionManager;

#[derive(Clone)]
pub struct UserServiceImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserServiceImpl {
    async fn connection(&self) -> Result<PgConnection, Error> {
        Ok(self.pool.get().await?.into_inner())
    }
}

pub fn create_user_service(database_url: String) -> UserServiceImpl {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = mobc::Pool::builder().build(manager);
    UserServiceImpl { pool: pool }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: User) -> Result<User, Error> {
        Ok(diesel::insert_into(schema::users::table)
            .values(&user)
            .get_result(&self.connection().await?)?)
    }

    async fn list(&self) -> Result<Vec<User>, Error> {
        Ok(users.load::<User>(&self.connection().await?)?)
    }
}
