extern crate rust_tide_test;

use rust_tide_test::{api::Api, database::DatabaseService, mock::MockService};
use std::env;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tide::log::start();
    let service = MockService::new(); //DatabaseService::new(database_url);
    let api = Api::new(service);
    api.listen().await?;
    Ok(())
}
