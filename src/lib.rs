//! lib.rs

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

async fn health_check(_req: HttpRequest) -> impl Responder {
    // use `HttpResponse::Ok` to get a `HttpResponseBuilder`
    // primed with a 200 OK status code
    // `HttpResponseBuilder` implements `Responder`
    // we can therefore omit our call to `finish`
    // HttpResponse::Ok().finish()
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/*
`HttpServer` handles all *transport level* concerns

`App` is the component whose job is to take an incoming request as input
  and spit out a response

`route` takes two parameters:
  * path, a string, possibly templated to accomodate dynamic path segments
  * route, an instance of `Route` struct

`web::get()` is a short-cut for `Route::new().guard(guard::Get())
*/
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
