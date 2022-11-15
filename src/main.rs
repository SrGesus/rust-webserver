use warp;

use rust_webserver::routes;
use rust_webserver::db;
use rust_webserver::cfg::SERVER_ADDRESS;


#[tokio::main]
async fn main() {
    // an HashMap to simulate a database
    // it is necessary to use Arc smart pointers and Mutex for asynchronicity
    let db = db::init_db();

    let routes = routes::get_routes(db);
    
    warp::serve(routes)
        .run(SERVER_ADDRESS).await;
}
