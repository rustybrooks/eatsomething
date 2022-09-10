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
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let _hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let pool = pool::init_pool().await;
    let all_routes = routes::routes(pool);
    warp::serve(all_routes).run(([0, 0, 0, 0], 5000)).await;
}
