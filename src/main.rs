extern crate rust_tide_test;

use async_std::sync::Arc;
use rust_tide_test::{
    api::Api, database::DatabaseService, mock::MockService, service::Service,
};
use std::env;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let database_url = env::var("DATABASE_URL").map(Some).unwrap_or(None);
    let service: Arc<dyn Service> = match database_url {
        Some(url) => Arc::new(DatabaseService::new(url)),
        None => Arc::new(MockService::new()),
    };
    let api = Api::new(service);
    api.listen().await?;
    Ok(())
}
