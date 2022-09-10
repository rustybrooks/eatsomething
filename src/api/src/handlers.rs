use serde_derive::{Deserialize, Serialize};

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType};
use crate::models::CreateUser;
use rand::Rng;

fn respond<T: serde::Serialize>(
    result: Result<T, AppError>,
    status: warp::http::StatusCode,
) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        )),
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

async fn check_password(
    password: String,
    password_hash: String,
) -> Result<bool, pbkdf2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    Ok(Pbkdf2
        .verify_password(&password.into_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn user_signup(
    mut db: DBAccessManager,
    signup: UserSignupReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut rng = rand::thread_rng();

    if signup.username.len() <= 4 {
        return Err(warp::reject::custom(AppError::new(
            "Username must be at least 4 characters",
            ErrorType::BadRequest,
        )));
    }

    if signup.password != signup.password2 {
        return Err(warp::reject::custom(AppError::new(
            "Passwords do not match",
            ErrorType::BadRequest,
        )));
    }

    if signup.password.len() < 8 {
        return Err(warp::reject::custom(AppError::new(
            "Password must be at least 8 characters",
            ErrorType::BadRequest,
        )));
    }

    if signup.email.is_empty() {
        return Err(warp::reject::custom(AppError::new(
            "Email required",
            ErrorType::BadRequest,
        )));
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

pub async fn user_login(
    mut _db: DBAccessManager,
    user: UserLoginReq,
) -> Result<impl warp::Reply, warp::Rejection> {
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
