use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tide::{prelude::*, Body, Request}; // Pulls in the json! macro.

#[derive(Deserialize, Serialize, Clone)]
struct User {
    id: u64,
    name: String,
}

#[derive(Clone)]
pub struct State {
    users: Arc<RwLock<Vec<User>>>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::with_state(State {
        users: Arc::new(RwLock::new(Vec::new())),
    });
    app.at("/users")
        .post(|mut request: Request<State>| async move {
            let user: User = request.body_json().await?;
            println!("user name: {}", user.name);
            request
                .state()
                .users
                .clone()
                .write()
                .unwrap()
                .push(user.clone());
            Ok(Body::from_json(&user)?)
        });

    app.at("/users").get(|request: Request<State>| async move {
        let users = request.state().users.clone();
        let users = &*users.read().unwrap();
        Ok(json!(users))
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
