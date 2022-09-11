use crate::errors::AppError;
use crate::models::{CreateUser, User};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn create_user(&mut self, dto: CreateUser) -> Result<User, AppError> {
        // use super::schema::books;

        diesel::insert_into(crate::schema::users::table)
            .values(&dto)
            .get_result(&mut self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while creating user"))
        // if error occurred map it to AppError
    }

    pub fn get_user(&mut self, uname: String) -> Option<User> {
        use crate::schema::users::dsl::*;
        match users.filter(username.eq(&uname).or(email.eq(&uname))).limit(1).first::<User>(&mut self.connection) {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    }
}
