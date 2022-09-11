use std::collections::HashMap;

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};

use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType, FlexError};
use crate::models::CreateUser;

fn respond<T: serde::Serialize>(result: Result<T, AppError>, status: warp::http::StatusCode) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(warp::reply::json(&response), status)),
        Err(err) => {
            // log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}

async fn encrypt_password() -> Result<String, pbkdf2::password_hash::Error> {
    let password = b"hunter42"; // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);
    Ok(Pbkdf2.hash_password(password, &salt)?.to_string())
}

async fn check_password(password: String, password_hash: String) -> Result<bool, pbkdf2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    Ok(Pbkdf2.verify_password(&password.into_bytes(), &parsed_hash).is_ok())
}

pub async fn user_signup(mut db: DBAccessManager, signup: UserSignupReq) -> Result<impl warp::Reply, warp::Rejection> {
    let mut rng = rand::thread_rng();
    let mut error: FlexError = FlexError::new(ErrorType::BadRequest);

    if signup.username.len() <= 4 {
        error.add("username", "Username must be at least 4 characters".to_string());
    }

    if signup.password != signup.password2 {
        error.add("password", "Passwords do not match".to_string());
    }

    if signup.password.len() < 8 {
        error.add("password", "Password must be at least 8 characters".to_string());
    }

    if signup.email.is_empty() {
        error.add("email", "Email required".to_string());
    }

    if !error.is_empty() {
        return Err(warp::reject::custom(error));
    }

    let user = CreateUser {
        username: signup.username,
        password: signup.password,
        email: signup.email,
        api_key: Some(format!("{:x}", rng.gen::<i128>())),
        is_admin: Some(false),
    };
    let created_user = db.create_user(user);

    respond(created_user, warp::http::StatusCode::OK)
}

pub async fn user_login(mut _db: DBAccessManager, user: UserLoginReq) -> Result<impl warp::Reply, warp::Rejection> {
    respond(Ok(user), warp::http::StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSignupReq {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password2: String,
}

pub struct UserSignupErrResp {
    username: Vec<String>,
    email: Vec<String>,
    password: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLoginReq {
    pub username: String,
    pub password: String,
}
