use crate::{model::CreateUser, service::Service};
use tide::{Body, Request, Response, Server};

#[derive(Clone)]
pub struct State<S: Service> {
    service: S,
}

async fn get_users<S: Service>(request: Request<State<S>>) -> tide::Result {
    let service = &request.state().service;
    let users = service.list_users().await?;
    Ok(Response::from(Body::from_json(&users)?))
}

async fn post_user<S: Service>(mut request: Request<State<S>>) -> tide::Result {
    let user: CreateUser = request.body_json().await?;
    let user = request.state().service.create_user(user).await?;
    Ok(Response::builder(201).body(Body::from_json(&user)?).build())
}

pub fn create_app<S: Service>(service: S) -> Server<State<S>> {
    let state = State { service: service };
    let mut app = tide::with_state(state);
    app.at("/users").get(get_users);
    app.at("/users").post(post_user);
    app
}
