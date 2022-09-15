use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::schema::*;
use crate::user::models::{AddFriend, AddUser, Friend, User, UserFriend};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn add_user(&mut self, dto: AddUser) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table).values(&dto).get_result(&mut self.connection)
    }

    pub fn get_user(&mut self, user_id: Option<&uuid::Uuid>, username: Option<&String>) -> Result<User, diesel::result::Error> {
        let mut query = users::table.into_boxed();

        if let Some(u) = user_id {
            query = query.filter(users::dsl::user_id.eq(u));
            // users::dsl::username.eq(&"".to_string())
        }
        if let Some(u) = username {
            query = query.filter(users::dsl::username.eq(u));
        }

        query.limit(1).first(&mut self.connection)
    }

    pub fn add_friend(&mut self, dto: AddFriend) -> Result<UserFriend, diesel::result::Error> {
        let inserted: Friend = diesel::insert_into(friends::table).values(&dto).get_result(&mut self.connection)?;

        self.get_friend(inserted.friend_id)
    }

    fn create_user_friend(user_friend: &(Friend, User)) -> UserFriend {
        UserFriend {
            friend_id: user_friend.0.friend_id,
            user_id_to: user_friend.0.user_id_to,
            username_to: user_friend.1.username.clone(),
            created_date: user_friend.0.created_date,
            updated_date: user_friend.0.updated_date,
        }
    }

    pub fn get_friend(&mut self, friend_id: uuid::Uuid) -> Result<UserFriend, diesel::result::Error> {
        let friend = friends::table
            .inner_join(users::table.on(friends::dsl::user_id_to.eq(users::dsl::user_id)))
            // .inner_join(users::table.on(friends::dsl::user_id_from.eq(users::dsl::user_id)))
            .filter(friends::dsl::friend_id.eq(&friend_id))
            .limit(1)
            .first(&mut self.connection)?;

        Ok(Self::create_user_friend(&friend))
    }

    pub fn get_friends(&mut self, user_id: uuid::Uuid) -> Result<Vec<UserFriend>, diesel::result::Error> {
        let res = friends::table
            .inner_join(users::table.on(friends::dsl::user_id_to.eq(users::dsl::user_id)))
            .filter(friends::dsl::user_id_from.eq(user_id))
            .load::<(Friend, User)>(&mut self.connection)?;

        Ok(res.iter().map(Self::create_user_friend).collect())
    }
}
