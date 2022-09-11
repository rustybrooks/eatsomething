use std::env;

lazy_static! {
    pub static ref ENV: Env = Env::new();
}

pub struct Env {
    pub jwt_secret: String,
    pub database_url: String,
}

impl Env {
    pub fn new() -> Env {
        Env { jwt_secret: env::var("JWT_SECRET").unwrap(), database_url: env::var("DATABASE_URL").unwrap() }
    }
}
