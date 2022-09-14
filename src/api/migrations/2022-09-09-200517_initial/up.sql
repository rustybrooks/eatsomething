CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION trigger_set_timestamp() RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_date = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE users (
    user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    password varchar(200) NOT NULL,
    email varchar(200) NOT NULL,
    username varchar(50) NOT NULL,
    is_admin bool DEFAULT FALSE NOT NULL,
    api_key char(64) NOT NULL,
    created_date timestamp NOT NULL DEFAULT NOW(),
    updated_date timestamp NOT NULL DEFAULT NOW()
);
CREATE TRIGGER update_user_updated_date
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
CREATE UNIQUE INDEX users_username ON users(LOWER(username));
CREATE UNIQUE INDEX users_email ON users(LOWER(email));
CREATE UNIQUE INDEX users_api_key ON users(LOWER(api_key));

create table friends(
    friend_id uuid primary key default gen_random_uuid(),
    user_id_from uuid not null references users(user_id),
    user_id_to uuid not null REFERENCES users(user_id),
    created_date timestamp NOT NULL DEFAULT NOW(),
    updated_date timestamp NOT NULL DEFAULT NOW()
);
CREATE TRIGGER update_friends_updated_date
    BEFORE UPDATE
    ON friends
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
create index friends_user_id_from on friends(user_id_from);
create index friends_user_id_to on friends(user_id_to);


CREATE TABLE restaurants (
    restaurant_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    create_user_id uuid NOT NULL REFERENCES users(user_id),
    name varchar(200),
    url text,
    created_date timestamp NOT NULL DEFAULT NOW(),
    updated_date timestamp NOT NULL DEFAULT NOW()
);
CREATE TRIGGER update_restaurant_updated_date
    BEFORE UPDATE
    ON restaurants
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
CREATE INDEX restaurants_create_user_id ON restaurants(create_user_id);

CREATE TYPE restaurant_rating_type AS enum ('delivery', 'pickup', 'dine-in');
CREATE TABLE restaurant_ratings (
    restaurant_ratings_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    restaurant_id uuid NOT NULL REFERENCES restaurants(restaurant_id),
    user_id uuid NOT NULL REFERENCES users(user_id),
    rating_type restaurant_rating_type NOT NULL,
    created_date timestamp NOT NULL DEFAULT NOW(),
    updated_date timestamp NOT NULL DEFAULT NOW()
);
CREATE TRIGGER update_restaurant_ratingss_updated_date
    BEFORE UPDATE
    ON restaurant_ratings
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
CREATE INDEX restaurant_ratings_user_id ON restaurant_ratings(user_id);
CREATE INDEX restaurant_ratings_restaurant_id ON restaurant_ratings(restaurant_id);


CREATE TABLE recipes (
    recipe_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(user_id),
    name varchar(200),
    details text,
    created_date timestamp NOT NULL DEFAULT NOW(),
    updated_date timestamp NOT NULL DEFAULT NOW()
);
CREATE TRIGGER update_recipes_updated_date
    BEFORE UPDATE
    ON recipes
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
CREATE INDEX recipes_user_id ON recipes(user_id);

CREATE TABLE recipe_ratings (
    restaurant_ratings_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    recipe_id uuid NOT NULL REFERENCES restaurants(restaurant_id),
    user_id uuid NOT NULL REFERENCES users(user_id),
    created_date timestamp NOT NULL DEFAULT NOW(),
    updated_date timestamp NOT NULL DEFAULT NOW()
);
CREATE TRIGGER update_recipe_ratings_updated_date
    BEFORE UPDATE
    ON recipe_ratings
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
CREATE INDEX recipe_ratings_user_id ON recipe_ratings(user_id);
CREATE INDEX recipe_ratings_recipe_id ON recipe_ratings(recipe_id);
