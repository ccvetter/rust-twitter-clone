#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};

mod db;
mod constants;
mod tweets;
mod likes;
mod error_handler;
mod schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenvy::dotenv().ok();
    db::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(tweets::init_routes)
            .configure(likes::init_routes)
    })
    .bind("localhost:9090")?
    .run()
    .await
}

