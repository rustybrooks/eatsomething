use crate::schema::users;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct User {
    user_id: uuid::Uuid,
    password: String,
    email: String,
    username: String,
    is_admin: bool,
    api_key: String,
    created_date: std::time::SystemTime,
    updated_date: std::time::SystemTime,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct CreateUser {
    password: String,
    email: String,
    username: String,
    is_admin: bool,
    api_key: String,
    created_date: std::time::SystemTime,
    updated_date: std::time::SystemTime,
}
