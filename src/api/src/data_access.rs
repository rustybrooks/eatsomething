use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::errors::AppError;
use crate::user::models::{AddUser, User, Friend, AddFriend};
use crate::schema::*;

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

//joinable!(friends -> users (user_id));

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn add_user(&mut self, dto: AddUser) -> Result<User, AppError> {
        // use super::schema::books;

        diesel::insert_into(users::table)
            .values(&dto)
            .get_result(&mut self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while creating user"))
    }

    pub fn get_user(&mut self, uname: String) -> Option<User> {
        match users::table.filter(users::dsl::username.eq(&uname).or(users::dsl::email.eq(&uname))).limit(1).first::<User>(&mut self.connection) {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    }

    pub fn add_friend(&mut self, dto: AddFriend) -> Result<Friend, AppError> {
        diesel::insert_into(friends::table)
            .values(&dto)
            .get_result(&mut self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while creating friend"))
    }

    pub fn get_friends(&mut self, uname: String) -> () {
        users::table
            .left_join(friends::table.on(users::user_id.eq(friends::user_id_from)))
            .filter(users::dsl::username.eq(uname))
            .into_boxed()
            .get_results(&mut self.connection)
    }
}
