//! src/main.rs
//pub mod startup;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Renamed!
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // we have removed the hard-coded address 8000, it is now read from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    println!("running on http://127.0.0.1:{}", address);
    run(listener, connection_pool)?.await
}
