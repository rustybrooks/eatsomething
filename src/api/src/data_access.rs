use crate::errors::{AppError, ErrorType};
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
            .map_err(|err| AppError::from_diesel_err(err, "while creating book"))
        // if error occurred map it to AppError
    }
}
