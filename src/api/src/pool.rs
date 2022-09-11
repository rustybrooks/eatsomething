use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub type OurPool = Pool<ConnectionManager<PgConnection>>;
pub type OurConn = PooledConnection<ConnectionManager<PgConnection>>;

pub async fn init_pool() -> OurPool {
    let url = &crate::env::ENV.database_url;
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(5)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
