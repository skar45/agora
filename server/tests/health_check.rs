use std::net::TcpListener;

use agora::config::{get_config, DbSettings};
use sqlx::{types::Uuid, Connection, Executor, PgConnection, PgPool};

struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn configure_db(config: &DbSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connect_sans_db_string())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database.");
    let db_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate database.");

    db_pool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let mut config = get_config().expect("Failed to read configuration.");
    config.db.db_name = Uuid::new_v4().to_string();
    let db_pool = configure_db(&config.db).await;
    let server = agora::startup::run(listener, db_pool.clone()).expect("Failed to create server.");
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp { address, db_pool }
}

#[tokio::test]
async fn health_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn post_creation_success() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "title=Test%20Title&body=Test%20Body&user=Deez&img_uri=hello.jpg";
    let response = client
        .post(&format!("{}/post", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(201, response.status().as_u16());

    let saved = sqlx::query!("SELECT title, body from posts p JOIN body b on p.body_id = b.id",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch post");
    assert_eq!(saved.title, "Test Title");
    assert_eq!(saved.body, Some("Test Body".to_owned()));
}
