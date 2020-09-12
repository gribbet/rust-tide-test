use crate::{
    endpoint::{get_user, get_users, post_user},
    service::Service,
};
use async_std::sync::Arc;
use std::io;
use tide::Server;

#[derive(Clone)]
pub struct State {
    pub service: Arc<dyn Service>,
}

pub struct Api {
    app: Server<State>,
}

impl Api {
    pub fn new(service: Arc<dyn Service>) -> Self {
        let state = State { service: service };
        let mut app = tide::with_state(state.clone());
        app.at("/users").nest({
            let mut app = tide::with_state(state);
            app.at("/").post(post_user);
            app.at("/").get(get_users);
            app.at("/:id").get(get_user);
            app
        });
        Api { app }
    }

    pub async fn listen(self) -> io::Result<()> {
        self.app.listen("0.0.0.0:8080").await
    }
}
