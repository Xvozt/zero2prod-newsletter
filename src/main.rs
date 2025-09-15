use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod_newsletter::configuration::get_config;
use zero2prod_newsletter::email_client::EmailClient;
use zero2prod_newsletter::startup::run;
use zero2prod_newsletter::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_config().expect("Failed to read configuration");
    // Добавьте эту строку для отладки
    tracing::info!("Database URL: {}:{}/{}", configuration.database.host, configuration.database.port, configuration.database.db_name);


    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address");
    // let base_url =
    let email_client = EmailClient::new(configuration.email_client.base_url, sender_email, configuration.email_client.auth_token);
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}

