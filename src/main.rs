use std::net::TcpListener;
use zero2prod_newsletter::startup::run;
use zero2prod_newsletter::configuration::read_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = read_config().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}