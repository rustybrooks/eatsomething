use core::clone::Clone;

use warp::{self, Filter};

use crate::pool::OurPool;
use crate::routes;

mod handlers;
mod models;

pub fn routes(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("restaurant").and(add(pool.clone()).or(list(pool.clone())))
}

pub fn add(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("add").and(warp::post()).and(routes::with_db(pool)).and_then(handlers::add)
}

pub fn list(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("list").and(warp::post()).and(routes::with_db(pool)).and_then(handlers::list)
}

pub fn rate(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("rate").and(warp::post()).and(routes::with_db(pool)).and_then(handlers::list)
}

pub fn ratings(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ratings").and(warp::post()).and(routes::with_db(pool)).and_then(handlers::list)
}
