#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate log;

use warp::http::Method;
use warp::Filter;

mod auth;
mod data_access;
mod env;
mod errors;
mod pool;
mod restaurant;
mod routes;
mod schema;
mod user;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting up");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(&[Method::GET, Method::POST]);
    let routes = routes::routes(pool::init_pool().await).recover(errors::handle_rejection).with(cors);
    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}
