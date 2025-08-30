use actix_web::{App, web, HttpServer};
use actix_web::dev;
use std::net::TcpListener;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<dev::Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}