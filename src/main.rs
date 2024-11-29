//! src/main.rs
//pub mod startup;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    println!("running on http://127.0.0.1:{}", port);
    run(listener)?.await
}
