use zero2prod_newsletter::configuration::get_config;
use zero2prod_newsletter::startup::Application;
use zero2prod_newsletter::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_config().expect("Failed to read configuration");
    // let server = build(configuration).await?;
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
