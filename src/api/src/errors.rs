use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;

use serde_derive::Serialize;
use warp::reject::Reject;
use warp::{self};
use warp::{Rejection, Reply};

#[derive(Debug, Clone)]
pub enum ErrorType {
    NotFound,
    Internal,
    BadRequest,
}

#[derive(Debug)]
pub struct AppError {
    pub err_type: ErrorType,
    pub message: String,
}

impl AppError {
    pub fn new(message: &str, err_type: ErrorType) -> AppError {
        AppError { message: message.to_string(), err_type }
    }

    pub fn to_http_status(&self) -> warp::http::StatusCode {
        match self.err_type {
            ErrorType::NotFound => warp::http::StatusCode::NOT_FOUND,
            ErrorType::Internal => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadRequest => warp::http::StatusCode::BAD_REQUEST,
        }
    }

    pub fn from_diesel_err(err: diesel::result::Error, context: &str) -> AppError {
        AppError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                diesel::result::Error::DatabaseError(db_err, _) => match db_err {
                    diesel::result::DatabaseErrorKind::UniqueViolation => ErrorType::BadRequest,
                    _ => ErrorType::Internal,
                },
                diesel::result::Error::NotFound => ErrorType::NotFound,
                // Here we can handle other cases if needed
                _ => ErrorType::Internal,
            },
        )
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Reject for AppError {}

#[derive(Serialize)]
struct ErrorMessage<'a> {
    status: &'a str,
    data: String,
}

#[derive(Serialize)]
struct FlexErrorMessage<'a> {
    status: &'a str,
    data: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct FlexError {
    pub err_type: ErrorType,
    pub data: HashMap<String, Vec<String>>,
}

impl FlexError {
    pub fn new(err_type: ErrorType) -> FlexError {
        let errv: HashMap<String, Vec<String>> = HashMap::new();
        FlexError { data: errv, err_type }
    }

    pub fn add(&mut self, key: &str, error: String) -> () {
        let val = self.data.get_mut(key);
        match val {
            Some(v) => v.push(error),
            None => {
                let v: Vec<String> = vec![error];
                self.data.insert(key.to_string(), v);
            }
        };
    }

    pub fn to_http_status(&self) -> warp::http::StatusCode {
        match self.err_type {
            ErrorType::NotFound => warp::http::StatusCode::NOT_FOUND,
            ErrorType::Internal => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadRequest => warp::http::StatusCode::BAD_REQUEST,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
}

impl Reject for FlexError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(app_err) = err.find::<AppError>() {
        code = app_err.to_http_status();
        message = app_err.message.as_str();
    } else if let Some(app_err) = err.find::<FlexError>() {
        code = app_err.to_http_status();
        let json = warp::reply::json(&FlexErrorMessage { status: "failed", data: app_err.data.clone() });

        return Ok(warp::reply::with_status(json, code));
    } else if err.find::<warp::filters::body::BodyDeserializeError>().is_some() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        // In case we missed something - log and respond with 500
        eprintln!("unhandled rejection: {:?}", err);
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "Unhandled rejection";
    }

    let json = warp::reply::json(&ErrorMessage { status: "failed", data: message.into() });

    Ok(warp::reply::with_status(json, code))
}
