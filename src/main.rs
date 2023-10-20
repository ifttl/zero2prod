//! main.rs

use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

/*
`main` is the entry point of the program, **cannot** be `async`
`tokio::main` is a macro that sets up a tokio runtime
use `cargo expand --bin zero2prod` to see the expanded code
*/
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", &address));
    run(listener, connection_pool)?.await
}
