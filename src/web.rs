use crate::{model::CreateUser, service::Service};
use tide::{Body, Request, Response, Server};

#[derive(Clone)]
pub struct State<S: Service> {
    service: S,
}

async fn get_users<S: Service>(
    request: Request<State<S>>,
) -> Result<impl Into<Response>, tide::Error> {
    let service = &request.state().service;
    let users = service.list_users().await?;
    Ok(Body::from_json(&users)?)
}

async fn post_user<S: Service>(
    mut request: Request<State<S>>,
) -> Result<impl Into<Response>, tide::Error> {
    let user: CreateUser = request.body_json().await?;
    let user = request.state().service.create_user(user).await?;
    let mut response = Response::new(201);
    response.set_body(Body::from_json(&user)?);
    Ok(response)
}

pub fn create_app<S: Service>(service: S) -> Server<State<S>> {
    let state = State { service: service };
    let mut app = tide::with_state(state);
    app.at("/users").get(get_users);
    app.at("/users").post(post_user);
    app
}
