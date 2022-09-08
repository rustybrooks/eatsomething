use warp::Filter;

mod db;

#[tokio::main]
async fn main() {
    let _sql = db::Sql::new().await;

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([0, 0, 0, 0], 5000)).await;
}
