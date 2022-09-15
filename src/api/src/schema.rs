// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "restaurant_rating_type"))]
    pub struct RestaurantRatingType;
}

diesel::table! {
    friends (friend_id) {
        friend_id -> Uuid,
        user_id_from -> Uuid,
        user_id_to -> Uuid,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    recipe_ratings (restaurant_ratings_id) {
        restaurant_ratings_id -> Uuid,
        recipe_id -> Uuid,
        user_id -> Uuid,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    recipes (recipe_id) {
        recipe_id -> Uuid,
        user_id -> Uuid,
        name -> Nullable<Varchar>,
        details -> Nullable<Text>,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RestaurantRatingType;

    restaurant_ratings (restaurant_ratings_id) {
        restaurant_ratings_id -> Uuid,
        restaurant_id -> Uuid,
        user_id -> Uuid,
        rating_type -> RestaurantRatingType,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    restaurants (restaurant_id) {
        restaurant_id -> Uuid,
        create_user_id -> Uuid,
        name -> Nullable<Varchar>,
        url -> Nullable<Text>,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        password -> Varchar,
        email -> Varchar,
        username -> Varchar,
        is_admin -> Bool,
        api_key -> Bpchar,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::joinable!(recipe_ratings -> restaurants (recipe_id));
diesel::joinable!(recipe_ratings -> users (user_id));
diesel::joinable!(recipes -> users (user_id));
diesel::joinable!(restaurant_ratings -> restaurants (restaurant_id));
diesel::joinable!(restaurant_ratings -> users (user_id));
diesel::joinable!(restaurants -> users (create_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    friends,
    recipe_ratings,
    recipes,
    restaurant_ratings,
    restaurants,
    users,
);
