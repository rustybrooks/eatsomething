use crate::auth;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};

use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType, FlexError};
use crate::models::CreateUser;

fn respond<T: serde::Serialize>(result: Result<T, AppError>) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK)),
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}

fn encrypt_password(password: String) -> Result<String, pbkdf2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Pbkdf2.hash_password(&password.into_bytes(), &salt)?.to_string())
}

fn check_password(password: String, password_hash: String) -> Result<bool, pbkdf2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    Ok(Pbkdf2.verify_password(&password.into_bytes(), &parsed_hash).is_ok())
}

pub async fn signup(mut db: DBAccessManager, signup: UserSignupReq) -> Result<impl warp::Reply, warp::Rejection> {
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

    let password = encrypt_password(signup.password);
    if password.is_err() {
        return Err(warp::reject::custom(AppError::new("blorp", ErrorType::Internal)));
    }

    let user = CreateUser {
        username: signup.username,
        password: password.unwrap(),
        email: signup.email,
        api_key: Some(format!("{:x}", rng.gen::<i128>())),
        is_admin: Some(false),
    };
    let created_user = db.create_user(user);

    respond(created_user)
}

pub async fn login(mut db: DBAccessManager, user_login: UserLoginReq) -> Result<impl warp::Reply, warp::Rejection> {
    log::warn!("login {user_login:?}");
    if !user_login.username.is_empty() && !user_login.password.is_empty() {
        let user = db.get_user(user_login.username);
        if let Some(suser) = user {
            let res = check_password(user_login.password, suser.password);
            if res.is_ok() && res.unwrap() {
                return respond(Ok(UserLoginResp {
                    status: "ok".to_string(),
                    token: crate::auth::gen_login_token(suser.username).expect("invalid"),
                }));
            }
        }
    }

    Err(AppError::reject_forbidden(None))
}

pub async fn auth_test(claims: auth::Claims) -> Result<impl warp::Reply, warp::Rejection> {
    respond(Ok(claims))
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLoginResp {
    pub status: String,
    pub token: String,
}
