//! main.rs

use zero2prod::run;

/*
`main` is the entry point of the program, **cannot** be `async`
`tokio::main` is a macro that sets up a tokio runtime
use `cargo expand --bin zero2prod` to see the expanded code
*/
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run().await
}
