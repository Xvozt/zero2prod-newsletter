use std::net::TcpListener;
use sqlx::PgPool;
use env_logger::Env;
use zero2prod_newsletter::startup::run;
use zero2prod_newsletter::configuration::get_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_config().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to a Postgres database");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}