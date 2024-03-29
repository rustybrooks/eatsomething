use core::clone::Clone;

use warp::{self, Filter};

use crate::pool::OurPool;
use crate::routes;
use crate::routes::{with_auth, with_db, with_json_body};
use crate::user::handlers::{AddFriendReq, UserLoginReq, UserSignupReq};

mod handlers;
pub mod models;

pub fn routes(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user").and(
        signup(pool.clone())
            .or(login(pool.clone()))
            .or(me(pool.clone()))
            .or(add_friend(pool.clone()))
            .or(friends(pool.clone())),
    )
}

pub fn signup(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("signup")
        .and(warp::post())
        .and(routes::with_db(pool))
        .and(routes::with_json_body::<UserSignupReq>())
        .and_then(handlers::signup)
}

pub fn login(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(with_db(pool))
        .and(with_json_body::<UserLoginReq>())
        .and_then(handlers::login)
}

pub fn me(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("me").and(warp::get()).and(with_db(pool)).and(with_auth()).and_then(handlers::me)
}

pub fn add_friend(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("add_friend")
        .and(warp::get())
        .and(with_db(pool))
        .and(with_auth())
        .and(warp::query::<AddFriendReq>())
        .and_then(handlers::add_friend)
}

pub fn friends(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("friends")
        .and(warp::get())
        .and(with_db(pool))
        .and(with_auth())
        .and_then(handlers::friends)
}
