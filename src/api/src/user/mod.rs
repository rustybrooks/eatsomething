use crate::pool::OurPool;
use core::clone::Clone;
use warp::{self, Filter};

mod handlers;
pub mod models;
mod routes;

pub fn routes(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user").and(
        routes::signup(pool.clone())
            .or(routes::login(pool.clone()))
            .or(routes::me(pool.clone()))
            .or(routes::add_friend(pool.clone()))
            .or(routes::friends(pool.clone())),
    )
}
