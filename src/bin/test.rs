extern crate rust_tide_test;

use rust_tide_test::{create_user_service, models::User, services};
use std::env;
use tide::{Body, Request, Response};

#[derive(Clone)]
pub struct State<UserService: services::UserService> {
    user_service: UserService,
}

async fn get_users<UserService: services::UserService>(
    request: Request<State<UserService>>,
) -> tide::Result<impl Into<Response>> {
    let users = request.state().user_service.list().await?;
    Ok(Body::from_json(&users)?)
}

async fn post_user<UserService: services::UserService>(
    mut request: Request<State<UserService>>,
) -> tide::Result<impl Into<Response>> {
    let user: User = request.body_json().await?;
    let user = request.state().user_service.create(user).await?;
    let mut response = Response::new(201);
    response.set_body(Body::from_json(&user)?);
    Ok(response)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tide::log::start();
    let state = State {
        user_service: create_user_service(database_url),
    };
    let mut app = tide::with_state(state);
    /* app.at("/users")
    .post(|mut request: Request<State<_>>| async move {
        let user: User = request.body_json().await?;
        println!("user name: {}", user.name);
        let user =
            request.state().user_service.clone().create(user).await?;
        Ok(Body::from_json(&user)?)
    });*/

    app.at("/users").get(|request| get_users(request));
    app.at("/users").post(|request| post_user(request));
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
