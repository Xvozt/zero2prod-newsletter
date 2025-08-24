use actix_web::{App, HttpServer};
use actix_web::dev;
use std::net::TcpListener;
use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener) -> Result<dev::Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
    })
        .listen(listener)?
        .run();
    Ok(server)
}