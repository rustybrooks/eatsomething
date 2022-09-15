use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};

use crate::auth;
use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType, FlexError};
use crate::user::models::{AddUser,AddFriend};

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

    let user = AddUser {
        username: signup.username.clone(),
        password: password.unwrap(),
        email: signup.email,
        api_key: Some(format!("{:x}", rng.gen::<i128>())),
        is_admin: Some(false),
    };
    let created_user = db.add_user(user);

    // FIXME automate this more
    match created_user {
        Ok(_) => respond(Ok(UserSignupResp {
          status: "ok".to_string(),
          token: auth::gen_login_token(signup.username).expect("invalid"),
        })),
        Err(_) => Err(warp::reject::custom(AppError::new("blorp", ErrorType::Internal)))
    }
}

pub async fn login(mut db: DBAccessManager, user_login: UserLoginReq) -> Result<impl warp::Reply, warp::Rejection> {
    if !user_login.username.is_empty() && !user_login.password.is_empty() {
        let user = db.get_user(user_login.username);
        if let Some(suser) = user {
            let res = check_password(user_login.password, suser.password);
            if res.is_ok() && res.unwrap() {
                return respond(Ok(UserLoginResp {
                    status: "ok".to_string(),
                    token: auth::gen_login_token(suser.username).expect("invalid"),
                }));
            }
        }
    }

    Err(AppError::reject_forbidden(None))
}

pub async fn me(mut db: DBAccessManager, claims: auth::Claims) -> Result<impl warp::Reply, warp::Rejection> {
    let user = db.get_user(claims.username);
    match user {
        Some(v) => respond(Ok(v)),
        None => Err(AppError::reject_notfound(None)),
    }
}

pub async fn add_friend(mut db: DBAccessManager, add_friend: AddFriend) -> Result<impl warp::Reply, warp::Rejection> {
    let friend = db.add_friend(add_friend);
    match friend {
        Ok(v) => respond(Ok(v)),
        Err(_) => Err(warp::reject::custom(AppError::new("blorp", ErrorType::Internal)))
    }
}

pub async fn friends(mut db: DBAccessManager, claims: auth::Claims) -> Result<impl warp::Reply, warp::Rejection> {
    let friends = db.get_friends(claims.username);
    respond(Ok("hi"))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSignupReq {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password2: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSignupResp {
    pub status: String,
    pub token: String,
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
