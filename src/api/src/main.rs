#![allow(dead_code)]

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
    let routes = routes::routes(pool::init_pool().await).recover(errors::handle_rejection);
    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}
