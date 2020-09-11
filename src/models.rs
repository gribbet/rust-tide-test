use super::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Insertable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}
