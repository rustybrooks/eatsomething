use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct User {
    user_id: uuid::Uuid,
    password: String,
    email: String,
    username: String,
    is_admin: bool,
    api_key: String,
    created_date: chrono::NaiveDateTime,
    updated_date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub password: String,
    pub email: String,
    pub username: String,
    pub is_admin: Option<bool>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Queryable, Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}
