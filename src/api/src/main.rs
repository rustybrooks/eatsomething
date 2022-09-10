#![allow(dead_code)]

mod data_access;
mod errors;
mod handlers;
mod models;
mod pool;
mod routes;
mod schema;

#[tokio::main]
async fn main() {
    warp::serve(routes::routes(pool::init_pool().await))
        .run(([0, 0, 0, 0], 5000))
        .await;
}
