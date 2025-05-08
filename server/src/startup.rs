use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::*;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/post", web::post().to(create_post))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
