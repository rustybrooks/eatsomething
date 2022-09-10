use serde::de::DeserializeOwned;
use warp::{self, Filter};

use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType};
use crate::handlers;
use crate::models::CreateUser;
use crate::pool::OurPool;

fn with_db(
    pool: OurPool,
) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    // warp::any().map(move || pool.clone())
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: OurPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DBAccessManager::new(conn)),
                // Err(_) => Err("")
                Err(err) => Err(warp::reject::custom(AppError::new(
                    format!("Error getting connection from pool: {}", err).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn user_signup(
    pool: OurPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user")
        .and(warp::post())
        .and(with_db(pool))
        .and(with_json_body::<CreateUser>()) // Try to deserialize JSON body to AddBook
        .and_then(handlers::user_signup)
}

pub fn routes(
    pool: OurPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    user_signup(pool)
}
