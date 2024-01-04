use crate::constants::APPLICATION_JSON;
use actix_web::web::Path;
use actix_web::{delete, get, post, HttpResponse};

#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(path: Path<(String,)>) -> HttpResponse {
    // get tweets
    let likes = 100;
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn like_tweet(path: Path<(String,)>) -> HttpResponse {
    // get tweets
    let like = "ok";
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like(path: Path<(String,)>) -> HttpResponse {
    // get tweets
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
