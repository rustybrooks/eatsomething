use core::clone::Clone;

use warp::{self, Filter};

use crate::pool::OurPool;
use crate::routes;
use crate::routes::{with_auth, with_db, with_json_body};
use crate::user::handlers;
use crate::user::handlers::{UserLoginReq, UserSignupReq};

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

pub fn auth_test() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("auth_test").and(warp::get()).and(with_auth()).and_then(handlers::auth_test)
}

pub fn me(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("me").and(warp::get()).and(with_db(pool)).and(with_auth()).and_then(handlers::me)
}
