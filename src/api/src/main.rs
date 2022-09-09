use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let url = env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(url);
    let _pool = Pool::builder()
        .max_size(5)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([0, 0, 0, 0], 5000)).await;
}
