use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::{users, friends};

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct User {
    pub user_id: uuid::Uuid,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub username: String,
    pub is_admin: bool,
    pub api_key: String,
    pub created_date: chrono::NaiveDateTime,
    pub updated_date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct AddUser {
    pub password: String,
    pub email: String,
    pub username: String,
    pub is_admin: Option<bool>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Friend {
    pub friend_id: uuid::Uuid,
    pub user_id_from: uuid::Uuid,
    pub user_id_to: uuid::Uuid,
    pub created_date: chrono::NaiveDateTime,
    pub updated_date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[diesel(table_name = friends)]
pub struct AddFriend {
    pub user_id_from: uuid::Uuid,
    pub user_id_to: uuid::Uuid,
}


