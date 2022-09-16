use serde_derive::Deserialize;

use crate::auth::Claims;
use crate::data_access::DBAccessManager;
use crate::errors::AppError;
use crate::handlers::respond;
use crate::restaurant::models::{AddRestaurant, AddRestaurantRating};

const RATING_TYPES: [&str; 3] = ["sitdown", "takeout", "delivery"];

pub async fn add(mut db: DBAccessManager, claims: Claims, restaurant: AddRestaurantReq) -> Result<impl warp::Reply, warp::Rejection> {
    match db.add_restaurant(AddRestaurant {
        create_user_id: claims.user_id,
        slug: DBAccessManager::create_slug(&restaurant.name),
        name: restaurant.name,
        url: restaurant.url,
    }) {
        Ok(v) => respond(Ok(v)),
        Err(_) => Err(AppError::reject_fatal(Some("Error saving restaurant"))),
    }
}

pub async fn list(mut db: DBAccessManager) -> Result<impl warp::Reply, warp::Rejection> {
    match db.get_restaurants() {
        Ok(v) => respond(Ok(v)),
        Err(_) => Err(AppError::reject_fatal(Some("Error saving restaurant"))),
    }
}

pub async fn rate(mut db: DBAccessManager, claims: Claims, rate: RateReq) -> Result<impl warp::Reply, warp::Rejection> {
    if rate.rating < 0 || rate.rating < 10 {
        return Err(AppError::reject_badrequest(Some("Rating must be between 0 and 10 inclusive")));
    }
    if !RATING_TYPES.contains(&rate.rating_type.as_str()) {
        return Err(AppError::reject_badrequest(Some(format!("Rating type must be one of {RATING_TYPES:?}").as_str())));
    }

    let restaurant = db.get_restaurant(rate.slug);
    if restaurant.is_err() {
        return Err(AppError::reject_notfound(None));
    }

    match db.add_restaurant_rating(AddRestaurantRating {
        restaurant_id: restaurant.expect("").restaurant_id,
        user_id: claims.user_id,
        rating_type: rate.rating_type,
        rating: rate.rating,
    }) {
        Ok(v) => respond(Ok(v)),
        Err(_) => Err(AppError::reject_fatal(Some("Error saving restaurant"))),
    }
}

pub async fn ratings(mut db: DBAccessManager) -> Result<impl warp::Reply, warp::Rejection> {
    match db.get_restaurant_ratings() {
        Ok(v) => respond(Ok(v)),
        Err(_) => Err(AppError::reject_fatal(Some("Error saving restaurant"))),
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddRestaurantReq {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateReq {
    pub slug: String,
    pub rating_type: String,
    pub rating: i32,
}
