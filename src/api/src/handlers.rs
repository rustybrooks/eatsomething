use serde::Serialize;

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

use crate::data_access::DBAccessManager;
use crate::errors::AppError;
use crate::models::CreateUser;
use rand::Rng;

fn respond<T: Serialize>(
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

/*
 async signup({ username, email, password, password2 }: { username: string; email: string; password: string; password2: string }) {
   const errors: { [id: string]: string } = {};
   if (username.length < 4) {
     errors.username = 'Username must be at least 4 characters';
   }

   const re = /^[a-z,A-Z,0-9,\-,_]+$/;
   if (!username.match(re)) {
     errors.username = 'Username must be composed of only letters, numbers, _ and -';
   }

   if (password !== password2) {
     errors.password2 = 'Passwords do not match';
   }

   if (password.length < 8) {
     errors.password = 'Password must be at least 8 characters';
   }

   if (!email) {
     errors.email = 'Email required';
   }

   if (Object.keys(errors).length) {
     throw new HttpBadRequest(errors);
   }

   try {
     await queries.addUser({ username, password, email });
     return queries.generateToken(username);
   } catch (e) {
     throw new HttpBadRequest({ username: 'Failed to create user' });
   }
 }
*/

async fn encrypt_password() -> Result<String, pbkdf2::password_hash::Error> {
    let password = b"hunter42"; // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)
    Ok(Pbkdf2.hash_password(password, &salt)?.to_string())

    // // Verify password against PHC string
    // assert!(Pbkdf2.verify_password(password, &parsed_hash).is_ok());
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
    user: CreateUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut rng = rand::thread_rng();
    let api_key: i128 = rng.gen();
    let created_user = db.create_user(CreateUser {
        api_key: Some(format!("{api_key:x}")),
        is_admin: Some(false),
        ..user
    });

    respond(created_user, warp::http::StatusCode::OK)
}
