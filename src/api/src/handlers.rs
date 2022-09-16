use crate::errors::AppError;

pub fn respond<T: serde::Serialize>(result: Result<T, AppError>) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK)),
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}
