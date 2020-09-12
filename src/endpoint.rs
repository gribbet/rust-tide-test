use crate::{api::State, model::CreateUser, service::Service};
use tide::{Body, Request, Response, StatusCode};

pub async fn post_user<S: Service>(
    mut request: Request<State<S>>,
) -> tide::Result {
    let user: CreateUser = request.body_json().await?;
    let user = request.state().service.create_user(user).await?;
    Ok(Response::builder(StatusCode::Created)
        .body(Body::from_json(&user)?)
        .build())
}

pub async fn get_users<S: Service>(request: Request<State<S>>) -> tide::Result {
    let service = &request.state().service;
    let users = service.list_users().await?;
    Ok(Response::from(Body::from_json(&users)?))
}

pub async fn get_user<S: Service>(request: Request<State<S>>) -> tide::Result {
    let id: i32 = request
        .param("id")
        .map_err(|error| tide::Error::new(StatusCode::NotFound, error))?;
    let service = &request.state().service;
    let user = service.get_user(id).await?;
    Ok(match user {
        Some(user) => Response::from(Body::from_json(&user)?),
        None => Response::new(StatusCode::NotFound),
    })
}
