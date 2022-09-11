#![allow(dead_code)]

extern crate log;

#[macro_use]
extern crate lazy_static;

use warp::Filter;

mod auth;
mod data_access;
mod env;
mod errors;
mod pool;
mod routes;
mod schema;
mod user;
mod restaurant;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting up");

    let routes = routes::routes(pool::init_pool().await).recover(errors::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}
