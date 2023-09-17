use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}
