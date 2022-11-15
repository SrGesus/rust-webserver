
use warp::Filter;
use std::collections::HashMap;
use std::convert::Infallible;

use crate::db::Db;
use crate::handler;

// routes wrapper
pub fn get_routes(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_data(db.clone())
        .or(put_data(db))
}

// /get_data
fn get_data(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("get_data")
        .and(with_db(db))
        .and_then(handler::get_data)
}

// /put_data
fn put_data(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    put_data_get(db.clone())
        .or(put_data_post_formdata(db.clone()))
        .or(put_data_post_urlencode(db))
}

// /put_data GET queries
fn put_data_get(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("put_data")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_db(db.clone()))
        .and_then(handler::put_data_hash)
}

// /put_data POST multipart/form-data queries
fn put_data_post_formdata(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("put_data")
        .and(warp::post())
        .and(warp::multipart::form())
        .and(with_db(db))
        .and_then(handler::put_data_formdata)
}

// /put_data POST application/x-www-form-urlencoded queries
fn put_data_post_urlencode(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("put_data")
        .and(warp::post())
        .and(warp::body::form())
        .and(with_db(db.clone()))
        .and_then(handler::put_data_hash)
}

// Adds the database as an argument when used with and() method in routes
fn with_db(
    db: Db
) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
