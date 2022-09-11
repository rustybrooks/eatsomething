use core::clone::Clone;

use warp::{self, Filter};

use crate::pool::OurPool;

pub fn routes(_pool: OurPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("restaurant").map(|| "Hello".to_string())
}
