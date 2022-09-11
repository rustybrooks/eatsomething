use serde::de::DeserializeOwned;
use warp::header::headers_cloned;
use warp::http::{HeaderMap, HeaderValue};
use warp::{self, Filter};

use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType};
use crate::pool::OurPool;
use crate::user_handlers::{UserLoginReq, UserSignupReq};
use crate::{auth, user_handlers};

fn with_db(pool: OurPool) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
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

fn with_json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn with_auth() -> impl Filter<Extract = (auth::Claims,), Error = warp::Rejection> + Clone {
    headers_cloned().and_then(authorize)
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<auth::Claims, warp::Rejection> {
    match auth::jwt_from_header(&headers) {
        Ok(jwt) => match crate::auth::validate_token(jwt) {
            Ok(v) => Ok(v),
            Err(_) => Err(AppError::reject_forbidden(None)),
        },
        Err(_) => Err(AppError::reject_forbidden(None)),
    }
}

fn user_signup(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("signup")
        .and(warp::post())
        .and(with_db(pool))
        .and(with_json_body::<UserSignupReq>())
        .and_then(user_handlers::signup)
}

fn user_login(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(with_db(pool))
        .and(with_json_body::<UserLoginReq>())
        .and_then(user_handlers::login)
}

fn user_auth_test() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("auth_test")
        // .and(warp::post())
        .and(with_auth())
        .and_then(user_handlers::auth_test)
}

pub fn user_routes(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user").and(user_signup(pool.clone()).or(user_login(pool.clone())).or(user_auth_test()))
}

pub fn routes(pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api").and(user_routes(pool.clone()).or(user_routes(pool.clone())))
}
