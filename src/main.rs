use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::TcpListener;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_micros(2))
        .connect_lazy_with(configuration.database.with_db());

    // In Heroku there will be a PORT variable defined
    let address = if let Ok(port) = env::var("PORT") {
        format!("{}:{}", configuration.application.host, port)
    } else {
        format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
    };

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?
        .await
        .expect("Failed to run Http server");

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
