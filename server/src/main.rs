use std::net::TcpListener;

use agora::{config, startup};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = config::get_config().expect("Failed to read configuration!");
    let address = config.db.connection_string();
    let connection_pool = PgPool::connect(&address)
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address);
    startup::run(listener?, connection_pool)?.await
}
