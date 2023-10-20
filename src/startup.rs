use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use actix_web::middleware::Logger;

use crate::routes::{health_check, subscribe};

/*
`HttpServer` handles all *transport level* concerns

`App` is the component whose job is to take an incoming request as input
  and spit out a response

`route` takes two parameters:
  * path, a string, possibly templated to accomodate dynamic path segments
  * route, an instance of `Route` struct

`web::get()` is a short-cut for `Route::new().guard(guard::Get())
*/
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
