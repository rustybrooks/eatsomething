use chrono::prelude::*;
use jsonwebtoken::{decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_derive::{Deserialize, Serialize};
use warp::http::{HeaderMap, HeaderValue};

use crate::env::ENV;
use crate::errors::AppError;

const BEARER: &str = "Bearer ";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub username: String,
    pub user_id: uuid::Uuid,
    pub exp: usize,
}

pub fn gen_login_token(username: String, user_id: uuid::Uuid) -> errors::Result<String> {
    let expiration = Utc::now().checked_add_signed(chrono::Duration::days(14)).expect("valid timestamp").timestamp();

    let claims = Claims { username, user_id, exp: expiration as usize };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(ENV.jwt_secret.as_bytes()))
}

pub fn validate_token(jwt: String) -> errors::Result<Claims> {
    let decoded = decode::<Claims>(&jwt, &DecodingKey::from_secret(ENV.jwt_secret.as_bytes()), &Validation::new(Algorithm::HS512))?;

    Ok(decoded.claims)
}

pub fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, AppError> {
    let header = match headers.get(warp::http::header::AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AppError::err_forbidden(None)),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AppError::err_forbidden(None)),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(AppError::err_forbidden(None));
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

pub async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<Claims, warp::Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => match validate_token(jwt) {
            Ok(v) => Ok(v),
            Err(_) => Err(AppError::reject_forbidden(None)),
        },
        Err(_) => Err(AppError::reject_forbidden(None)),
    }
}
