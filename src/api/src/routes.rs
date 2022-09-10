use crate::errors::{AppError, ErrorType};
use warp::{self, Filter};

use crate::handlers;
use crate::pool::{OurConn, OurPool};

fn with_db(pool: OurPool) -> impl Filter<Extract = (OurConn,), Error = warp::Rejection> + Clone {
    // warp::any().map(move || pool.clone())
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: OurPool| async move {
            match pool.get() {
                Ok(conn) => Ok(conn),
                // Err(_) => Err("")
                Err(err) => Err(warp::reject::custom(AppError::new(
                    format!("Error getting connection from pool: {}", err).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}

fn user_signup(
    pool: OurPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user")
        .and(warp::post())
        .and(with_db(pool))
        .and_then(handlers::user_signup)
}

pub fn routes(
    pool: OurPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    user_signup(pool)
}
