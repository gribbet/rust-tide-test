extern crate rust_tide_test;

use rust_tide_test::{api::Api, database::DatabaseService, mock::MockService};
use std::env;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let database_url = env::var("DATABASE_URL").map(Some).unwrap_or(None);
    match database_url {
        Some(url) => Api::new(DatabaseService::new(url)).listen().await?,
        None => Api::new(MockService::new()).listen().await?,
    }
    Ok(())
}
