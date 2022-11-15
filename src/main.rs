use warp::multipart::FormData;
use warp::{self, Filter};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json;

// Constants
//      localhost (127.0.0.1)
//      port 5000
const SERVER_ADDRESS: ([u8; 4], u16) = ([127, 0, 0, 1], 5000);

type Db = Arc<Mutex<HashMap<String, Vec<String>>>>;

fn init_db() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}

#[tokio::main]
async fn main() {
    // um HashMap para simular uma base de dados
    // it is necessary to use Arc smart pointers and Mutex for asynchronicity
    let db = init_db();

    let routes = get_routes(db);
    
    warp::serve(routes)
        .run(SERVER_ADDRESS).await;
}

// routes wrapper
fn get_routes(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_data(db.clone())
        .or(put_data(db.clone()))
}

// /put_data
fn put_data(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("put_data")

    // /put_data GET queries
        .and(warp::get()
            .and(warp::query::<HashMap<String, String>>())
            .and(with_db(db.clone()))
            .and_then(handler_put_data_get))
    
    // /put_data POST queries
        .or(warp::post()
            .and(warp::multipart::form())
            .and(with_db(db))
            .and_then(handler_put_data_post))
}

// TODO
// /get_data
fn get_data(
    db: Db
)
-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("get_data")
        .and(with_db(db))
        .and_then(handler_get_data)
}
async fn handler_get_data(
    db: Db
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().await;
    
    let json_table = serde_json::to_string_pretty(&db.clone()).unwrap();

    println!("get_data GET request: {}\n", json_table);
    Ok(json_table)
}

async fn handler_put_data_get(
    values: HashMap<String, String>,
    db: Db
) -> Result<impl warp::Reply, Infallible> {
    let mut result = String::from("");
    let mut db = db.lock().await;

    for (key, value) in values.iter() {
        result.push_str(&format!("{} = {}\n", key, value));
        if let Some(vec) = db.get_mut(key) {
            vec.push(value.to_owned());
        } else {
            db.insert(key.to_owned(), vec![value.to_owned()]);
        }
    };

    println!("put_data GET request: {:?}\n", values);
    Ok(result)
}

// TODO (don't know how)
async fn handler_put_data_post(
    values: FormData,
    db: Db
) -> Result<impl warp::Reply, Infallible> {
    
    //println!("put_data POST request: {}");
    Ok(format!("{:?}", values))
}


// adds the database as an argument when used with .and method in routes
fn with_db(
    db: Db
) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
