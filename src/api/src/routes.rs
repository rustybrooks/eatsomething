use serde::de::DeserializeOwned;
use warp::header::headers_cloned;
use warp::{self, Filter};

use crate::auth;
use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType};
use crate::pool::OurPool;

pub fn with_db(pool: OurPool) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any().map(move || pool.clone()).and_then(|pool: OurPool| async move {
        match pool.get() {
            Ok(conn) => Ok(DBAccessManager::new(conn)),
            Err(err) => Err(warp::reject::custom(AppError::new(
                format!("Error getting connection from pool: {}", err).as_str(),
                ErrorType::Internal,
            ))),
        }
    })
}

pub fn with_json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn with_auth() -> impl Filter<Extract = (auth::Claims,), Error = warp::Rejection> + Clone {
    headers_cloned().and_then(auth::authorize)
}

pub fn routes(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api").and(crate::user::routes(pool.clone()).or(crate::restaurant::routes(pool.clone())))
}
