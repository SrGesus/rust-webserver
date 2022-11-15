use warp::multipart::FormData;
use warp::{self, Filter};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;

// Constants
//      localhost (127.0.0.1)
//      port 5000
const SERVER_ADDRESS: ([u8; 4], u16) = ([127, 0, 0, 1], 5000);

type DataPoint = HashMap<String, String>;
//type Db = Arc<Mutex<Vec<DataPoint>>>;
type Db = Arc<Mutex<HashMap<String, Vec<String>>>>;

fn init_db() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}

#[tokio::main]
async fn main() {
    // um HashMap para simular 
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
            .and(with_db(db.clone()))
            .and_then(handler_put_data_post))
        
        // .map(|_| format!("KAWABUNGA"))
        //.and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        //.and_then(|a: String| format!("Here's data mf: {}", a))
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

    //println!("{:?}", db);
    Ok(result)
}

async fn handler_put_data_post(
    values: FormData,
    db: Db
) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("{:?}", values))
}

fn with_db(
    db: Db
) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}



// fn get_data_post() {

// }

// /put_data
fn get_data(
    db: Db
)
-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("get_data")
        .map(|| "shudup")
}


/*// Constants
const SERVER_ADDRESS: &str = "127.0.0.1:1234";

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

    println!("Server listening!");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established with");
        
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[0..len]);
    print!("Received: {}", message);
}*/