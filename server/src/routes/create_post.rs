use actix_web::{web, HttpResponse, Responder};
use sqlx::{query, types::chrono::Utc, PgPool};

#[derive(serde::Deserialize)]
pub struct PostData {
    title: String,
    user: String,
    body: String,
    img_uri: String,
}

pub async fn create_post(post: web::Form<PostData>, pool: web::Data<PgPool>) -> impl Responder {
    let body = match query!(
        "INSERT INTO body (body, image_uri) VALUES ($1, $2) RETURNING ID",
        &post.body,
        &post.img_uri
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Err(_) => return HttpResponse::InternalServerError(),
        Ok(v) => v,
    };
    println!("body id {}", body.id);
    println!("body title {}", post.title);
    match query!(
        "INSERT INTO posts (title, body_id, created_at) VALUES ($1, $2, $3)",
        &post.title,
        body.id,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Err(_) => HttpResponse::InternalServerError(),
        Ok(_) => HttpResponse::Created(),
    }
}
