use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::{restaurant_ratings, restaurants};

#[derive(Debug, Clone, Queryable, Deserialize, Serialize)]
pub struct Restaurant {
    pub restaurant_id: uuid::Uuid,
    pub create_user_id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub created_date: chrono::NaiveDateTime,
    pub updated_date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[diesel(table_name = restaurants)]
pub struct AddRestaurant {
    pub create_user_id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub url: String,
}

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct RestaurantRating {
    pub restaurant_rating_id: uuid::Uuid,
    pub restaurant_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rating_type: String,
    pub rating: i32,
    pub created_date: chrono::NaiveDateTime,
    pub updated_date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[diesel(table_name = restaurant_ratings)]
pub struct AddRestaurantRating {
    pub restaurant_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rating_type: String,
    pub rating: i32,
}
