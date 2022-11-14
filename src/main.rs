use warp::{self, Filter};

// Constants
//      localhost (127.0.0.1)
//      port 5000
const SERVER_ADDRESS: ([u8; 4], u16) = ([127, 0, 0, 1], 5000);

#[tokio::main]
async fn main() {
    let routes = 
        warp::path("get_data")
            .map(|| "Here's data mf")
        .or(warp::path("put_data")
            .map(|| "shudup"));
    
    warp::serve(routes)
        .run(SERVER_ADDRESS).await;
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