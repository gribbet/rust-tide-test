use crate::{
    error::Error,
    model::{CreateUser, User},
    service::Service,
};
use async_std::sync::{Arc, RwLock};
use async_trait::async_trait;
use core::cmp::max;

#[derive(Clone)]
pub struct MockService {
    users: Arc<RwLock<Vec<User>>>,
}

impl MockService {
    pub fn new() -> Self {
        MockService {
            users: Arc::new(RwLock::new(vec![])),
        }
    }
}

#[async_trait]
impl Service for MockService {
    async fn create_user(&self, user: CreateUser) -> Result<User, Error> {
        let users = self.users.clone();
        let mut users = users.write().await;
        let next_id =
            users.iter().map(|user| user.id).fold(0, |x, y| max(x, y)) + 1;
        let user = User {
            id: next_id,
            name: user.name,
        };
        users.push(user.clone());
        Ok(user)
    }

    async fn get_user(&self, user_id: i32) -> Result<Option<User>, Error> {
        Ok(self
            .users
            .clone()
            .read()
            .await
            .iter()
            .find(|user| user.id == user_id)
            .map(|user| user.clone()))
    }

    async fn list_users(&self) -> Result<Vec<User>, Error> {
        Ok(self.users.clone().read().await.clone())
    }
}
