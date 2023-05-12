use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    // use `HttpResponse::Ok` to get a `HttpResponseBuilder`
    // primed with a 200 OK status code
    // `HttpResponseBuilder` implements `Responder`
    // we can therefore omit our call to `finish`
    // HttpResponse::Ok().finish()
    HttpResponse::Ok().finish()
}
