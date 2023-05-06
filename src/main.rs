use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/*
`main` is the entry point of the program, **cannot** be `async`
`tokio::main` is a macro that sets up a tokio runtime
use `cargo expand` to see the expanded code

`HttpServer` handles all *transport level* concerns

`App` is the component whose job is to take an incoming request as input
  and spit out a response

`route` takes two parameters:
  * path, a string, possibly templated to accomodate dynamic path segments
  * route, an instance of `Route` struct

`web::get()` is a short-cut for `Route::new().guard(guard::Get())
*/
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new( || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
