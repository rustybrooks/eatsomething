use core::clone::Clone;

use warp::{self, Filter};

use crate::pool::OurPool;
use crate::restaurant::handlers::{AddRestaurantReq, RateReq};
use crate::routes;
use crate::routes::with_auth;

mod handlers;
pub mod models;

pub fn routes(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("restaurant").and(add(pool.clone()).or(list(pool.clone())))
}

pub fn add(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("add")
        .and(warp::post())
        .and(routes::with_db(pool))
        .and(with_auth())
        .and(routes::with_json_body::<AddRestaurantReq>())
        .and_then(handlers::add)
}

pub fn list(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("list").and(warp::post()).and(routes::with_db(pool)).and_then(handlers::list)
}

pub fn rate(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("rate")
        .and(warp::post())
        .and(routes::with_db(pool))
        .and(with_auth())
        .and(routes::with_json_body::<RateReq>())
        .and_then(handlers::list)
}

pub fn ratings(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ratings").and(warp::post()).and(routes::with_db(pool)).and_then(handlers::list)
}
