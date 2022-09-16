use crate::data_access::DBAccessManager;
use crate::handlers::respond;

pub async fn add(_db: DBAccessManager) -> Result<impl warp::Reply, warp::Rejection> {
    respond(Ok("hi"))
}

pub async fn list(_db: DBAccessManager) -> Result<impl warp::Reply, warp::Rejection> {
    respond(Ok("hi"))
}

pub async fn rate(_db: DBAccessManager) -> Result<impl warp::Reply, warp::Rejection> {
    respond(Ok("hi"))
}

pub async fn ratings(_db: DBAccessManager) -> Result<impl warp::Reply, warp::Rejection> {
    respond(Ok("hi"))
}

