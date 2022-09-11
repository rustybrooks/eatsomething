#![allow(dead_code)]

#[macro_use]
extern crate log;
// extern crate pretty_env_logger;

use warp::Filter;

mod data_access;
mod errors;
mod handlers;
mod models;
mod pool;
mod routes;
mod schema;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting up");

    let routes = routes::routes(pool::init_pool().await).recover(errors::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}
