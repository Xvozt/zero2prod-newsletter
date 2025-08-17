use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev;
use std::net::TcpListener;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<dev::Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(health_check)
    })
        .listen(listener)?
        .run();
    Ok(server)
}