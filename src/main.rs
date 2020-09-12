extern crate rust_tide_test;

use rust_tide_test::{database::DatabaseService, web::create_app};
use std::env;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tide::log::start();
    let service = DatabaseService::new(database_url);
    let app = create_app(service);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
