use warp;

mod routes;
mod db;
mod cfg;
mod handler;


#[tokio::main]
async fn main() {
    // an HashMap to simulate a database
    // it is necessary to use Arc smart pointers and Mutex for asynchronicity
    println!("Initializing db...");
    let db = db::init_db();

    println!("Getting routes...");
    let routes = routes::get_routes(db);
    
    println!("Starting server...");
    warp::serve(routes)
        .run(cfg::SERVER_ADDRESS).await;
}
