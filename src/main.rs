//! main.rs

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
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", &address));
    run(listener)?.await
}
